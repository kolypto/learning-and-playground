import sqlalchemy as sa
import sqlalchemy.orm
import sqlalchemy.ext.asyncio
import sqlalchemy.future


engine: sqlalchemy.ext.asyncio.AsyncEngine = sa.ext.asyncio.create_async_engine(
    'postgresql+asyncpg://postgres:postgres@localhost/test',
    # echo=True,
)

AsyncSession = sa.orm.sessionmaker(
    engine, 
    # Do not expire on commit. To prevent lazy loads!
    expire_on_commit=False, 
    class_=sa.ext.asyncio.AsyncSession,
)


async def main():
    # CREATE tables
    conn: sa.ext.asyncio.engine.AsyncConnection
    async with engine.begin() as conn:
        # Use run_sync() to call DDL functions that don't include an awaitable hook
        await conn.run_sync(Base.metadata.drop_all)
        await conn.run_sync(Base.metadata.create_all)
    
        await conn.execute(
            User.__table__.insert(),
            [
                {'login': 'a'},
                {'login': 'b'},
            ],
        )
    
    # SELECT some users
    async with engine.connect() as conn:
        res = await conn.execute(
            sa.select([User.login]).select_from(User)
        )
        print(res.fetchall())
    
    # SELECT, using async stream (server-side cursors)
    async with engine.connect() as conn:
        res = await conn.stream(
            sa.select([User.login]).select_from(User)
        )

        async for row in res:
            print(row)

    # SELECT using sync-style ORM code
    def sync_code(session: sa.orm.Session):
        pass 
        # ssn.query(...)
        # ssn.add(...)

    async with AsyncSession() as session:
        await session.run_sync(sync_code)
    
    # AsyncSession: full ORM functionality
    async with AsyncSession() as session:
        # Load {login: User} mapping
        res = await session.stream(
            sa.future.select(User).options(
                sa.orm.selectinload(User.articles)
            )
        )

        users = {
            user.login: user
            async for user, in res
        }

        # Insert articles
        session.add_all([
            Article(user=users['a'], title='first'),
            Article(user=users['a'], title='second'),
        ])

        # Done
        await session.commit()



Base = sa.orm.declarative_base()

class User(Base):
    __tablename__ = 'users'

    id = sa.Column(sa.Integer, primary_key=True)
    login = sa.Column(sa.String)


class Article(Base):
    __tablename__ = 'articles'

    id = sa.Column(sa.Integer, primary_key=True)
    title = sa.Column(sa.String)

    user_id = sa.Column(sa.ForeignKey(User.id))
    user = sa.orm.relationship(User, backref='articles')

    # Required to load SQL DEFAULTs after INSERT. This prevents expired attributes.
    __mapper_args__ = {"eager_defaults": True}



import asyncio 
asyncio.run(main())
