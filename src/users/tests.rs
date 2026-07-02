use googletest::prelude::*;
use googletest::{Result, verify_that};

use super::*;

#[test]
fn users_are_created_with_username_and_id() -> Result<()> {
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
