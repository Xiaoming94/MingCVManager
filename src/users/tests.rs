use googletest::prelude::*;
use std::result::Result;

use super::*;

type TestResult<T> = googletest::Result<T>;

#[test]
fn users_are_created_with_username_and_id() -> TestResult<()> {
    let uuid = Uuid::from_u128(0x1337);
    let username = "username";
    let user = User::new(uuid, username);

    verify_that!(
        user,
        matches_pattern!(User {
            id: eq(&uuid),
            username: username,
        })
    )
}
