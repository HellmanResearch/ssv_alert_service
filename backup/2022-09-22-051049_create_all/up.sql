create table operator
(
    id                 int unsigned not null,
    name               varchar(50) not null,
    account_public_key varchar(42) not null,
    status             varchar(10) not null,
    validator_count    int unsigned not null,
    fee_human          float unsigned null,
    constraint operator_account
        foreign key (account_public_key) references account (public_key)
            on delete cascade
);

create unique index operator_name_uindex
    on operator (name);

