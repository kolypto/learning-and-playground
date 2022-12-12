from telegram.client import Telegram

tg = Telegram(
    api_id='26749369',
    api_hash='5688fc4d5a0a821a1316f471fb996917',
    phone='+79010099553',
    database_encryption_key='changeme',
    files_directory='/tmp/.tdlib_files/'
)
tg.login()

result = tg.call_method('account.updateEmojiStatus', {
    'emoji_status': {
        'document_id': 0,
        # 'until': 0,
    },
})
result.wait(raise_exc=True)
print(result.update)

