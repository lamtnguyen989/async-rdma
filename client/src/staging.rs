
#[derive(Clone, Copy, Debug, PartialEq, Eq, clap::ValueEnum)]
pub enum IoMode {
    Auto,
    Uring,
    Pread,
}