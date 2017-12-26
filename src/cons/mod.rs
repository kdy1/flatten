#![doc(hidden)]

pub mod fix;
use self::fix::Fix;

/// `Cons` or `Nil`.
pub trait Node: Sized {
    type ConsHead;
    type ConsTail: Node;
}
impl Node for Nil {
    type ConsHead = Nil;
    type ConsTail = Nil;
}
impl<H, T: Node> Node for Cons<H, T> {
    type ConsHead = H;
    type ConsTail = T;
}

pub type AsCons<N> = Cons<<N as Node>::ConsHead, <N as Node>::ConsTail>;

/// Everything **except** `Cons` and `Nil`.
pub trait ValidHead {}
#[allow(auto_impl)]
impl ValidHead for ..{}
impl !ValidHead for Nil {}
impl<H, T> !ValidHead for Cons<H, T> {}

/// Fixed node.
pub trait ValidNode: Node + Fix<Fixed = Self> {}
impl ValidNode for Nil {}
impl<Head: ValidHead, Tail: ValidNode> ValidNode for Cons<Head, Tail> {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Cons<H, T: Node> {
    pub head: H,
    pub tail: T,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Nil;

///Appends a value to end of `Cons`.
///
///# Notes for Nil
///
///As Nil is treated as a something like zero,
///
///```rust,ignore
///
///Nil + Nil = Nil
///Nil + Cons<H, T> = Cons<H, T>
///```
pub trait Append<Tail: ValidNode> {
    type Output: ValidNode;

    fn append(self, tail: Tail) -> Self::Output;
}

impl<T: ValidNode> Append<T> for Nil {
    type Output = T;

    #[inline(always)]
    fn append(self, t: T) -> Self::Output {
        t
    }
}

impl<Head, Tail, NewTail> Append<NewTail> for Cons<Head, Tail>
where
    Head: ValidHead,
    Tail: Node + Append<NewTail>,
    NewTail: ValidNode,
{
    type Output = Cons<Head, Concat<Tail, NewTail>>;
    #[inline(always)]
    fn append(self, tail: NewTail) -> Self::Output {
        Cons {
            head: self.head,
            tail: self.tail.append(tail),
        }
    }
}

impl<Value, Tail> Append<Tail> for Value
where
    Value: ValidHead,
    Tail: ValidNode,
{
    type Output = Cons<Value, Tail>;
    #[inline(always)]
    fn append(self, tail: Tail) -> Self::Output {
        Cons { head: self, tail }
    }
}
pub type Concat<Head, Tail> = <Head as Append<Tail>>::Output;

macro_rules! Cons {
    [
        $t:ty
    ] => {
        $crate::cons::Cons<$t, $crate::cons::Nil>
    };

    [
        $t:ty,
    ] => {
        $crate::cons::Cons<$t, $crate::cons::Nil>
    };

    [
        $t:ty,
        $ ($rest:ty,) *
    ] => {
        Cons<$t, Cons![ $( $rest, )* ]>
    };

    [
        $t:ty,
        $ ($rest:ty), *
    ] => {
        Cons<$t, Cons![ $( $rest, )* ]>
    };
}
