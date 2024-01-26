mod binary;
mod linearity;
mod max;
mod min;

pub use self::binary::search as binary_search;
pub use self::linearity::search as linearity_search;
pub use self::max::max as max_search;
pub use self::min::min as min_search;
