pub mod normal;
pub mod dual;

pub mod provider {
    mod add;
    pub use add::Add;

    mod max;
    pub use max::Max;

    mod min;

    mod assign;
    pub use assign::Assign;

    mod assign_or;
    pub use assign_or::{AssignOr, AssignOrProvider};

    mod affine;
    pub use affine::Affine;

    pub mod legacy;
}

pub mod traits;
