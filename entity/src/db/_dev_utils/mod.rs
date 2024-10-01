use crate::db::ModelManager;
use tokio::sync::OnceCell;

// endregion: --- Modules

/// Initialize test environment.
pub async fn init_test() -> ModelManager {
    static INIT: OnceCell<ModelManager> = OnceCell::const_new();
    let mm = INIT
        .get_or_init(|| async {
            // NOTE: Rare occasion where unwrap is kind of ok.
            ModelManager::new_test("postgres://dev:devpassword@localhost/testdb".to_string())
                .await
                .unwrap()
        })
        .await;

    mm.clone()
}
