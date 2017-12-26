//! Make `Cons` flat.

use cons::*;

/// Convert
///
///    [A, B, [C, Nil], [[D, Nil], Nil], Nil]
/// to [A, B, C, D, Nil]
///
///
/// Recursion trait for `Tail`.
pub trait Fix: Node + Sized {
    // type Fixed: Fix<Fixed = <Self as Fix>::Fixed>;
    type Fixed: ValidNode;

    fn fix(self) -> Self::Fixed;
}

/// Fixed `Tail`.
pub type Fixed<T> = <T as Fix>::Fixed;

/// Recursion trait for `Head`.
pub trait FixHead {
    type Fixed: ValidNode;
    fn fix_head(self) -> Self::Fixed;
}

/// Fixed `Head`.
pub type FixedHead<T> = <T as FixHead>::Fixed;

/// Termination of recursion for `Head`.
impl<V> FixHead for V
where
    V: ValidHead,
{
    type Fixed = Cons<Self, Nil>;
    #[inline(always)]
    fn fix_head(self) -> Self::Fixed {
        Cons {
            head: self,
            tail: Nil,
        }
    }
}

impl<H: FixHead, T: Node> FixHead for Cons<H, T>
where
    FixedHead<H>: Append<Fixed<T>>,
    T: Fix,
{
    type Fixed = ConcatFixed<H, T>;
    #[inline(always)]
    fn fix_head(self) -> Self::Fixed {
        self.head.fix_head().append(self.tail.fix())
    }
}

/// Concat `Head` and `Tail` dropping `Nil`.
pub type Concat<Head, Tail> = <Head as Append<Tail>>::Output;

pub type ConcatFixed<Head, Tail> = Concat<FixedHead<Head>, Fixed<Tail>>;

/// Termination of recursion for `Tail`.
impl Fix for Nil {
    type Fixed = Nil;
    #[inline(always)]
    fn fix(self) -> Self {
        self
    }
}

impl<Head, Tail> Fix for Cons<Head, Tail>
where
    Head: FixHead,
    Tail: Node + Fix,
    FixedHead<Head>: Append<Fixed<Tail>>,
{
    type Fixed = ConcatFixed<Head, Tail>;
    #[inline(always)]
    fn fix(self) -> Self::Fixed {
        self.head.fix_head().append(self.tail.fix())
    }
}

/// Compile time assertion.
trait AssertFix<Res: Node>: Fix<Fixed = Res>
where
    Res: ValidNode + Fix<Fixed = Res>,
{
}
trait AssertValid: Node + ValidNode + Fix<Fixed = Self> {}

impl AssertValid for Cons<usize, Nil> {}

impl AssertFix<Cons<usize, Nil>> for Cons<Cons<usize, Nil>, Nil> {}
impl AssertFix<Cons<usize, Nil>> for Cons<Cons<Cons<usize, Nil>, Nil>, Nil> {}

impl AssertFix<Cons<usize, Cons<u64, Nil>>> for Cons<Cons<Cons<usize, Nil>, Cons<u64, Nil>>, Nil> {}
