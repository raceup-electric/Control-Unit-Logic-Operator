pub mod init_can;

pub trait Ph {
    fn init(&mut self) -> Option<()>;
}
