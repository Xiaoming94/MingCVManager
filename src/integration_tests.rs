/**
 * AUTHOR: Xiaoming
 *
 * File with all the end-2-end integration tests.
 *
 * Note that these are guarded by the --features integration flag
 * So to execute these, you need to use `cargo test --features integration`
 *
 * Beware though, these require that you have running postgresql instance running
 * so it's recommended that you either set that up yourself
 * or just deploy the docker stack that is described in docker-compose.yml
 */
use googletest::prelude::*;

use crate::create_app;
use axum_test::TestServer;
use serde_json::json;

#[tokio::test]
async fn test_server_can_be_started() {
    let server = TestServer::new(create_app());

    let response = server.get("/").await;
    response.assert_status_ok();
}

struct BackendAppCanBeStarted {
    app: TestServer,
}

impl ConsumableFixture for BackendAppCanBeStarted {
    fn set_up() -> Result<Self> {
        Ok(BackendAppCanBeStarted {
            app: TestServer::new(create_app()),
        })
    }
}

const API_HEALTH_CHECK_URL: &str = "/api/v1.0/healthcheck";

#[tokio::test]
#[gtest]
async fn verify_api_client_can_healthcheck_server(fixture: BackendAppCanBeStarted) {
    let response = fixture.app.get(API_HEALTH_CHECK_URL).await;
    response.assert_status_ok();
}

#[tokio::test]
#[gtest]
async fn verify_api_client_can_check_connectivity(fixture: BackendAppCanBeStarted) {
    let response = fixture.app.get(API_HEALTH_CHECK_URL).await;
    response.assert_json_contains(&json!({
        "status": "connected",
    }));
}
