import graphene

class UserQuery(graphene.ObjectType):
    hello = graphene.String(
        name=graphene.String(default_value='stranger')
    )

    def resolve_hello(_, info: graphene.ResolveInfo, *, name):
        return f'Hello {name}'

class Query(UserQuery, graphene.ObjectType):
    pass

class ChangeNameMutation(graphene.Mutation):
    class Arguments:
        name = graphene.String()

    ok = graphene.Boolean()

    def mutate(_, info: graphene.ResolveInfo, *, name):
        return {'ok': True}

class Mutation(graphene.ObjectType):
    changeName = ChangeNameMutation.Field()


class Subscription(graphene.ObjectType):
    countdown = graphene.String()

    async def subscribe_countdown(root, info: graphene.ResolveInfo):
        for i in range(3):
            yield i

schema = graphene.Schema(query=Query, mutation=Mutation, subscription=Subscription, auto_camelcase=True)

result = schema.execute(''' query { hello(name: "User") } ''')
print(result)

result = schema.execute(''' mutation { changeName(name: "User") { ok } } ''')
print(result)

import asyncio
async def subscription_load(q: str):
    return [res async for res in await schema.subscribe(q)]
result = asyncio.run(subscription_load(''' subscription { countdown } '''))
print(result)


import uvicorn
from starlette.applications import Starlette
from starlette_graphene3 import GraphQLApp, make_playground_handler

app = Starlette()
app.mount("/", GraphQLApp(schema, on_get=make_playground_handler()))

uvicorn.run(app)
