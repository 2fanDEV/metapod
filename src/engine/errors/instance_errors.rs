use thiserror::Error;

#[derive(Error, Debug)]
pub enum EntryLoadingError {
    #[error("No vulkan library found at '{path:?}, original error: {msg:?}'")]
    InvalidLocation {
        path: String,
        msg: String,
    }
}


#[derive(Error, Debug)]
pub enum InstanceCreationError {

}
