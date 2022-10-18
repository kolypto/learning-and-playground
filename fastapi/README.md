# FastAPI Tutorial

* <a href="https://github.com/kolypto/my-learn-fastapi#a_intropy">Inroduction</a>
* <a href="https://github.com/kolypto/my-learn-fastapi#b_dipy">Dependency Injection</a>
* <a href="https://github.com/kolypto/my-learn-fastapi#c_authenticationpy">Authentication</a>
* <a href="https://github.com/kolypto/my-learn-fastapi#d_dbpy">Database</a>
* <a href="https://github.com/kolypto/my-learn-fastapi#e_advancedpy">Advanced Stuff</a>
* <a href="https://github.com/kolypto/my-learn-fastapi#f_graphqlpy">GraphQL</a>
* <a href="https://github.com/kolypto/my-learn-fastapi#g_websocketspy">Websockets</a>
* <a href="https://github.com/kolypto/my-learn-fastapi#h_pydanticpy">Pydantic Primer</a>
* <a href="https://github.com/kolypto/my-learn-fastapi#testspy">/tests/</a>
* <a href="https://github.com/kolypto/my-learn-fastapi#pytest-primer">Pytest Primer</a>
# a_intro.py
```python
from __future__ import annotations
import time
from datetime import datetime
from enum import Enum
from random import randint
from typing import Optional, List
from pydantic import BaseModel, Field, HttpUrl
from starlette.middleware.cors import CORSMiddleware
from starlette.responses import HTMLResponse, JSONResponse, PlainTextResponse
from fastapi import FastAPI, Depends, Request, Response, status, HTTPException
from fastapi import Query, Path, Body, Cookie, Header, Form, File, UploadFile

# CookieCutter template:
# https://fastapi.tiangolo.com/tutorial/sql-databases/


app = FastAPI(
    # App title and version, used in docs
    title='simplest',
    version='0.0.1',
    openapi_tags=[
        # API categories' info
        # The order of tags defines the order of categories in the API docs
        {'name': 'users', 'description': "User API"},
        {'name': 'integration:google',
         # Category with more information elsewhere
         "externalDocs": {
             "description": "Items external docs",
             "url": "https://fastapi.tiangolo.com/",
            },
         }
    ],
    # Return debug tracebacks on errors
    debug=True,
    # OpenAPI URL can be customized.
    # Set it to `None` to disable completely
    openapi_url="/api/v1/openapi.json",
    docs_url='/docs',
    redoc_url='/redoc',
    servers=[
        # Other servers:
        # in case you want the docs to interact with the "staging" and "production" servers too.
        # You'll be able to select a server in a drop-down
        {"url": "https://stag.example.com", "description": "Staging environment"},
        {"url": "https://prod.example.com", "description": "Production environment"},
    ],
)

# Running:
# uvicorn a_intro:app --reload

# OpenAPI docs: http://127.0.0.1:8000/docs
# ReDoc:        http://127.0.0.1:8000/redoc
# OpenAPI JSON: http://127.0.0.1:8000/openapi.json

# Operations:
# POST: to create data.
# GET: to read data.
# PUT: to update data (whole or partial).
# DELETE: to delete data.
# PATCH: for partial updates (some teams use it)






# http://127.0.0.1:8000/
@app.get("/", status_code=200)
# When you use the async methods, FastAPI runs the file methods in a threadpool and awaits for them.
async def read_root(
        response: Response,
        user_agent: str = Header(None),  # takent from headers
        user_id: int = Cookie(None),  # taken from a cookie
    ):
    # Set a cookie (on a temporary response object)
    response.set_cookie('user_id', randint(0,999))

    # Set a header
    response.headers["X-User-id"] = 'abc'

    # Customize the status code
    response.status_code = status.HTTP_200_OK

    # You can return:
    # dict, list, singular values as str, int, etc.
    # Pydantic models
    # many other objects and models
    # ORM models
    return {"Hello": "World",
            'user_id': user_id,
            'User-Agent': user_agent,
            }









# Using enums
class ItemType(str, Enum):
    ALEXNET = "alexnet"
    RESNET = "resnet"
    LENET = "lenet"


# http://127.0.0.1:8000/items/5?q=somequery
# One path parameter, one query parameter
@app.get("/items/{item_id}")
def read_item_by_id(
        # singular parameters (like int, float, str, bool, etc) are interpreted as query parameters
        # Pydantic models are interpreted as a request body.
        # Use Path(), Query(), Body() to mark parameters' source
        item_id: int = Path(..., # no default
                            title='Title for OpenAPI docs'
                            ),
        # query parameter
        q: Optional[str] = Query('fixedquery',  # default; use `...` for no default
                                 # validation
                                 min_length=3,
                                 max_length=60,
                                 regex="^fixedquery$"
                                 ),
        # This query parameter can be provided multiple times
        item_type: List[ItemType] = Query(
            [],
            title='Title for OpenAPI',
            description='Help text for OpenAPI',
            alias='itemType',  # another name, e.g. when not a valid Python identifier
            deprecated=True,  # stop using it
        )
        ):
    # Enums will be converted to their **values** (not names)
    return {"item_id": item_id, "q": q, 'item_type': item_type}







# OpenAPI documentation

# operationId
@app.get("/items/",
         # 'tags' used for OpenAPI docs navigation as categories
         tags=['items'],
         # Describe the API (OpenAPI docs)
         summary='API summary (right next to the title)',
         response_description='Description for the response value',
         # Deprecate an API (OpenAPI docs)
         deprecated=True,
         # Some code generators will use them.
         # Got to be unique.
         operation_id="some_specific_id_you_define",
         # Response schema
         responses={
             # Will go into the OpenAPI schema
             # '404': {"model": Item},
             200: {
                 # Custom description
                 "description": "Item requested by ID",
                 "content": {
                     "application/json": {
                         # Custom example for a content-type
                         "example": {"id": "bar", "value": "The bar tenders"}
                     }
                 },
             },
         },
         )
async def read_items():
    """ This **markdown** docstring will be used in OpenAPI

    \f
    Text after this "form feed" won't be included in OpenAPI

    Returns:
        something
    """
    return [{"item_id": "Foo"}]

# If you want to use function names as operation ids, you've got to do it manually:
#     for route in app.routes:
#         if isinstance(route, APIRoute):
#             route.operation_id = route.name  # in this case, 'read_items'
# You'll have to make sure that they all have unique names.


@app.get("/items/",
         # Exclude from OpenAPI schema and generated documentation
         include_in_schema=False)
async def read_items():
    return [{"item_id": "Foo"}]







# Using paths
@app.get("/item_path/{path:path}")
def read_item_by_type(path: str):
    return {'path': path}








# Interface: data model, saving objects
class Item(BaseModel):
    # Pydantic validation
    # (note: Query(), Path(), Body() are subclasses of Pydantic.field)
    name: str = Field(..., title='Name', description='Name', min_length=1)
    price: float
    is_offer: Optional[bool] = None
    when: Optional[datetime]
    image: Optional[Image]  # a forward reference!

    # Customize example
    class Config:
        schema_extra = {
            # Example: will go into the docs
            "example": {
                "name": "Foo",
                "description": "A very nice Item",
                "price": 35.4,
                "tax": 3.2,
            }
        }

class Image(BaseModel):
    url: HttpUrl = Field(...,
                         # Docs info
                         title='Image URL',
                         example='http://example.com/example.png')
    name: str

Item.update_forward_refs()  # when forward references fail to update


class PutItemResponse(BaseModel):
    item_name: str
    item_id: int
    with_default_value: int = 0

# Saving: body as JSON (`item`)
@app.put("/items/{item_id}",
         status_code=status.HTTP_201_CREATED,  # created
         # Response model
         response_model=PutItemResponse,
         # Remove default values; only use those explicitly set
         response_model_exclude_unset=True,
         # Remove None values
         response_model_exclude_none=True,
         # Include/exclude individual fields (like the password field)
         # NOTE: it is recommended to use separate classes instead
         response_model_include=[],
         response_model_exclude=[],
         )
def update_item(item_id: int, item: Item):
    return {"item_name": item.name, "item_id": item_id}








# Partial updates
# NOTE: the input model is still validated! If you need to skip it, create a new model with all fields optional
@app.patch("/items/{item_id}", response_model=Item)
async def update_item(item_id: str, item: Item):
    # Load
    db_item: Item = ...  # load it somehow and convert to a Pydantic type (from_orm())

    # Update the item using dict(exclude_unset=True) + copy(update)
    update_data = item.dict(exclude_unset=True)  # `exclude_unset`: omit defaults; only use values set by the user
    updated_item = db_item.copy(update=update_data)  # partial update

    # Save
    ...(jsonable_encoder(updated_item))

    # DOne
    return updated_item










# Saving from an HTML form
# OAUTH2 spec says those fields have to be named 'username' and 'password' and sent as form fields
@app.post("/login/")
async def login(username: str = Form(...), password: str = Form(...)):
    return {"username": username, 'password': 'no way'}











# Example recursive models

# NOTE: in real world, you'll likely have multiple related models:
# * UserIn (input, with password)
# * UserOut (output, no password)
# * UserDB (DB model)
# * UserPart (partial user input)
# and do like this:
#   UserInDB(**user_in.dict(), hashed_password=hashed_password)
#
# But to reduce code duplication:
# use class inheritance ; Union[] ; generate partial classes

class User(BaseModel):
    id: int
    devices: List[Device]

class Device(BaseModel):
    uid: Optional[int]
    user: Optional[User]

User.update_forward_refs()
Device.update_forward_refs()

@app.put('/save')
def save(user: User):
    pass










# HTML response

@app.get("/items/", response_class=HTMLResponse)
async def read_items():
    return """
    <html>...</html>
    """









# Templates
from fastapi.templating import Jinja2Templates
templates = Jinja2Templates(directory="templates")


@app.get("/items/{id}")
async def read_item(request: Request, id: str):
    return templates.TemplateResponse("item.html", {"request": request, "id": id})












# Receiving files
@app.post("/files/")
async def create_file(
        # Stream of bytes
        # the whole contents will be stored in memory. For small files!
        file: bytes = File(...)):
    return {"file_size": len(file)}


# UploadFile
@app.post("/uploadfile/")
async def create_upload_file(
        # Uploaded file, partly on disk, with metadata available ("spooled" file)
        # It also has an async interface
        file: UploadFile = File(...)):
    return {"filename": file.filename,
            "contents": await file.read()
            }

# Upload many files at once
@app.post("/uploadfiles/")
async def create_upload_files(files: List[UploadFile] = File(...)):
    return {"filenames": [file.filename for file in files]}


@app.get("/")
async def main():
    content = """
        <body>
        <form action="/files/" enctype="multipart/form-data" method="post">
            <input name="files" type="file" multiple>
            <input type="submit">
        </form>
        <form action="/uploadfiles/" enctype="multipart/form-data" method="post">
            <input name="files" type="file" multiple>
            <input type="submit">
        </form>
        </body>
    """
    return HTMLResponse(content=content)









# Serve static files

# $ pip install aiofiles
from fastapi.staticfiles import StaticFiles
app.mount(  # An independent application is mounted, and is responsible to handling sub-paths
    "/static",  # URL
    StaticFiles(directory="static"),  # serve files from here
    name="static",  # internal name for referencing
)












# Report errors
@app.get("/item-by-id/{item_id}",
         # Predefined responses for certain error codes
         responses={
             404: {"description": "Not found"},
         }
         )
async def get_item_by_id(item_id: str):
    items = {"foo": "The Foo Wrestlers"}
    if item_id not in items:
        raise HTTPException(status_code=404,
                            # Will return a JSON response {'detail': {'msg': ...}}
                            detail={'msg': "Item not found"})
    return {"item": items[item_id]}

# Globally convert UnicornException to HTTPException
class UnicornException(Exception): pass

@app.exception_handler(UnicornException)
async def unicorn_exception_handler(request: Request, exc: UnicornException):
    return JSONResponse(
        status_code=418,
        content={"message": f"Oops! {exc.name} did something. There goes a rainbow..."},
    )


# Advanced: override the exception handler for validation errors
if False:
    from fastapi.encoders import jsonable_encoder

    @app.exception_handler(RequestValidationError)
    async def validation_exception_handler(request: Request, exc: RequestValidationError):
        # If you return a JSON response directly, it has to be ready for json.dumps()
        # When you return a Response directly its data is not validated, converted (serialized), nor documented automatically.
        return JSONResponse(
            status_code=status.HTTP_422_UNPROCESSABLE_ENTITY,
            # exc.body: the body it received with invalid data.
            # jsonable_encoder() is what @vdmit11 calls "jsonify()" :)
            content=jsonable_encoder({"detail": exc.errors(), "body": exc.body}),
        )

        # you can reuse the default handler
        from fastapi.exception_handlers import request_validation_exception_handler
        request_validation_exception_handler(request, exc)

# Advanced: override the exception handler for HTTP errros
if False:
    @app.exception_handler(StarletteHTTPException)
    async def http_exception_handler(request, exc):
        return PlainTextResponse(str(exc.detail), status_code=exc.status_code)






# Middleware
# Wraps path operations
@app.middleware("http")
async def add_process_time_header(request: Request, call_next):
    # Measure execution time
    start_time = time.time()
    response = await call_next(request)
    process_time = time.time() - start_time

    # Add header
    response.headers["X-Process-Time"] = str(process_time)
    return response



# Built-in middlewares

# Redirect all traffic to "https" and "wss"
from fastapi.middleware.httpsredirect import HTTPSRedirectMiddleware
app.add_middleware(HTTPSRedirectMiddleware)

# Enforces that all incoming requests have a correctly set Host header, in order to guard against HTTP Host Header attacks.
from fastapi.middleware.trustedhost import TrustedHostMiddleware
app.add_middleware(
    TrustedHostMiddleware, allowed_hosts=["example.com", "*.example.com"]
)

# gzip responses
from fastapi.middleware.gzip import GZipMiddleware
app.add_middleware(GZipMiddleware, minimum_size=1000)

# Sentry ASGI middleware
# https://docs.sentry.io/platforms/python/asgi/

# MessagePack content negotiation
# Benefits: reduced bandwidth usage (binary JSON)
# https://github.com/florimondmanca/msgpack-asgi






# CORS
# Cross-origin: when the UI runs on a host different from the API
# Origin: scheme+domain+port

# Then, the browser will send an HTTP OPTIONS request to the backend, and if the backend sends
# the appropriate headers authorizing the communication from this different origin,
# then the browser will let the JavaScript in the frontend send its request to the backend.
#
# To achieve this, the backend must have a list of "allowed origins".

# It's also possible to declare the list as "*" (a "wildcard") to say that all are allowed.
# But that will only allow certain types of communication, excluding everything that
# involves credentials: Cookies, Authorization headers like those used with Bearer Tokens, etc.
#
# So, for everything to work correctly, it's better to specify explicitly the allowed origins.

# You can configure it in your FastAPI application using the CORSMiddleware.
origins = [
    "http://localhost.tiangolo.com",
    "https://localhost.tiangolo.com",
    "http://localhost",
    "http://localhost:8080",
]

app.add_middleware(
    CORSMiddleware,
    # Origins permitted to make cross-origin requests
    allow_origins=origins,  # as a list
    allow_origin_regex=None,  # as a regexp
    # Allowed
    allow_credentials=True,  # Credentials (Authorization headers, Cookies, etc).
    allow_methods=["*"],  # HTTP methods
    allow_headers=["*"],  # HTTP headers
    # List of response HTTP headers that should be made accessible to the browser
    expose_headers=[],
)















# Blueprints

from fastapi import APIRouter

router = APIRouter()

# Create your operations
@router.get("/users/", tags=["users"])
async def read_users():
    return [{"username": "Foo"}, {"username": "Bar"}]


def get_token_header():pass


# Add it to the app
# It will actually create all operations on `app`
app.include_router(router)  # simple

app.include_router(
    # Advanced
    router,
    # URL prefix to use for every operation
    prefix="/items",
    # Predefined operation arguments that will be added to all operations
    tags=["items"],  # same tag to all operations
    dependencies=[Depends(get_token_header)],  # evaluated for all operations
    responses={404: {"description": "Not found"}},  # predefined responses
)











# Background tasks

from fastapi import BackgroundTasks


# Any function, async or not, can be run in the background
def write_notification(email: str, message=""):
    with open("log.txt", mode="w") as email_file:
        content = f"notification for {email}: {message}"
        email_file.write(content)


@app.post("/send-notification/{email}")
async def send_notification(email: str, background_tasks: BackgroundTasks):
    # Run a task in the background
    # The task will be run in the same process. If it does heavy computation, use Celery
    background_tasks.add_task(
        # Function, parameters
        write_notification,
        email,
        message="some notification")
    return {"message": "Notification sent in the background"}





















# Run (debug)
if __name__ == "__main__":
    uvicorn.run(app, host="0.0.0.0", port=8000)

```

# b_di.py
```python
from http.client import HTTPException
from typing import Optional

from fastapi import FastAPI
from fastapi.params import Depends, Cookie, Header

app = FastAPI()


# A dependency is just a callable: a factory function, a class, ..
# It takes parameters the same way a path operation function ("view") does: Query, Path, Body, etc.
# Same defaults apply: simple types are query ; complex are body
async def common_parameters(  # use `async` or not ; FastAPI knows what to do
        # These go as query parameters (?q=&skip=&limit=) for every view that uses it
        # All parameters are optional
        q: Optional[str] = None,
        skip: int = 0,
        limit: int = 100):
    return {
        "q": q,
        "skip": skip,
        "limit": limit
    }

# Dependency is provided: it takes (q,skip,limit) from every query and provides a dict
@app.get("/items/")
async def read_items(commons: dict = Depends(common_parameters)):
    return commons

@app.get("/users/")
async def read_users(commons: dict = Depends(common_parameters)):
    return commons






# Same dependency, as a class
class CommonQueryParams:
    def __init__(self, q: Optional[str] = None, skip: int = 0, limit: int = 100):
        self.q = q
        self.skip = skip
        self.limit = limit

@app.get("/items/")
async def read_items(
        # Note: a shortcut is used: Depends() gets its argument from the type hint
        commons: CommonQueryParams = Depends()
):
    pass







# Sub-dependencies: resolved automatically
def query_extractor(q: Optional[str] = None):
    return q


def query_or_cookie_extractor(
    q: str = Depends(query_extractor), last_query: Optional[str] = Cookie(None)
):
    if not q:
        return last_query
    return q


@app.get("/items/")
async def read_query(
        # If the same dependency is required twice, it's re-used and not called twice
        # Set `use_cache=False` to override
        query_or_default: str = Depends(query_or_cookie_extractor)
):
    return {"q_or_cookie": query_or_default}





# Dependencies without a return value
# If you don't need the return value, or a dependency provdes no value, but you need it run:
# list those in the path operation decorator:
async def verify_token(x_token: str = Header(...)):
    if x_token != "fake-super-secret-token":
        raise HTTPException(status_code=400, detail="X-Token header invalid")


async def verify_key(x_key: str = Header(...)):
    if x_key != "fake-super-secret-key":
        raise HTTPException(status_code=400, detail="X-Key header invalid")
    return x_key

@app.get("/items/",
         # Dependencies without return values
         dependencies=[Depends(verify_token), Depends(verify_key)]
         )
async def read_items():
    return [{"item": "Foo"}, {"item": "Bar"}]





# Dependencies with `yield` (that do some extra stuff after finishing)
async def get_db():  # optional sub-dependencies may go here
    # Preparation
    db = DBSession()
    # Return
    try:
        yield db
    # Clean-up
    finally:
        # NOTE: can't raise HTTP errors here and modify the response: it's already been finalized.
        # You can spawn background tasks, though
        db.close()

```

# c_authentication.py
```python
from datetime import timedelta, datetime
from typing import Optional, List

from fastapi import Depends, FastAPI, HTTPException
from fastapi.params import Security
from fastapi.security import OAuth2PasswordBearer, SecurityScopes
from jose import jwt, JWTError
from passlib.context import CryptContext
from pydantic import BaseModel, ValidationError
from starlette import status



# https://fastapi.tiangolo.com/tutorial/security/oauth2-jwt/



# openssl rand -hex 32
SECRET_KEY = "09d25e094faa6ca2556c818166b7a9563b93f7099f6f0f4caa6cf63b88e8d3e7"
ALGORITHM = "HS256"
ACCESS_TOKEN_EXPIRE_MINUTES = 30


# Security models

class Token(BaseModel):
    """ OAuth2 token """
    access_token: str
    token_type: str


class TokenData(BaseModel):
    """ OAuth2 token, encoded as JWT (JSON Web Token) """
    username: Optional[str] = None

    # Optional: security scopes
    scopes: List[str] = []

# hashing
pwd_context = CryptContext(schemes=["bcrypt"], deprecated="auto")


def verify_password(plain_password, hashed_password):
    """ Compare a pasword to its hash """
    return pwd_context.verify(plain_password, hashed_password)


def get_password_hash(password):
    """ Get password's hash """
    return pwd_context.hash(password)






# App

app = FastAPI()


# Use it in any API and the docs will automatically get an "Autnetication" page
oauth2_scheme = OAuth2PasswordBearer(
    # URL to send the username&password to
    # Relative URL; means "./token"
    # Using a relative URL is important to make sure your application keeps working even in an advanced use case like Behind a Proxy.
    tokenUrl="token",

    # Optional: available security scopes.
    # They will show up in the API docs
    scopes={
        "me": "Read information about the current user.",
        "items": "Read items.",
    },
)

# OAuth2 was designed so that the backend or API could be independent of the server that authenticates the user.
# But in this case, the same FastAPI application will handle the API and the authentication.

# In this example we are going to use OAuth2, with the "Password" flow, using a "Bearer" token.


# It will check for an "Authorization: Bearer xxxx" header value or respond with a 401, and return the token.
# It's now up to you to verify it
@app.get("/most-basic-oauth-token-getter/")
async def read_items(token: str = Depends(oauth2_scheme)):
    return {"token": token}






# Get the current user from the token
class User(BaseModel):
    """ A user """
    username: str
    email: Optional[str] = None
    full_name: Optional[str] = None
    disabled: Optional[bool] = None

class UserInDB(User):
    """ A user with a password """
    hashed_password: str

async def get_current_user(token: str = Depends(oauth2_scheme)):
    user = User(...)  # decode token, load the user
    return user

@app.get("/users/me")
async def read_users_me(current_user: User = Depends(get_current_user)):
    return current_user






# Authentication provider
# OAuth2: with the "password flow", the client/user must send a username and password fields as form data.
# OAuth2: you can also send the "scope": string of space-sep scopes.
# Scopes are normally mean security permissions required:
#   "users:read users:write instagram_basic https://www.googleapis.com/auth/drive"


from fastapi.security import OAuth2PasswordBearer, OAuth2PasswordRequestForm


def get_user(db, username: str):
    """ Load a user from the database """
    if username in db:
        user_dict = db[username]
        return UserInDB(**user_dict)


async def get_current_user(
        token: str = Depends(oauth2_scheme),
        # Optional: work with security scopes from the `Request`
        security_scopes: SecurityScopes = Depends(),
):
    """ Authenticate: get an OAuth2 user """
    # Optional: security scopes stuff for the exception
    if security_scopes.scopes:
        # In this exception, we include the scopes required (if any) as a string separated by spaces (using scope_str).
        # We put that string containing the scopes in in the WWW-Authenticate header (this is part of the spec).
        authenticate_value = f'Bearer scope="{security_scopes.scope_str}"'
    else:
        authenticate_value = f"Bearer"

    # Prepare an exception we might need
    credentials_exception = HTTPException(
        status_code=status.HTTP_401_UNAUTHORIZED,
        detail="Could not validate credentials",
        headers={"WWW-Authenticate": authenticate_value},
    )

    # Get data from the JWT token
    try:
        # Decode the token
        payload = jwt.decode(token, SECRET_KEY, algorithms=[ALGORITHM])
        # Get the username
        username: str = payload.get("sub")
        if username is None:
            raise credentials_exception

        # Prepare TokenData
        token_scopes = payload.get("scopes", [])
        token_data = TokenData(scopes=token_scopes, username=username)
    except (JWTError, ValidationError):
        raise credentials_exception

    # Optional: check scopes vs API operation scopes
    for scope in security_scopes.scopes:  # scopes required by the API operation
        # The important and "magic" thing here is that get_current_user will have a different list of scopes to check for each path operation.
        if scope not in token_data.scopes:  # scopes provided by the user
            raise HTTPException(
                status_code=status.HTTP_401_UNAUTHORIZED,
                detail="Not enough permissions",
                headers={"WWW-Authenticate": authenticate_value},
            )

    # Load a user
    user = get_user(..., username=token_data.username)
    if user is None:
        raise credentials_exception
    return user


# async def get_current_active_user(current_user: User = Depends(get_current_user)):
async def get_current_active_user(
        # Depends() using Security() with a parameter: `scopes`
        # FastAPI will know that this is the permission required
        current_user: User = Security(get_current_user, scopes=['me'])
):
    """ Authenticate: get an OAuth2 *active* user """
    if current_user.disabled:
        raise HTTPException(status_code=400, detail="Inactive user")
    return current_user


def authenticate_user(fake_db, username: str, password: str):
    """ Identify & authorize """
    user = get_user(fake_db, username)
    if not user:
        return False
    if not verify_password(password, user.hashed_password):
        return False
    return user


def create_access_token(data: dict, expires_delta: Optional[timedelta] = None):
    """ Create a JWT token from a JSON object """
    to_encode = data.copy()

    # Expiration
    if expires_delta:
        expire = datetime.utcnow() + expires_delta
    else:
        expire = datetime.utcnow() + timedelta(minutes=15)
    to_encode.update({"exp": expire})

    # Encode
    encoded_jwt = jwt.encode(to_encode, SECRET_KEY, algorithm=ALGORITHM)
    return encoded_jwt



# OAuth2 authentication url
@app.post("/token", response_model=Token)
async def login(form_data: OAuth2PasswordRequestForm = Depends()):
    # Fields: username, password
    # Fields: grant_type="password" (fixed)
    # Fields: (optional) client_id, client_secret (not used here)

    # Authenticate
    form_data.scopes  # optional scopes
    user = authenticate_user(db, form_data.username, form_data.password)
    if not user:
        raise HTTPException(
            status_code=status.HTTP_401_UNAUTHORIZED,
            detail="Incorrect username or password",
            headers={"WWW-Authenticate": "Bearer"},
        )

    # Token
    access_token_expires = timedelta(minutes=ACCESS_TOKEN_EXPIRE_MINUTES)
    access_token = create_access_token(
        data={
            # The JWT specification says that there's a key sub, with the subject of the token.
            # It needs to be unique across the application
            # If you plan to provide accesses to other subjects (users, articles, etc), prefix if f'user:{login}'
            "sub": user.username,
        }, expires_delta=access_token_expires
    )

    # Response: must be a JSON object
    return {
        # Const: token type (must be "bearer")
        "token_type": "bearer",
        # String containing our access token
        "access_token": access_token,
    }


@app.get("/users/me")
async def read_users_me(current_user: User = Depends(get_current_active_user)):
    return current_user
```

# d_db.py
```python
# CookieCutter template:
# https://fastapi.tiangolo.com/tutorial/sql-databases/
from http.client import HTTPException

from fastapi import FastAPI
from fastapi.params import Depends
from sqlalchemy import create_engine
from sqlalchemy.ext.declarative import declarative_base
from sqlalchemy.orm import sessionmaker



# Init database
# database.py

SQLALCHEMY_DATABASE_URL = "sqlite:///./sql_app.db"
# SQLALCHEMY_DATABASE_URL = "postgresql://user:password@postgresserver/db"

engine = create_engine(
    SQLALCHEMY_DATABASE_URL,
    # ...is needed only for SQLite. It's not needed for other databases.
    connect_args={"check_same_thread": False}
)
SessionLocal = sessionmaker(autocommit=False, autoflush=False, bind=engine)

Base = declarative_base()



# SqlAlchemy Models
# models.py
from sqlalchemy import Boolean, Column, ForeignKey, Integer, String
from sqlalchemy.orm import relationship

class models:
    class User(Base):
        __tablename__ = "users"

        id = Column(Integer, primary_key=True, index=True)
        email = Column(String, unique=True, index=True)
        hashed_password = Column(String)
        is_active = Column(Boolean, default=True)

        items = relationship("Item", back_populates="owner")


    class Item(Base):
        __tablename__ = "items"

        id = Column(Integer, primary_key=True, index=True)
        title = Column(String, index=True)
        description = Column(String, index=True)
        owner_id = Column(Integer, ForeignKey("users.id"))

        owner = relationship("User", back_populates="items")


# Pydantic models
# schemas.py

from typing import List, Optional

from pydantic import BaseModel

class schemas:
    class ItemBase(BaseModel):
        """ Item: writable fields """
        title: str
        description: Optional[str] = None


    class ItemCreate(ItemBase):
        """ Item: creating (without ids) """
        pass


    class Item(ItemBase):
        """ Item: reading (with ids) """
        id: int
        owner_id: int

        class Config:
            # Pydantic model config
            # https://pydantic-docs.helpmanual.io/usage/model_config/
            orm_mode = True  # allow reading data from objects (will also try getattr())


    class UserBase(BaseModel):
        """ User: writable fields """
        email: str


    class UserCreate(UserBase):
        """ User: creating """
        password: str


    class User(UserBase):
        """ User: reading """
        id: int
        is_active: bool
        items: List[models.Item] = []

        class Config:
            orm_mode = True







# Create tables
Base.metadata.create_all(bind=engine)






# CRUD methods

from sqlalchemy.orm import Session

# User

# NOTE: By creating functions that are only dedicated to interacting with the database
# (get a user or an item) independent of your path operation function,
# you can more easily reuse them in multiple parts and also add unit tests for them.

def get_user(db: Session, user_id: int):
    return db.query(models.User).filter(models.User.id == user_id).first()


def get_user_by_email(db: Session, email: str):
    return db.query(models.User).filter(models.User.email == email).first()


def get_users(db: Session, skip: int = 0, limit: int = 100):
    return db.query(models.User).offset(skip).limit(limit).all()

def create_user(db: Session, user: schemas.UserCreate):
    db_user = models.User(
        # Every field
        email=user.email,
        hashed_password=user.password + "notreallyhashed")
    db.add(db_user)
    db.commit()
    db.refresh(db_user)
    return db_user

# Items

def get_items(db: Session, skip: int = 0, limit: int = 100):
    return db.query(models.Item).offset(skip).limit(limit).all()


def create_user_item(db: Session, item: schemas.ItemCreate, user_id: int):
    db_item = models.Item(
        # ModelBase.dict() makes a dict
        **item.dict(),
        owner_id=user_id
    )
    db.add(db_item)
    db.commit()
    db.refresh(db_item)
    return db_item








# CRUD operations

app = FastAPI()

# Dependency
def get_db():
    db = SessionLocal()
    try:
        yield db
    finally:
        db.close()

@app.post("/users/", response_model=schemas.User)
def create_user(user: schemas.UserCreate, db: Session = Depends(get_db)):
    db_user = get_user_by_email(db, email=user.email)
    if db_user:
        raise HTTPException(status_code=400, detail="Email already registered")
    return create_user(db=db, user=user)


@app.get("/users/", response_model=List[schemas.User])
def read_users(skip: int = 0, limit: int = 100, db: Session = Depends(get_db)):
    users = get_users(db, skip=skip, limit=limit)
    return users


@app.get("/users/{user_id}", response_model=schemas.User)
def read_user(user_id: int, db: Session = Depends(get_db)):
    db_user = get_user(db, user_id=user_id)
    if db_user is None:
        raise HTTPException(status_code=404, detail="User not found")
    return db_user


@app.post("/users/{user_id}/items/", response_model=schemas.Item)
def create_item_for_user(
    user_id: int, item: schemas.ItemCreate, db: Session = Depends(get_db)
):
    return create_user_item(db=db, item=item, user_id=user_id)


@app.get("/items/", response_model=List[schemas.Item])
def read_items(skip: int = 0, limit: int = 100, db: Session = Depends(get_db)):
    items = get_items(db, skip=skip, limit=limit)
    return items
```

# e_advanced.py
```python
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
```

# f_graphql.py
```python
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
```

# g_websockets.py
```python
from fastapi import FastAPI, WebSocket, status
from fastapi.responses import HTMLResponse
from starlette.websockets import WebSocketDisconnect

app = FastAPI()

# The websocket connection endpoint
@app.websocket("/ws")
async def websocket_endpoint(websocket: WebSocket):
    try:
        # Accept the connection
        await websocket.accept()

        # Loop
        while True:
            # Receive
            data = await websocket.receive_text()

            # Respond
            await websocket.send_text(f"Message text was: {data}")
    except WebSocketDisconnect as e:
        await websocket.close(code=e.code)



# The HTML page to test-drive it

html = """
<!DOCTYPE html>
<html>
    <head>
        <title>Chat</title>
    </head>
    <body>
        <h1>WebSocket Chat</h1>
        <form action="" onsubmit="sendMessage(event)">
            <input type="text" id="messageText" autocomplete="off"/>
            <button>Send</button>
        </form>
        <ul id='messages'>
        </ul>
        <script>
            var ws = new WebSocket("ws://localhost:8000/ws");
            
            ws.onmessage = function(event) {
                var messages = document.getElementById('messages')
                var message = document.createElement('li')
                var content = document.createTextNode(event.data)
                message.appendChild(content)
                messages.appendChild(message)
            };
            
            function sendMessage(event) {
                var input = document.getElementById("messageText")
                ws.send(input.value)
                input.value = ''
                event.preventDefault()
            }
        </script>
    </body>
</html>
"""


@app.get("/")
async def get():
    return HTMLResponse(html)
```

# h_pydantic.py
```python
import typing
from typing import List, Optional, Iterable, Sequence, Union
from uuid import UUID

from pydantic import BaseModel, Field, ValidationError, PyObject


# pydantic is primarily a parsing library, not a validation library.

# Model
class User(BaseModel):
    # Type: integer
    # Required
    id: int

    # Type: string (inferred)
    # Not required (has a default)
    name = 'Jane Doe'


class Device(BaseModel):
    # Reference to another model
    past_users: List[User]


# Create
user = User(id='123')
assert user.id == 123  # converted from string

# Input: helper functions

User.construct#()  # create a model without validation. 30x faster
User.construct#(_fields_set=[...])  # ... can specify which fields were set by the user

User.__init__#()  # create, using keyword arguments
User.parse_obj#({'id': 123})  # create, using one argument

User.parse_raw#('')  # load json
User.parse_raw#('', content_type='application/pickle', allow_pickle=True)  # load pickle

User.parse_file#('')  # loading files

User.from_orm#()  # load from arbitrary class


# Inspect
User.__fields__  # model's fields
User.__config__  # Configuration class for the model
assert user.__fields_set__ == {'id'}  # provided by the user

# JSON schema
User.schema()  # get JSON schema
User.schema_json()  # get JSON schema as a string

# Deep copy
user.copy()

# Output: dict
assert user.dict() == dict(user) == {'id': 123, 'name': 'Jane Doe'}

# Output: json
user.json()







# Validation
from pydantic import constr, conint, validator, root_validator
from pydantic import ValidationError
from pydantic.error_wrappers import ErrorWrapper

class Location(BaseModel):
    lat = 0.1
    lng = 10.1


class Model(BaseModel):
    is_required: float
    # int constraint: >= 42
    gt_int: conint(gt=42)
    list_of_ints: List[int] = None
    a_float: float = None
    recursive_model: Location = None

    foo: str

    # Custom validator
    @validator('foo')
    def name_must_contain_space(cls, v):
        # Use ValueError, TypeError, AssertionError
        # Subclass PydanticValueError to have a custom type
        if v != 'bar':
            raise ValueError('value must be "bar"')

        return v

    # Feature-complete validator
    @validator('foo')
    def multiple_requirements(cls, v, values: dict, config: object, field: Field):
        # v: the current value
        # values: all previously validated fields. Validation is done in order of definition. Failed fields not included.
        # config: the model config
        # field: the field being validated
        return v

    # NOTE: a validator can be applied to multiple fields
    @validator('foo', 'foo')
    def v1(cls, v): return v

    # For lists, it can be applied to every value
    @validator('foo', each_item=True)
    def v2(cls, v): return v

    # A validator can be applied to all fields, prior to other validators (e.g. preraparation)
    @validator('*', pre=True)
    def v3(cls, v): return v

    # Normally validators are not called when a value is not supplied.
    # Use always=True to run it always:
    @validator('foo', pre=True, always=True)
    def v4(cls, v): return v

    # A validator on the entire model
    @root_validator(pre=True)
    def check_model(cls, values: dict):
        assert 'card_number' not in values, 'card_number should not be included'
        return values


# pydantic will raise ValidationError
try:
    Model(
        list_of_ints=['1', 2, 'bad'],
        a_float='not a float',
        recursive_model={'lat': 4.2, 'lng': 'New York'},
        gt_int=21,
    )
except ValidationError as e:
    e.errors()  # list of errors
    e.json()  # list of errors in JSON
    str(e)  # human-redable errors

    error: ErrorWrapper = e.errors()[0]
    error['loc']  # path
    error['type']  # a computer-readable identifier of the error type.
    error['msg']  # a human readable explanation of the error.
    #error['ctx']  # (optional) values required to render the error message.








# Rename a field
class MyModel(BaseModel):
    # Rename a field
    metadata: typing.Dict[str, str] = Field(alias='metadata_')
















# Required/Optional
class Model(BaseModel):
    # A Required field: no default value, or default value = `...`
    a: int
    b: int = ...
    c: int = Field(...)

    # Optional fields
    d: Optional[int] = None
    e: int = 0

    # Required Optional
    # A field is required (has to be provided), but can be None
    f: Optional[int] = ...















# Generics
# Let's make a generic response type

from typing import TypeVar, Optional, Generic, Type, Tuple, Any
from pydantic.generics import GenericModel

DataT = TypeVar('DataT')


class Error(BaseModel):
    code: int
    message: str


class Response(GenericModel, Generic[DataT]):
    data: Optional[DataT]
    error: Optional[Error]

    # If the name of the concrete subclasses is important, you can also override the default behavior:
    @classmethod
    def __concrete_name__(cls: Type[Any], params: Tuple[Type[Any], ...]) -> str:
        return f'{params[0].__name__.title()}Response'


# Every specific model is cached, so there's no overhead

print(Response[int](data=1))
#> data=1 error=None
print(Response[str](data='value'))
#> data='value' error=None
print(Response[str](data='value').dict())
#> {'data': 'value', 'error': None}









# Parse into other types
from pydantic import parse_obj_as


class Item(BaseModel):
    id: int
    name: str


items = parse_obj_as(
    # Any type Pydantic can handle
    List[Item],
    # Input data
    [{'id': 1, 'name': 'My Item'}]
)








# Validation function arguments
from pydantic import validate_arguments, ValidationError

# Validates function arguments
# Argument types are inferred from type annotations on the function
# arguments without a type decorator are considered as Any
@validate_arguments
def repeat(s: str, count: int, *, separator: bytes = b'') -> bytes:
    pass









# SqlAlchemy interaction
from pydantic import constr

class CompanyModel(BaseModel):
    id: int
    public_key: constr(max_length=20)
    name: constr(max_length=63)
    domains: List[constr(max_length=255)]

    class Config:
        # Enable attribute access from objects
        orm_mode = True

company = {}  # load

# Convert to Pydantic
try:
    CompanyModel.from_orm(company)
except ValidationError:
    pass











# Dynamic models
# When the shape is not known until runtime

from pydantic import BaseModel, create_model

DynamicFoobarModel = create_model(
    # Name
    'DynamicFoobarModel',
    # Tuple(type, default value)
    # ... -> no default value, but can be None
    foo=(str, ...),
    # Default value
    bar=123,
)







# Immutable models
class FooBarModel(BaseModel):
    class Config:
        # Cannot modify once created
        # WARNING: Immutability in python is never strict.
        # If developers are determined/stupid they can always modify a so-called "immutable" object.
        allow_mutation = False






# __root__
# To validate an object without giving it a name

class Container(BaseModel):
    # The argument of parse_obj() is validated against the root type
    __root__: List[str]

print(Container.parse_obj(['a', 'b']).dict())
# -> {'__root__': ['a', 'b']}







# Using with ABCs
import abc

class FooBarModel(BaseModel, abc.ABC):
    a: str
    b: int

    @abc.abstractmethod
    def my_abstract_method(self):
        pass







# Complex types

# Generators
class Model(BaseModel):
    # Will be consumed on assignment
    finite: Sequence[int]
    # Won't be consumed: will remain a generator
    infinite: Iterable[int]


    # You can create a validator that consumes the first value using next()
    # and puts it back by using chain()


# Unions
class Model(BaseModel):
    # Pydantic will use the first type that works
    id: Union[UUID, int, str]

    # The type Optional[x] is a shorthand for Union[x, None].
    login: Optional[str]



# DateTime
#
# A datetime can be supplied as:
# * datetime
# * int/float/str UNIX epoch timestamp
# * str: ISO 8601: YYYY-MM-DD[T]HH:MM[:SS[.ffffff]][Z[±]HH[:]MM]]]
#
# date:
# * date
# * int/float/str
# * str: ISO8601: YYYY-MM-DD
#
# time:
# * time
# * str: ISO8601: HH:MM[:SS[.ffffff]]
#
# timedelta:
# * timedelta
# * int/float: seconds
# * str: ISO8601:
#       [-][DD ][HH:MM]SS[.ffffff]
#       [±]P[DD]DT[HH]H[MM]M[SS]S










# Forward references
# They just work.

from typing import ForwardRef
from pydantic import BaseModel

Foo = ForwardRef('Foo')

class Foo(BaseModel):
    a: int = 123
    b: Foo = None

# But in some cases, you'll have to update
Foo.update_forward_refs()














# Settings management

from pydantic import BaseSettings, RedisDsn, PostgresDsn

# Reads config from the environment

class Settings(BaseSettings):
    # AUTH_KEY=...
    auth_key: str

    # MY_API_KEY=...
    api_key: str = Field(..., env='my_api_key')

    # DB connection URLs
    redis_dsn: RedisDsn = 'redis://user:pass@localhost:6379/1'
    pg_dsn: PostgresDsn = 'postgres://user:pass@localhost:5432/foobar'

    # Python function reference
    special_function: PyObject = 'math.cos'

    class Config:
        # Default: not case sensitive
        case_sensitive = False

        # Environment variables prefix: app name
        env_prefix = 'my_prefix_'

        # Settings for individual fields
        fields = {
            'auth_key': {
                # Override env name
                'env': 'my_auth_key',
            },
            'redis_dsn': {
                # alternative env names
                'env': ['service_redis_dsn', 'redis_url']
            }
        }

        # Load from .env files
        # NOTE: env variables always taks priority!
        env_file = 'prod.env'  # filename
        env_file_encoding = 'utf-8'

# Load the settings
print(Settings(
    # Load from a .env file by name
    _env_file='prod.env', _env_file_encoding='utf-8'
).dict())
```

# tests.py
```python
from fastapi import FastAPI, WebSocket
from fastapi.params import Depends
from fastapi.testclient import TestClient

app = FastAPI()

# $ pip install pytest


@app.get("/")
async def read_main():
    return {"msg": "Hello World"}


client = TestClient(app)


# Test function: test_*()
def test_read_main():
    response = client.get("/")
    # Use normal assertions
    assert response.status_code == 200
    assert response.json() == {"msg": "Hello World"}







# Test websockets

@app.websocket_route("/ws")
async def websocket(websocket: WebSocket):
    await websocket.accept()
    await websocket.send_json({"msg": "Hello WebSocket"})
    await websocket.close()


def test_websocket():
    # Connect using `with`
    with client.websocket_connect("/ws") as websocket:
        # Receive
        data = websocket.receive_json()
        assert data == {"msg": "Hello WebSocket"}






# Test events

@app.on_event("startup")
async def startup_event():
    app.extra['startup'] = True


def test_read_items():
    with TestClient(app) as client:
        assert app.extra['startup'] == True









# Test: override dependencies

def original_dependency():
    raise NotImplementedError

def overridden_dependency():
    pass

@app.get('/dependency')
def dependency(dep=Depends(original_dependency)):
    return {'ok': 1}

app.dependency_overrides[original_dependency] = overridden_dependency


def test_dependency():
    res = client.get('/dependency').json()   # no error
    assert res['ok'] == 1


```

# pytest primer

## Command-Line

Show all available *fixtures* (injectable dependencies):

    $ pytest --fixtures

Stop on first error:

    $ pytest -x
    
show local variables:

    $ pytest -l
    
drop to pdb for first 3 failures, use `breakpoint()` in the code, or:

    $ pytest --pdb --maxfail=3
    
show 10 slowest tests:

    $ pytest --durations=10
    
do not capture stdout:

    $ pytest -s

Re-run failures:

* `--lf` - to only re-run the failures.
* `--ff` - to run the failures first and then the rest of the tests.






















## Python tests


### Tests

* Any file named `test_*.py`
* Any class named `Test*` (optional; don't have to use classes)
* Any function named `test_*()`









### Assertions

Simply use `assert`, and pytest will provide nice detailed output:

```python
import pytest

def test_function():
    assert f() == 4

    with pytest.raises(ValueError):
        5/0
```












### Fixtures

Fixtures are automatically provided to tests. Recognized by names.

```python
# conftest.py
# Shared fixture functions

# Use for services, sharing data, etc

import pytest

# scope:
# 'function': invoke once per test function (default)
# 'module': invoke once per module
# 'session': invoked once per test run
# callable: function(fixture_name, config) that determines the scope on the fly
#
# autouse=True: enable for every test that sees it
@pytest.fixture(scope="module")
def some_connection():
    with connect() as connection:
        # yield it, clean-up afterwards
        yield connection


# Alternative teardown
@pytest.fixture(scope="module")
def some_connection(request: FixtureRequest):  # can use fixtures themselves: DI
    ...

    def fin():
        ...

    request.addfinalizer(fin)
    return ...

# A fixture can yield a factory function with parameters that tests can use.yiel
@pytest.fixture(scope="module")
def some_connection(request: FixtureRequest):
    def factory(param):
        ...
    
    yield factory

    ... # clean-up

```

A parameterized fixture. 
Will cause every test using it to run twice:

```python
@pytest.fixture(scope="module", 
                # Run every test twice, for every parameter
                params=["smtp.gmail.com", "mail.python.org"],
                # Nicer, human-readable names (in case you used magic numbers for parameters)
                ids=['gmail', 'python']
)
def some_connection(request):
    connection = connect(request.param)  # 1) gmail, 2) python
    yield connection

# Parameters can be marked, e.g. with tags, or for skipping:
@pytest.fixture(..., params=[
    ...,
    pytest.param(2, marks=pytest.mark.skip)
])
def some_connection(request): pass
```

Use fixtures without arguments:

```python
# For one test
@pytest.mark.usefixtures("cleandir", "anotherfixture")
def test(): ...

# For the whole module
pytestmark = pytest.mark.usefixtures("cleandir")
```

Auto-using fixtures for the whole class:

```python
@pytest.fixture(scope="module")
def db():
    return DB()  # DB connection


# Notice how this class is used as a ... scope of sorts
class TestClass:
    # This fixture will begin&rollback a transaction for the whole class
    # autouse: will be invoked for all tests in view
    @pytest.fixture(autouse=True)
    def transact(self, request, db):
        db.begin(request.function.__name__)
        yield
        db.rollback()

    def test_method1(self, db):
        assert db.intransaction == ["test_method1"]

    def test_method2(self, db):
        assert db.intransaction == ["test_method2"]
```

To override a fixture, just use a function with the same name.
The `super()` fixture will be provided as an argument:

```python
@pytest.fixture()
def db(db):
    ...
```

Override a fixture with a fixed value:

```python
# Feed a constant value
@pytest.mark.parametrize('username', ['directly-overridden-username'])
def test_username(username):
    assert username == 'directly-overridden-username'
```











### Attributes

```python
import pytest

pytest.skip # - always skip a test function
pytest.skipif # - skip a test function if a certain condition is met
pytest.xfail # - produce an “expected failure” outcome if a certain condition is met
pytest.parametrize # - to perform multiple calls to the same test function.
```

example:

```python
# Skip a failing test
@pytest.mark.skip(reason="no way of currently testing this")
def failing_test(): 
    pytest.skip("unsupported configuration")  # another way

# Conditional skip
@pytest.mark.skipif(sys.version_info < (3, 6), reason="requires python3.6 or higher")
def test_function():
    ...

# Give up; not too good, but not fatal either
@pytest.mark.xfail(raises=RuntimeError)  # expected error
def test_function2():
    import slow_module
    if slow_module.slow_function():
        pytest.xfail("slow_module taking too long")  # another way
```

expected failures can be used with parameters:

```python
@pytest.mark.parametrize(
    ("n", "expected"),
    [
        (1, 2),
        # known bug
        pytest.param(1, 3, marks=pytest.mark.xfail(reason="some bug")),
        # known not working 
        pytest.param(
            10, 11, marks=pytest.mark.skipif(sys.version_info >= (3, 0), reason="py2k")
        ),
    ],
)
def test_increment(n, expected):
    assert n + 1 == expected
```









### Monkeypatching other objects

Use the `monkeypatch` fixture.
All modifications will be undone after the requesting test function or fixture has finished. 

```python
monkeypatch: MonkeyPatch
monkeypatch.setattr(obj, name, value, raising=True)
monkeypatch.delattr(obj, name, raising=True)
monkeypatch.setitem(mapping, name, value)
monkeypatch.delitem(obj, name, raising=True)
monkeypatch.setenv(name, value, prepend=False)
monkeypatch.delenv(name, raising=True)
monkeypatch.syspath_prepend(path)
monkeypatch.chdir(path)
```

Can it as a context manager:

```python
with monkeypatch.context() as m:
    m.setattr(...)
m.undo()  # undo all changes. ALL changes.
```














### Parameterizing

Run the function multiple times, with different inputs:

```python
@pytest.mark.parametrize(
    ('input', 'expeced'), 
    [("3+5", 8), ("2+4", 6), ("6*9", 42)])
def test_eval(input, expected):
    assert eval(input) == expected
```
