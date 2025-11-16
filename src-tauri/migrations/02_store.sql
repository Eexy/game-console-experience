create table if not exists stores
(
    id         integer primary key autoincrement,
    name       text not null,
    created_at timestamp default current_timestamp
);

insert into stores(name)
values ('steam');

create table if not exists games_new
(
    id         integer primary key autoincrement,
    title      text not null,
    store_id   integer,
    created_at timestamp default current_timestamp,
    foreign key (store_id) references stores (id)
);

insert into games_new(title)
select title
from games;

drop table games;

alter table games_new
    rename to games;
