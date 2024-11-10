pub mod init_can;

#[derive(Debug)]
pub enum PhStatus{
    NonInitialized,
    Initialized,
    Error,
}

pub trait Ph {
    type InitializedPh;

    fn init(&mut self) -> Self::InitializedPh;
}
