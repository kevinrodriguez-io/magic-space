pub use magic_space_derive::MagicSpace;

pub trait Space {
    const MAGIC_SPACE: usize;
}

pub mod __private {
    pub const fn max(a: usize, b: usize) -> usize {
        [a, b][(a < b) as usize]
    }
}
