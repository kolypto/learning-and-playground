CREATE TABLE users (
    id bigserial,
    login varchar NOT NULL,
    age int,
    PRIMARY KEY(id)
);

CREATE TABLE articles (
    id bigserial,
    author_id bigint NOT NULL REFERENCES users(id),
    title varchar NOT NULL,
    body text,
    PRIMARY KEY(id)
);
