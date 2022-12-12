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
