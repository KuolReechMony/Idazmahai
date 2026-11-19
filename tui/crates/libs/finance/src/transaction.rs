use primitives::models::{DateTimeSelection, NonEmptyString};
use rust_decimal::Decimal;

pub(crate) struct Expense;
pub(crate) struct Income;
pub(crate) struct Transfer;
pub(crate) struct ReverseTransaction;

#[derive(Debug)]
pub(crate) struct FinanceTransaction<Type> {
    id: i64,
    account_id: i64,
    budget_id: i64,
    transaction_title: NonEmptyString,
    transaction_description: Option<NonEmptyString>,
    transaction_amount: Decimal,
    transaction_type: Type,
    timestamp: DateTimeSelection,
}
