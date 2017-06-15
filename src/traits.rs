pub trait Add<RHS=Self> {
    type Output;
    fn add(lhs: &Self, rhs: &RHS) -> Self::Output;
}

pub trait Sub<RHS=Self> {
    type Output;
    fn sub(lhs: &Self, rhs: &RHS) -> Self::Output;
}

pub trait Mul<RHS=Self> {
    type Output;
    fn mul(lhs: &Self, rhs: &RHS) -> Self::Output;
}

pub trait Div<RHS=Self> {
    type Output;
    fn div(lhs: &Self, rhs: &RHS) -> Self::Output;
}

pub trait BitAnd<RHS=Self> {
    type Output;
    fn bitand(lhs: &Self, rhs: &RHS) -> Self::Output;
}

pub trait BitOr<RHS=Self> {
    type Output;
    fn bitor(lhs: &Self, rhs: &RHS) -> Self::Output;
}

pub trait BitXor<RHS=Self> {
    type Output;
    fn bitxor(lhs: &Self, rhs: &RHS) -> Self::Output;
}

pub trait Rem<RHS=Self> {
    type Output;
    fn rem(lhs: &Self, rhs: &RHS) -> Self::Output;
}

pub trait Shl<RHS=Self> {
    type Output;
    fn shl(lhs: &Self, rhs: &RHS) -> Self::Output;
}

pub trait Shr<RHS=Self> {
    type Output;
    fn shr(lhs: &Self, rhs: &RHS) -> Self::Output;
}