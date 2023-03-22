
use diesel::prelude::*;


#[derive(Queryable)]
#[diesel(belongs_to(Validator))]
#[diesel(belongs_to(Operator))]
struct ValidatorOperator {
    validator_public_key: str,
    operator_id: u32,
}