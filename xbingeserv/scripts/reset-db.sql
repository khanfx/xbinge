DROP TABLE IF EXISTS items;

CREATE TABLE items (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL
);

select * from items;

-- ----------------------------------------------------

DROP TABLE IF EXISTS watchers;
CREATE TABLE watchers (
    id SERIAL PRIMARY KEY,
    uuid TEXT NOT NULL
    name TEXT NOT NULL
);

DROP TABLE IF EXISTS schedules;
CREATE TABLE schedules (
    id SERIAL PRIMARY KEY,
    uuid TEXT NOT NULL,
    name TEXT NOT NULL
);

DROP TABLE IF EXISTS subscriptions;
CREATE TABLE subscriptions (
    id SERIAL PRIMARY KEY,
    watcher_id int,
    schedule_id int
);

DROP TABLE IF EXISTS episodes;
CREATE TABLE episodes (
    id SERIAL PRIMARY KEY,
    uuid TEXT NOT NULL,
    name TEXT NOT NULL,
    watch_date DATETIME NOT NULL
);
