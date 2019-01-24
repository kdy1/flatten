#![feature(optin_builtin_traits, on_unimplemented, never_type, specialization)]
#![cfg_attr(not(feature = "std"), no_std)]

extern crate either;

pub use into_cons::NotTuple;
use into_cons::{ConsOf, IntoCons};
use tuple::{IntoTuple, TupleOf};

#[macro_use]
pub mod cons;
pub mod into_cons;
pub mod tuple;

pub trait Flatten: Sized + IntoCons {
    /// Type of tuple after flattening.
    type Flattened;
    ///
    ///
    ///# Examples
    ///
    ///```rust
    /// use flatten::Flatten;
    /// "".flatten();
    /// assert_eq!((1, 2, 3, 4), (1, (2, 3), 4).flatten());
    /// assert_eq!((1, 2, 3, 4), (1, (2, (3,)), ((4,),)).flatten());
    /// ```
    fn flatten(self) -> Self::Flattened;
}
impl<Tup> Flatten for Tup
where
    ConsOf<Self>: IntoTuple<Self>,
{
    type Flattened = TupleOf<ConsOf<Self>, Self>;
    #[inline(always)]
    fn flatten(self) -> Self::Flattened {
        self.into_cons().into_tuple()
    }
}

/// Type of tuple after flattening.
pub type Flattened<T> = <T as Flatten>::Flattened;
