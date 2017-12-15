//! This is **not** a public api.
//!
//! Convert values to `Cons`.

use cons::*;
use cons::fix::{FixHead, FixedHead};

///Creates cons.
///
///
///# Note
/// Due to a [rust bug][], if your struct contains a tuple field,
/// You need to do `impl flatten::NotTuple for YourType`.
///
///
///
///[rust bug]:https://github.com/rust-lang/rust/issues/46707
pub trait IntoCons: Sized {
    type Out: ValidNode;

    ///# Examples
    ///
    ///```rust
    ///use flatten::into_cons::IntoCons;
    ///use flatten::cons::{Append, Cons, Nil};
    ///
    ///assert_eq!(('a', 1usize).into_cons(), 'a'.into_cons().append(1usize.into_cons()));
    ///fn sig(_: Cons<char, Cons<usize, Nil>>) {}
    ///sig(('a', 1usize).into_cons());
    ///
    ///```
    ///
    ///# Example for using struct with tuple field
    ///```rust
    ///use flatten::into_cons::{IntoCons, NotTuple};
    ///use flatten::cons::{Append, Cons, Nil};
    ///#[derive(Debug, PartialEq, Eq)]
    ///struct A { a: (u8, u8), }
    ///impl flatten::NotTuple for A {
    ///}
    ///# fn main() {
    ///assert_eq!(Cons{
    ///    head: A{ a: (0,1), },
    ///    tail: Nil,
    ///}, (A{a:(0, 1),},).into_cons());
    ///# }
    ///```
    fn into_cons(self) -> Self::Out;
}

pub trait SpecializedIntoCons: Sized {
    type Out: ValidNode;

    fn into_cons_inner(self) -> Self::Out;
}

impl<T> IntoCons for T
where
    T: IntoConsSpecializer,
{
    type Out = <<Self as IntoConsSpecializer>::Specialized as SpecializedIntoCons>::Out;
    fn into_cons(self) -> Self::Out {
        self.specialize().into_cons_inner()
    }
}

pub trait IntoConsSpecializer: Sized {
    type Specialized: From<Self> + SpecializedIntoCons;
    fn specialize(self) -> Self::Specialized {
        self.into()
    }
}

/// Assert that every type implements `IntoCons`.
trait AssertIntoCons: IntoCons {}
impl<T> AssertIntoCons for T {}



pub type ConsOf<V> = <V as IntoCons>::Out;

pub struct DefaultIntoCons<V>(V);
impl<V> From<V> for DefaultIntoCons<V> {
    fn from(v: V) -> Self {
        DefaultIntoCons(v)
    }
}

/// Used to remove unit tuples.
pub struct NilIntoCons;
impl From<()> for NilIntoCons {
    fn from(_: ()) -> Self {
        NilIntoCons
    }
}
impl IntoConsSpecializer for () {
    type Specialized = NilIntoCons;
}
impl SpecializedIntoCons for NilIntoCons {
    type Out = Nil;
    fn into_cons_inner(self) -> Self::Out {
        Nil
    }
}


impl<V> IntoConsSpecializer for V {
    default type Specialized = DefaultIntoCons<Self>;
}

pub trait NotTuple {}
#[allow(auto_impl)]
impl NotTuple for ..{}
impl !NotTuple for () {}
impl<A> !NotTuple for (A,) {}

impl<NormalValue> IntoConsSpecializer for NormalValue
where
    NormalValue: NotTuple + ValidHead,
{
    type Specialized = DefaultIntoCons<Self>;
}

#[macro_export]
macro_rules! register_flatten {
    ($Type:ty) => {
        impl $crate::into_cons::NotTuple for $Type {
            type Specialized = DefaultIntoCons<Self>;
        }
    };
}

// #[specialize]
impl<V> SpecializedIntoCons for DefaultIntoCons<V>
where
    Cons<V, Nil>: FixHead,
{
    type Out = FixedHead<Cons<V, Nil>>;

    #[inline(always)]
    fn into_cons_inner(self) -> Self::Out {
        Cons {
            head: self.0,
            tail: Nil,
        }.fix_head()
    }
}



pub struct TupleIntoCons<V>(V);
impl<V> From<V> for TupleIntoCons<V> {
    fn from(v: V) -> Self {
        TupleIntoCons(v)
    }
}

// #[specialize]
impl<A> IntoConsSpecializer for (A,) {
    type Specialized = TupleIntoCons<Self>;
}

impl<A> SpecializedIntoCons for TupleIntoCons<(A,)> {
    type Out = ConsOf<A>;

    fn into_cons_inner(self) -> Self::Out {
        let v = self.0;
        IntoCons::into_cons(v.0)
    }
}



/// This implements IntoCons for (A, B, C)
///  by changing it to A + (B, C).
macro_rules! impl_for_tuple {
    (
        ($first_i:tt, $first_ty:ident),
        $( ($i:tt, $N:ident), ) +
    ) => {
        impl<$first_ty, $( $N ),*> !NotTuple for ( $first_ty, $( $N, )* ) {}

        /// Disable default implementation.
        impl < $first_ty, $( $N ),* > IntoConsSpecializer for ( $first_ty, $( $N, )* )
            where ConsOf<$first_ty>: Append<ConsOf< ( $( $N, )* ) >>,{
            type Specialized = TupleIntoCons<Self>;
        }

        impl < $first_ty, $( $N ),* > SpecializedIntoCons
            for TupleIntoCons<( $first_ty, $( $N, )* )>
            where
                ConsOf<$first_ty>: Append<ConsOf<( $( $N, )* )>>,
            {
            type Out = Concat<
                ConsOf<$first_ty>,
                ConsOf<
                    ( $( $N ),* )
                >
            >;

            #[inline(always)]
            fn into_cons_inner(self) -> Self::Out {
                let v = self.0;
                let tail = IntoCons::into_cons(( $( v.$i, )* ));
                IntoCons::into_cons(v.$first_i).append(tail)
            }
        }
    };
}



impl_for_tuple! {
   (0, A),
   (1, B),
}

impl_for_tuple! {
   (0, A),
   (1, B),
   (2, C),
}

impl_for_tuple! {
   (0, A),
   (1, B),
   (2, C),
   (3, D),
}

impl_for_tuple! {
   (0, A),
   (1, B),
   (2, C),
   (3, D),
   (4, E),
}

impl_for_tuple! {
   (0, A),
   (1, B),
   (2, C),
   (3, D),
   (4, E),
   (5, F),
}

impl_for_tuple! {
   (0, A),
   (1, B),
   (2, C),
   (3, D),
   (4, E),
   (5, F),
   (6, G),
}

impl_for_tuple! {
   (0, A),
   (1, B),
   (2, C),
   (3, D),
   (4, E),
   (5, F),
   (6, G),
   (7, H),
}

impl_for_tuple! {
   (0, A),
   (1, B),
   (2, C),
   (3, D),
   (4, E),
   (5, F),
   (6, G),
   (7, H),
   (8, I),
}



impl_for_tuple! {
   (0, A),
   (1, B),
   (2, C),
   (3, D),
   (4, E),
   (5, F),
   (6, G),
   (7, H),
   (8, I),
   (9, J),
}



impl_for_tuple! {
   (0, A),
   (1, B),
   (2, C),
   (3, D),
   (4, E),
   (5, F),
   (6, G),
   (7, H),
   (8, I),
   (9, J),
   (10, K),
}

impl_for_tuple! {
   (0, A),
   (1, B),
   (2, C),
   (3, D),
   (4, E),
   (5, F),
   (6, G),
   (7, H),
   (8, I),
   (9, J),
   (10, K),
   (11, L),
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn simple_value() {
        assert_eq!(
            Cons {
                head: 1usize,
                tail: Nil,
            },
            1usize.into_cons()
        );
    }

    #[test]
    fn simple_tuple() {
        assert_eq!(
            (1usize,).into_cons(),
            Cons {
                head: 1usize,
                tail: Nil,
            }
        );
    }


    #[test]
    fn nested_tuple() {
        assert_eq!((1, 2).into_cons(), (((1,),), 2).into_cons());

        assert_eq!(
            (1, 2, (3, 4,), (5, 6, (7, 8))).into_cons(),
            (1, 2, 3, 4, 5, 6, 7, 8).into_cons()
        );
    }
}
