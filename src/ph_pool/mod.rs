pub mod init_can;

pub trait Ph {
    type InitializedPh;

    fn init(&mut self) -> Option<Self::InitializedPh>;
}
