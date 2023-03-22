create table posts
(
    id        int auto_increment,
    title     varchar(30)        not null,
    body      text               not null,
    published bool default false not null,
    constraint posts_pk
        primary key (id)
);
