// @generated automatically by Diesel CLI.

diesel::table! {
    account (public_key) {
        public_key -> Varchar,
        ssv_balance_human -> Float,
        is_liquidation -> Bool,
    }
}

diesel::table! {
    decided (id) {
        id -> Bigint,
        role -> Varchar,
        validator_public_key -> Varchar,
        signature -> Text,
        height -> Unsigned<Integer>,
        round -> Unsigned<Integer>,
        identifier -> Varchar,
        message_type -> Unsigned<Integer>,
        timestamp -> Unsigned<Bigint>,
    }
}

diesel::table! {
    operator (id) {
        id -> Unsigned<Integer>,
        name -> Varchar,
        account_public_key -> Varchar,
        status -> Varchar,
        validator_count -> Unsigned<Integer>,
        fee_human -> Float,
        performance_1day -> Float,
        performance_1month -> Float,
    }
}

diesel::table! {
    operator_decided_record (id) {
        id -> Bigint,
        operator_id -> Unsigned<Integer>,
        validator_public_key -> Varchar,
        height -> Unsigned<Integer>,
        round -> Unsigned<Integer>,
        timestamp -> Unsigned<Bigint>,
    }
}

diesel::table! {
    posts (id) {
        id -> Integer,
        title -> Varchar,
        body -> Text,
        published -> Bool,
    }
}

diesel::table! {
    validator (public_key) {
        account_public_key -> Varchar,
        public_key -> Varchar,
    }
}

diesel::table! {
    validator_operator (id) {
        id -> Integer,
        validator_public_key -> Varchar,
        operator_id -> Unsigned<Integer>,
    }
}


diesel::table! {
    performance_record (id) {
        id -> Unsigned<Integer>,
        operator_id -> Unsigned<Integer>,
        performance -> Float,
        timestamp -> Unsigned<Bigint>,
    }
}

diesel::table! {
    tag (key) {
        key -> Varchar,
        value -> Text,
    }
}


diesel::joinable!(operator -> account (account_public_key));

diesel::allow_tables_to_appear_in_same_query!(
    account,
    decided,
    operator,
    operator_decided_record,
    posts,
    validator,
    validator_operator,
    performance_record,
    tag,
);
