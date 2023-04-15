-- Your SQL goes here
create table cookies (
    id serial primary key,
    name varchar not null,
    image_path varchar not null
)