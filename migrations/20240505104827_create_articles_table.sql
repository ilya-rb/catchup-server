CREATE TABLE articles
(
    id          uuid        NOT NULL,
    PRIMARY KEY (id),
    link        TEXT        NOT NULL,
    title       TEXT        NOT NULL,
    description TEXT,
    tags        TEXT[]      NOT NULL,
    source      TEXT        NOT NULL,
    created_at  timestamptz NOT NULL,

    constraint check_title_not_empty check (length(title) > 0),
    constraint check_link_not_empty check (length(link) > 0)
);