DROP TABLE IF EXISTS busers CASCADE;
CREATE TABLE busers (
    id bigserial,
    login varchar NOT NULL,
    PRIMARY KEY(id)
);

DROP TABLE IF EXISTS barticles CASCADE;
CREATE TABLE barticles (
    id bigserial,
    author_id bigint,
    title text,
    body text,
    PRIMARY KEY(id),
    CONSTRAINT fk_author FOREIGN KEY (author_id) REFERENCES busers(id)
);

DROP TABLE IF EXISTS btags CASCADE;
CREATE TABLE btags (
    id bigserial,
    name varchar,
    PRIMARY KEY(id)
);

DROP TABLE IF EXISTS barticle_tags CASCADE;
CREATE TABLE barticle_tags (
    article_id bigint,
    tag_id bigint,
    PRIMARY KEY(article_id, tag_id),
    CONSTRAINT fk_article_id FOREIGN KEY (article_id) REFERENCES barticles(id),
    CONSTRAINT fk_tag_id FOREIGN KEY (tag_id) REFERENCES btags(id)
);

INSERT INTO busers (id, login) VALUES 
    (1, 'A'),
    (2, 'B'),
    (3, 'C')
;
INSERT INTO barticles (author_id, title) VALUES 
    (1, 'First'),
    (1, 'Second'),
    (1, 'Third'),
    (2, 'Red'),
    (2, 'Green'),
    (3, 'Blah')
;

INSERT INTO btags (id, name) VALUES
    (1, '#a'),
    (2, '#b')
;

INSERT INTO barticle_tags (article_id, tag_id) VALUES 
    (1, 1),
    (1, 2),
    (2, 1),
    (3, 2)
;