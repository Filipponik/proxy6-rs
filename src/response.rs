use crate::value_object::{Currency, ResponseStatus, UserBalance, UserId};

struct SuccessResponse {
    status: ResponseStatus,
    user_id: UserId,
    balance: UserBalance,
    currency: Currency,
}
