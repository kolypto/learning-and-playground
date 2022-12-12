from functools import lru_cache

from fastapi import FastAPI
from fastapi.params import Depends
from fastapi.responses import ORJSONResponse
from starlette.responses import HTMLResponse

app = FastAPI(
    # a faster JSON encoder
    default_response_class=ORJSONResponse
)


# But if you return a Response directly, the data won't be automatically converted,
# and the documentation won't be automatically generated (for example, including the specific
# "media type", in the HTTP header Content-Type as part of the generated OpenAPI).


# JSON performance

# if you are squeezing performance, you can install and use orjson and set the response to be ORJSONResponse.
@app.get("/items/", response_class=ORJSONResponse)
async def read_items():
    return [{"item_id": "Foo"}]




# Serving by a proxy, with a prefix /api
# $ uvicorn main:app --root-path /api/v1
#   or
# app = FastAPI(root_path="/api/v1")





# Events

@app.on_event("startup")
async def startup_event():
    # Do things when the app initializes
    pass


@app.on_event("shutdown")
def shutdown_event():
    # Do things when the app is terminating
    pass










# Custom Request
# Use this to prepare requests' body: msgpack, gzip, etc

import gzip
from typing import Callable, List

from fastapi import Body, FastAPI, Request, Response
from fastapi.routing import APIRoute

class GzipRequest(Request):
    """ Custom request that un-gzips itself """
    async def body(self) -> bytes:
        if not hasattr(self, "_body"):
            body = await super().body()
            if "gzip" in self.headers.getlist("Content-Encoding"):
                body = gzip.decompress(body)
            self._body = body
        return self._body


class GzipRoute(APIRoute):
    """ Custom route """
    # Returns a callable.
    # Basically, acts like a middleware
    def get_route_handler(self) -> Callable:
        original_route_handler = super().get_route_handler()

        async def custom_route_handler(request: Request) -> Response:
            request = GzipRequest(
                # ASGI spec
                request.scope,  # request metadata
                request.receive  # a function to "receive" the body of the request.
            )

            # Call the parent route handler
            # NOTE: try..except block can be used to catch exceptions , e.g. validation errors
            # You can also use it for timing requests
            return await original_route_handler(request)

        return custom_route_handler


app = FastAPI()
app.router.route_class = GzipRoute  # use our custom classes







# Validation error logging
# We'll use APIRoute's handler as a middleware

from fastapi.exceptions import HTTPException, RequestValidationError


class ValidationErrorLoggingRoute(APIRoute):
    def get_route_handler(self) -> Callable:
        original_route_handler = super().get_route_handler()

        async def custom_route_handler(request: Request) -> Response:
            try:
                # Call the route, and, the operation
                return await original_route_handler(request)
            except RequestValidationError as exc:
                # Process validation errors
                body = await request.body()
                detail = {"errors": exc.errors(), "body": body.decode()}

                # Make an exception
                raise HTTPException(status_code=422, detail=detail)

        return custom_route_handler


app = FastAPI()
app.router.route_class = ValidationErrorLoggingRoute










# Configuration

from pydantic import BaseSettings

# Settings with defaults and values coming from the environment variables
class Settings(BaseSettings):
    # Pydantic will read the environment variables in case-insensitive way
    app_name: str = "Awesome API"
    admin_email: str
    items_per_user: int = 50

    class Config:
        # Read from .env files
        env_file = '.env'


# Global object
settings = Settings(
    # Optionally, give it the name of an .env file
    _env_file='prod.env'
)


# In bigger apps, it may be better to provide it as a dependency
# Why? Because then you can override it in tests!
@lru_cache()
def get_settings():
    return Settings()


@app.get("/info")
async def info(settings: Settings = Depends(get_settings)):
    return {
        "app_name": settings.app_name,
        "admin_email": settings.admin_email,
        "items_per_user": settings.items_per_user,
    }













# Custom OpenAPI generator
from fastapi.openapi.utils import get_openapi

# Custom method
def custom_openapi():
    # Cached?
    if app.openapi_schema:
        return app.openapi_schema

    # Generate
    openapi_schema = get_openapi(
        title="Custom title",
        version="2.5.0",
        description="This is a very custom OpenAPI schema",
        routes=app.routes,
    )

    # Modify
    openapi_schema["info"]["x-logo"] = {
        "url": "https://fastapi.tiangolo.com/img/logo-margin/logo-teal.png"
    }

    # Cache
    app.openapi_schema = openapi_schema

    # Done
    return app.openapi_schema


# Replace the method
app.openapi = custom_openapi











# Mounting sub-applications
# Django, Flask, etc

from fastapi.middleware.wsgi import WSGIMiddleware

flask_app = ...

# Wrap it into a WSGIMiddleware, and mount
app.mount("/v1", WSGIMiddleware(flask_app))
