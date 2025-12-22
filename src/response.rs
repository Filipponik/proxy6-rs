pub struct ResponseStatus(String);
pub struct UserId(String);
pub struct UserBalance(String);
pub struct Currency(String);

struct SuccessResponse {
    status: ResponseStatus,
    user_id: UserId,
    balance: UserBalance,
    currency: Currency,
}
