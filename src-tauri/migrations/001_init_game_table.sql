create table if not exists games
(
    id         integer primary key autoincrement,
    title      text not null,
    created_at timestamp default current_timestamp
);