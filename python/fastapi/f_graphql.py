import graphene
from fastapi import FastAPI
from graphql import ResolveInfo
from starlette.graphql import GraphQLApp

# The query class

class Query(graphene.ObjectType):
    # `hello`: a field ...
    hello = graphene.String(
        # ... with a parameter. Which makes it a method.
        name=graphene.String(default_value="stranger")
    )

    def resolve_hello(self, info: ResolveInfo, name):
        """ Test: hello world """
        return "Hello " + name

    # `user_agent`: a field .. without parameters

    user_agent = graphene.String()

    def resolve_user_agent(self, info: ResolveInfo):
        """ Return the User-Agent of the incoming request. """
        request = info.context["request"]
        return request.headers.get("User-Agent", "<unknown>")


# The app

app = FastAPI()

# Responds to both GET and POST
app.add_route("/", GraphQLApp(schema=graphene.Schema(query=Query)))
