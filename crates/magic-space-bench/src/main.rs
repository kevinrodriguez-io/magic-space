use magic_space::*;

#[derive(MagicSpace)]
#[allow(dead_code)]
struct Basket {
    apples: u32,
    oranges: u32,
    #[max_len(10)]
    others: Vec<u32>,
    #[max_len(10)]
    name: String,
    #[max_len(10, 5, 5)]
    some_other: Vec<Vec<Vec<u8>>>,
    #[max_len(10)]
    e: Vec<Option<SomeEnum>>,
}

#[derive(MagicSpace)]
pub enum SomeEnum {
    A(u32),
    B(u32, u32),
    C(#[max_len(10)] Vec<u32>),
}

#[derive(MagicSpace)]
pub struct DynamicSizeVecPattern {
    #[max_len(0)]
    pub data: Vec<Item>,
}

impl DynamicSizeVecPattern {
    pub fn size(vec_len: usize, str_len: usize) -> usize {
        Self::MAGIC_SPACE + (vec_len * Item::size(str_len))
    }
}

#[derive(MagicSpace)]
pub struct Item {
    pub data: u64,
    #[max_len(0)]
    pub name: String,
}

impl Item {
    pub fn size(str_len: usize) -> usize {
        Self::MAGIC_SPACE + str_len
    }
}

fn main() {
    println!("Basket = {:?}", Basket::MAGIC_SPACE);
    println!("SomeEnum = {:?}", SomeEnum::MAGIC_SPACE);
    println!(
        "DynamicSizeVecPattern = {:?}",
        DynamicSizeVecPattern::MAGIC_SPACE
    );
    println!(
        "DynamicSizeVecPattern with 10 items = {:?}",
        DynamicSizeVecPattern::size(10, 32)
    );
}
