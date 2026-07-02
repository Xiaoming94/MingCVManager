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

struct UserIsCreated {
    user: User,
}

const USER_ID: Uuid = Uuid::from_u128(0x1337);
const USERNAME: &str = "username";

impl Fixture for UserIsCreated {
    fn set_up() -> TestResult<Self> {
        Ok(UserIsCreated {
            user: User::new(USER_ID, USERNAME),
        })
    }

    fn tear_down(self) -> TestResult<()> {
        Ok(())
    }
}

/**
 * Repository
 * =====
 *
 * Module containing test for the basic repository functionalities.
 * Also contains a blanket implementation of a UserRepository that
 * can be used for testing in other places where needed.
 */
pub(crate) mod repository {
    use std::collections::HashMap;

    use super::*;

    struct StandardImpl {
        db: HashMap<Uuid, User>,
    }

    impl UserRepository for StandardImpl {
        async fn insert(&self, user: User) -> Result<User, UserRepositoryError> {
            Ok(user)
        }
        async fn find_by_id(&self, id: Uuid) -> Option<User> {
            None
        }
    }

    #[gtest]
    fn user_can_be_inserted_into_repository(fixture: &UserIsCreated) -> TestResult<()> {
        Ok(())
    }
}
