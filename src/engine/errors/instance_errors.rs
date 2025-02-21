use thiserror::Error;

#[derive(Error, Debug)]
pub enum InstanceCreationError {
    #[error("No vulkan library found at '{path:?}, original error: {msg:?}'")]
    EntryInvalidLocation {
        path: String,
        msg: String,
    },

    #[error("Failed the validation check")]
    ValidationCheck
}
