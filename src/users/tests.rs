use super::*;

use googletest::prelude::*;

#[test]
fn user_can_be_created_with_username_and_personal_name_and_password() -> Result<()> {
    let username = "my_username";
    let password = "P4ssword";
    let personal_name = "My Real Name";
    let my_user = User::new(
        username.to_string(),
        personal_name.to_string(),
        password.to_string(),
    );

    verify_that!(
        my_user,
        matches_pattern!(User {
            username: eq(username),
            personal_name: contains_substring(personal_name),
            password: eq(password),
            ..
        })
    )
}

struct ValidUserExists {
    valid_user: User,
}

impl Fixture for ValidUserExists {
    fn set_up() -> Result<Self> {
        let username = "my_username";
        let password = "P4ssword";
        let personal_name = "My Real Name";
        let my_user = User::new(
            username.to_string(),
            personal_name.to_string(),
            password.to_string(),
        );

        Ok(ValidUserExists {
            valid_user: my_user,
        })
    }

    fn tear_down(self) -> Result<()> {
        Ok(())
    }
}

#[gtest]
fn verify_user_has_a_profile_that_is_empty_when_created(
    valid_user_fixture: &ValidUserExists,
) -> Result<()> {
    verify_that!(valid_user_fixture.valid_user.profile, eq(""))
}

#[gtest]
fn verify_user_can_change_their_profile_text(valid_user_fixture: &ValidUserExists) -> Result<()> {
    let mut local_user = valid_user_fixture.valid_user.clone();

    let profile_text = "New user profile";
    local_user.profile = String::from(profile_text);
    verify_that!(local_user.profile, not(eq("")))?;
    verify_that!(local_user.profile, contains_substring(profile_text))
}
