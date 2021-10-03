CREATE TABLE IF NOT EXISTS "bookmarks" (
    "id" SERIAL NOT NULL PRIMARY KEY,
    "title" TEXT NOT NULL,
    "url" TEXT NULL,
    "description" TEXT NULL,
    "created_at" TIMESTAMPTZ NOT NULL,
    "updated_at" TIMESTAMPTZ NOT NULL
);

CREATE TABLE IF NOT EXISTS "tags" (
    "tag" VARCHAR(256) NOT NULL PRIMARY KEY,
    "created_at" TIMESTAMPTZ NOT NULL
);

CREATE TABLE IF NOT EXISTS "bookmarks_tags" (
    "bookmark_id" BIGINT NOT NULL REFERENCES "bookmarks" ("id"),
    "tag" BIGINT NOT NULL REFERENCES "tags" ("tag"),
    PRIMARY KEY ("bookmark_id", "tag")
);
