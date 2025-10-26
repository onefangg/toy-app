create schema if not exists app;
create extension pgcrypto;

create table if not exists app.users (
    id uuid PRIMARY KEY default gen_random_uuid(),
    username text not null,
    password varchar,
    created_dt TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    constraint unique_username unique (username)
);

insert into app.users (username, password) values
  (  'pseudopoo',crypt('securepassword', gen_salt('md5'))),
  (  'oposeudo', crypt('securepassword2', gen_salt('md5')));

