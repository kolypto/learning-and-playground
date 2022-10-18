from multiprocessing.connection import wait
import tartiflette as ta
import tartiflette.execution.types
import tartiflette_asgi
import uvicorn

# Run me:
# $ uvicorn 01-intro.py:app

@ta.Resolver("Query.hello")
async def resolve_hello(parent, args: dict, ctx: dict, info: ta.execution.types.ResolveInfo):
    print(parent, args, ctx, info)
    return "Hello"

LARGE_DICT = [
    dict.fromkeys(('a', 'b', 'c', 'd'), 'hey')
    for _ in range(1000)
]

@ta.Resolver("Query.loadMany")
async def resolve_load_many(parent, args: dict, context: dict, info: ta.execution.types.ResolveInfo):
    return LARGE_DICT

@ta.Resolver("Mutation.changeHello")
async def resolve_Change_hello(parent, args: dict, context: dict, info: ta.execution.types.ResolveInfo):
    print('ChangeHello', args['text'])


@ta.Subscription('Subscription.countdown')
async def resolve_countdown(parent, args: dict, context: dict, info: ta.execution.types.ResolveInfo):
    for i in range(3):
        yield {'countdown': i}


sdl = '''
type Query {
    hello: String!
    loadMany: [Object!]!
}

type Mutation {
    changeHello(text: String!): String
}

type Subscription {
    countdown: Int!
}

type Object {
    a: String
    b: String
    c: String
    d: String
}
'''

app = tartiflette_asgi.TartifletteApp(
    # SDL can be: GraphQL, path to file, list of paths to files, path[s] to a directory
    sdl=sdl
)


async def main():
    await app.startup()

    # === query
    res = await app.engine.execute(''' query { hello } ''')
    print(res)

    # === query: large
    import time
    t1 = time.time()
    await app.engine.execute(''' query { loadMany { a b c d } } ''' )
    total = time.time() - t1
    print(f'Large Query, Tartiflette: total={total:.02f}')

    import ariadne
    Query = ariadne.QueryType()
    @Query.field('loadMany')
    def resolve_load_many(_, info):
        return LARGE_DICT
    schema = ariadne.make_executable_schema(sdl, Query)
    t1 = time.time()
    res = await ariadne.graphql(schema, data={'query': ''' query { loadMany { a b c d } } ''' })
    total = time.time() - t1
    print(f'Large Query, Ariadne: total={total:.02f}')

    # === mutation
    res = await app.engine.execute(''' mutation { changeHello(text: "LOL") } ''')
    print(res)

    # === subscription
    res = [res async for res in app.engine.subscribe(''' subscription { countdown } ''')]
    print(res)


import asyncio
asyncio.run(main())
