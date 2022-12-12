import logging
import shelve
from datetime import datetime

# $ poetry add aiogram[fast] aiohttp[speedups]
from aiogram import Bot, Dispatcher, executor, types, filters
from aiogram.contrib.middlewares.logging import LoggingMiddleware
from aiogram.types import ParseMode, InlineKeyboardMarkup, InlineKeyboardButton, BotCommand
from aiogram.dispatcher.filters.state import State, StatesGroup
import aiogram.utils.markdown as md
from aiogram.dispatcher import FSMContext
from aiogram.contrib.fsm_storage.files import JSONStorage


# @BotFather register
API_TOKEN = ''

# Admin id
ADMIN_USERNAME = 'kolypto'
ADMIN_CHAT_ID = -596849428


# Configure logging
logging.basicConfig(level=logging.INFO)


# Shelve: our database :)
db = shelve.open('./db.shelve', writeback=True)
db.setdefault('reply-ids', {})


# Initialize bot and dispatcher
bot = Bot(token=API_TOKEN)
dp = Dispatcher(
    bot,
    # storage for FSM
    storage=JSONStorage('./storage.json')
)

dp.setup_middleware(LoggingMiddleware(logger=__name__))


def report_errors(func):
    """ Decorator: in case of exceptions, report them back to the user """
    async def wrapper(message: types.Message, *args, **kwargs):
        try:
            return await func(message, *args, **kwargs)
        except Exception as e:
            await message.reply(f'🛑 Error: {e}')
            raise e
    return wrapper


# region Simple commands

@dp.message_handler(commands=['start', 'help'])
async def send_welcome(message: types.Message):
    """ Start command """
    await message.reply("Hi!\nI'm EchoBot!\nPowered by aiogram.")


@dp.message_handler(commands=['msg'])
async def msg(message: types.Message):
    """ /msg

    Syntax: <chat_id> <text>

    Send a message to chat by id
    """
    # Arguments: <chat_id> <text>
    chat_id, text = message.get_args().split(' ', 1)

    # Send it
    await bot.send_message(chat_id, text=text)


# endregion

# region FSM Storage

@dp.message_handler(commands=['fsm_count'])
async def cmd_fsm(message: types.Message, state: FSMContext):
    """ /fsm_count

    Example FSM: incrementable counter
    """
    # Init the FSM
    async with state.proxy() as proxy:  # proxy = FSMContextProxy(state); await proxy.load()
        # Increment the counter for this chat, starting with 0
        proxy.setdefault('counter', 0)
        proxy['counter'] += 1

        # Reply
        return await message.reply(f"Counter: {proxy['counter']}")

# endregion

# region FSM States


class Form(StatesGroup):
    name = State()  # Will be represented in storage as 'Form:name'
    age = State()  # Will be represented in storage as 'Form:age'
    gender = State()  # Will be represented in storage as 'Form:gender'


@dp.message_handler(commands='fsm_state')
async def cmd_start(message: types.Message):
    """ Conversation's entry point """
    # Init state for this conversation
    await Form.name.set()

    # First question
    await message.reply("Hi there! What's your name?")


@dp.message_handler(state='*', commands='cancel')  # '*' works with any state
@dp.message_handler(filters.Text(equals='cancel', ignore_case=True), state='*')
async def cancel_handler(message: types.Message, state: FSMContext):
    """ Allow user to cancel any action """
    # Get state, if any
    current_state = await state.get_state()
    if current_state is None:
        return

    logging.info('Cancelling state %r', current_state)

    # Cancel state
    await state.finish()

    # And remove keyboard (just in case)
    await message.reply('Cancelled.', reply_markup=types.ReplyKeyboardRemove())


@dp.message_handler(state=Form.name)
async def process_name(message: types.Message, state: FSMContext):
    """ Process user name """
    # Input
    async with state.proxy() as data:
        data['name'] = message.text

    # Proceed
    await Form.next()
    await message.reply("How old are you?")


@dp.message_handler(state=Form.age)
async def process_age(message: types.Message, state: FSMContext):
    # Input
    await state.update_data(age=int(message.text))

    # Proceed
    await Form.next()
    markup = types.ReplyKeyboardMarkup(resize_keyboard=True, selective=True)
    markup.add("Male", "Female")
    markup.add("Won't say")
    await message.reply("What is your gender?", reply_markup=markup)


@dp.message_handler(state=Form.gender)
async def process_gender(message: types.Message, state: FSMContext):
    async with state.proxy() as data:
        # Input
        data['gender'] = message.text

        # Remove keyboard
        markup = types.ReplyKeyboardRemove()

        # And send message
        await bot.send_message(
            message.chat.id,
            md.text(
                md.text('Hi! Nice to meet you,', md.bold(data['name'])),
                md.text('Age:', md.code(data['age'])),
                md.text('Gender:', data['gender']),
                sep='\n',
            ),
            reply_markup=markup,
            parse_mode=ParseMode.MARKDOWN,
        )

    # Finish conversation
    await state.finish()

# endregion

# region Incoming requests

@dp.message_handler(filters.IDFilter(chat_id=ADMIN_CHAT_ID), filters.IsReplyFilter(True))
async def incoming_reply_in_admin_chat(message: types.Message):
    """ Incoming reply in the admin chat

    If an admin replies to a message, forward this reply to the original author.
    This lets anyone communicate with an admin through the bot.
    """
    # Look-up where it comes from
    mem = db['reply-ids'][message.reply_to_message.message_id]

    # Respond
    print(f'✅ Received admin reaction on {message.reply_to_message.message_id=}, original {mem=}')
    await bot.send_message(mem['chat-id'], message.md_text,
                           parse_mode=ParseMode.MARKDOWN_V2,
                           reply_to_message_id=mem['message-id'])

    # Remove the mem
    del db['reply-ids'][message.reply_to_message.message_id]
    db.sync()


# NOTE: this handler MUST be registered in the end! otherwise it will catch messages early on
@dp.message_handler()
async def incoming(message: types.Message):
    """ Handle every random incoming message

    Such messages are forwarded to ADMIN_CHAT_ID.
    If an admin replies ↶ to it, the response is also sent to the user: see incoming_reply_in_admin_chat()
    """
    # Get the current user. Same as message.user
    user = types.User.get_current()

    # Log
    print('=== Incoming message from', user)
    print(message)

    # Status: typing...
    await message.answer_chat_action('typing')

    # Send the message to the admin
    from_user_line = f'[{md.escape_md(message.from_user.full_name)}](tg://user?id={message.from_user.id})'
    res = await bot.send_message(
        ADMIN_CHAT_ID,
        f'➡️ {from_user_line}\n'
        f'{message.md_text}',
        parse_mode=ParseMode.MARKDOWN_V2,
        # Include a button: "take request"
        reply_markup=InlineKeyboardMarkup(2).add(
            InlineKeyboardButton('Взять заявку',
                                 callback_data=f'take-request:{message.chat.id}:{message.message_id}')
        ),
    )

    print(f'✅ Forwarded {message.chat.id=} {message.message_id=} as {res.message_id=}')

    # Remember it until an admin replies
    # If admin replies to {res.message_id} in the admin chat, we send a reply to the original {chat-id} {message-id}
    db['reply-ids'][res.message_id] = {
        'chat-id': message.chat.id,
        'message-id': message.message_id,
        'ctime': datetime.now()
    }
    db.sync()


@dp.callback_query_handler()
async def incoming_callback(callback_query: types.CallbackQuery):
    """ Handle button clicks """
    # Take request:
    # The message will be forwarded privately to the person who clicked
    if callback_query.data.startswith('take-request:'):
        # Parse
        _, chat_id, message_id = callback_query.data.split(':')

        # Forward to the person
        await bot.forward_message(callback_query.from_user.id, chat_id, message_id)

        # Remove the button
        await bot.edit_message_reply_markup(callback_query.message.chat.id, callback_query.message.message_id, reply_markup=None)

# endregion

async def on_startup(dispatcher: Dispatcher):
    await bot.set_my_commands([
        BotCommand('/msg', 'Send message to chat'),
        BotCommand('/fsm_count', 'Stateful counter'),
        BotCommand('/fsm_state', 'Stateful form'),
        BotCommand('/test', 'Test feature'),
    ])


# Finally: long polling
if __name__ == '__main__':
    executor.start_polling(dp, on_startup=on_startup)
