use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {

    #[error("value should be more than -1/e")]
    Invalidinput,     

    #[error("value should be less than 0")]
    ZeroValue,

}


