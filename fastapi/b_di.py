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

