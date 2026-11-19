use std::marker::PhantomData;

use primitives::models::NonEmptyString;
use rust_decimal::Decimal;

pub(crate) struct Checking;
pub(crate) struct Savings;
pub(crate) struct Hysa; // High yield savings account
pub(crate) struct Hsa; // Health savings account
pub(crate) struct Credit;
pub(crate) struct CD;

#[derive(Debug)]
pub(crate) struct FinanceAccount<Type> {
    id: i64,
    finance_institution_id: i64,
    account_title: NonEmptyString,
    interest_rate: Decimal,
    account_type: PhantomData<Type>,
}

// Methods
// A. REGULAR
//
// 1. get_current_account_balance()
//
// B. HISTORY
//
// 1. get_interest_rates()
// 2. get_account_balances()
// 3. get_incomes()
// 4. get_expenses()
// 5. get_transfers()
// 6.
