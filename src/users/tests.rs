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

impl UserIsCreated {
    fn new() -> Self {
        UserIsCreated {
            user: User::new(USER_ID, USERNAME),
        }
    }

    fn relinquish_user(self) -> User {
        self.user
    }
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
    use super::*;
    use std::collections::HashMap;
    use std::sync::Mutex;

    struct StandardRepo {
        db: Mutex<HashMap<Uuid, User>>,
    }

    impl StandardRepo {
        fn new() -> Self {
            StandardRepo {
                db: Mutex::new(HashMap::new()),
            }
        }
    }

    impl UserRepository for StandardRepo {
        async fn insert(&self, user: User) -> Result<User, UserRepositoryError> {
            let mut db = self.db.lock().unwrap();
            return if db.contains_key(&user.id) {
                Err(UserRepositoryError::UserExist(user.id))
            } else {
                db.insert(user.id, user.clone());
                Ok(user)
            };
        }

        async fn find_by_id(&self, id: Uuid) -> Option<User> {
            None
        }
    }

    #[gtest]
    #[tokio::test]
    async fn user_can_be_inserted_into_repository() -> TestResult<()> {
        let fixture = UserIsCreated::new();
        let repo = StandardRepo::new();

        verify_that!(
            repo.insert(fixture.relinquish_user()).await,
            ok(matches_pattern!(User {
                id: eq(&USER_ID),
                username: USERNAME,
            }))
        )
    }

    struct RepoExistWithAUser {
        repo: StandardRepo,
        existing_user: User,
    }

    impl RepoExistWithAUser {
        async fn new() -> Self {
            let repo = StandardRepo::new();
            let user = UserIsCreated::new().relinquish_user();

            repo.insert(user.clone())
                .await
                .expect("Fixture setup: first insert should succeed");

            RepoExistWithAUser {
                repo,
                existing_user: user,
            }
        }
    }

    #[gtest]
    #[tokio::test]
    async fn cannot_insert_user_if_uuid_exists() -> TestResult<()> {
        let fixture = RepoExistWithAUser::new().await;
        let user = fixture.existing_user.clone();
        let user_uuid = user.id;

        verify_that!(
            fixture.repo.insert(user).await,
            err(matches_pattern!(UserRepositoryError::UserExist(eq(
                &user_uuid
            ))))
        )
    }
}
