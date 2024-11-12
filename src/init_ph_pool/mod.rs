pub mod init_can;

pub trait Ph {
    type Output;
    fn init() -> Option<Self::Output>;
}
