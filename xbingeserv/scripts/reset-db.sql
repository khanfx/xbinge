DROP TABLE IF EXISTS subscriptions;
DROP TABLE IF EXISTS watchers;
DROP TABLE IF EXISTS episodes;
DROP TABLE IF EXISTS schedules;

CREATE TABLE watchers (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL
);

CREATE TABLE schedules (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL
);

CREATE TABLE subscriptions (
    id SERIAL PRIMARY KEY,
    watcher_id TEXT NOT NULL REFERENCES watchers(id) ON DELETE CASCADE,
    schedule_id TEXT NOT NULL REFERENCES schedules(id) ON DELETE CASCADE
);

CREATE TABLE episodes (
    id TEXT PRIMARY KEY,
    schedule_id TEXT NOT NULL REFERENCES schedules(id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    watch_date DATE NOT NULL
);




-- ----------------------------------------------------
-- SEED DATA
-- ----------------------------------------------------

INSERT INTO watchers VALUES ('johndoe', 'johndoe');
INSERT INTO schedules VALUES ('expanse', 'expanse');
INSERT INTO subscriptions VALUES (-1, 'johndoe', 'expanse');
INSERT INTO episodes VALUES ('bf46d7a1fe17a0a9a67b9f1c1e5806e3', 'expanse', 's1e1', '2025-01-01');
INSERT INTO episodes VALUES ('3c0addaa2649bc39e9bb60729f6c2cca', 'expanse', 's1e2', '2025-01-10');




-- ----------------------------------------------------
-- QUERIES
-- ----------------------------------------------------

select 'upcoming episodes', w.id, e.name, e.watch_date
from watchers w
left join subscriptions ss
    on w.id=ss.watcher_id
left join schedules s
    on ss.schedule_id=s.id
left join episodes e
    on s.id=e.schedule_id;



