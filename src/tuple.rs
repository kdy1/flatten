#![doc(hidden)]

//! Convert `Cons` into tuples.

use cons::{Cons, Nil, ValidHead, ValidNode};

///
///```rust,compile_fail
///extern crate flatten;
///use flatten::Flatten;
///
///# fn main() {
///let too_big = (1, 2, 3, (4, 5), 6, (7, 8), (9, 10), (11, 12, 13, 14), 15);
///too_big.flatten();//~ ERROR length of result tuple should be <= 12
///
///# }
///```
#[rustc_on_unimplemented = "
If you are writing generic code, add a bound like

    where {OrigType}: Flatten,

This bound is required because Flatten is implemented only if length of output tuple is smaller than 13.



If not, implement ::flatten::NotTuple for your type like

    impl ::flatten::NotTuple for {OrigType} {{}}

This is required because current type system does not have negative reasoning."]
pub trait IntoTuple<OrigType>: ValidNode {
    type Out;

    fn into_tuple(self) -> Self::Out;
}

pub type TupleOf<C, OrigType> = <C as IntoTuple<OrigType>>::Out;

macro_rules! impl_for {
    (
        $(
            $t:ident => $($e:tt).*,
        )+
    ) => {
        impl< $( $t ),* , OrigType> IntoTuple<OrigType> for Cons![ $( $t, )* ]
            where $(
                    $t: ValidHead,
                  )*
        {
            type Out = ( $( $t ),* );
            #[inline(always)]
            fn into_tuple(self) -> Self::Out {
                // self.head => 0
                // self.tail.head => 1
                // self.tail.tail.head => 2
                // ...

                (
                    $(
                        self.
                        $($e).*
                    ),*
                )
            }
        }
    };
}

impl_for! {
    A => head,
    B => tail.head,
}

impl_for! {
    A => head,
    B => tail.head,
    C => tail.tail.head,
}

impl_for! {
    A => head,
    B => tail.head,
    C => tail.tail.head,
    D => tail.tail.tail.head,
}

impl_for! {
    A => head,
    B => tail.head,
    C => tail.tail.head,
    D => tail.tail.tail.head,
    E => tail.tail.tail.tail.head,
}

impl_for! {
    A => head,
    B => tail.head,
    C => tail.tail.head,
    D => tail.tail.tail.head,
    E => tail.tail.tail.tail.head,
    F => tail.tail.tail.tail.tail.head,
}

impl_for! {
    A => head,
    B => tail.head,
    C => tail.tail.head,
    D => tail.tail.tail.head,
    E => tail.tail.tail.tail.head,
    F => tail.tail.tail.tail.tail.head,
    G => tail.tail.tail.tail.tail.tail.head,
}

impl_for! {
    A => head,
    B => tail.head,
    C => tail.tail.head,
    D => tail.tail.tail.head,
    E => tail.tail.tail.tail.head,
    F => tail.tail.tail.tail.tail.head,
    G => tail.tail.tail.tail.tail.tail.head,
    H => tail.tail.tail.tail.tail.tail.tail.head,
}

impl_for! {
    A => head,
    B => tail.head,
    C => tail.tail.head,
    D => tail.tail.tail.head,
    E => tail.tail.tail.tail.head,
    F => tail.tail.tail.tail.tail.head,
    G => tail.tail.tail.tail.tail.tail.head,
    H => tail.tail.tail.tail.tail.tail.tail.head,
    I => tail.tail.tail.tail.tail.tail.tail.tail.head,
}

impl_for! {
    A => head,
    B => tail.head,
    C => tail.tail.head,
    D => tail.tail.tail.head,
    E => tail.tail.tail.tail.head,
    F => tail.tail.tail.tail.tail.head,
    G => tail.tail.tail.tail.tail.tail.head,
    H => tail.tail.tail.tail.tail.tail.tail.head,
    I => tail.tail.tail.tail.tail.tail.tail.tail.head,
    J => tail.tail.tail.tail.tail.tail.tail.tail.tail.head,
}

impl_for! {
    A => head,
    B => tail.head,
    C => tail.tail.head,
    D => tail.tail.tail.head,
    E => tail.tail.tail.tail.head,
    F => tail.tail.tail.tail.tail.head,
    G => tail.tail.tail.tail.tail.tail.head,
    H => tail.tail.tail.tail.tail.tail.tail.head,
    I => tail.tail.tail.tail.tail.tail.tail.tail.head,
    J => tail.tail.tail.tail.tail.tail.tail.tail.tail.head,
    K => tail.tail.tail.tail.tail.tail.tail.tail.tail.tail.head,
}

impl_for! {
    A => head,
    B => tail.head,
    C => tail.tail.head,
    D => tail.tail.tail.head,
    E => tail.tail.tail.tail.head,
    F => tail.tail.tail.tail.tail.head,
    G => tail.tail.tail.tail.tail.tail.head,
    H => tail.tail.tail.tail.tail.tail.tail.head,
    I => tail.tail.tail.tail.tail.tail.tail.tail.head,
    J => tail.tail.tail.tail.tail.tail.tail.tail.tail.head,
    K => tail.tail.tail.tail.tail.tail.tail.tail.tail.tail.head,
    L => tail.tail.tail.tail.tail.tail.tail.tail.tail.tail.tail.head,
}

impl<A: ValidHead, OrigType> IntoTuple<OrigType> for Cons<A, Nil> {
    type Out = A;
    #[inline(always)]
    fn into_tuple(self) -> Self::Out {
        (self.head)
    }
}
