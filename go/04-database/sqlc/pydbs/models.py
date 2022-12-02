# Code generated by sqlc. DO NOT EDIT.
# versions:
#   sqlc v1.16.0
import pydantic
from typing import Optional


class Article(pydantic.BaseModel):
    id: int
    author_id: int
    title: str
    body: Optional[str]


class User(pydantic.BaseModel):
    id: int
    login: str
    age: Optional[int]
