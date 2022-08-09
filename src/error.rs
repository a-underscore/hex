#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Failed to downcast reference")]
    DowncastRefFailed,
}
