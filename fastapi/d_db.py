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
