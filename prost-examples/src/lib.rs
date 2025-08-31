mod card;
mod generated {
    pub mod example {
        include!("generated/example.rs");
    }
}

pub use card::*;
pub use generated::*;
