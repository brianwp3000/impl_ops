#[doc(hidden)]
#[macro_export]
macro_rules! _parse_binary_op {
    (+, $($t:tt)+) => (_impl_binary_op_internal!(Add, add, $($t)+););
    (-, $($t:tt)+) => (_impl_binary_op_internal!(Sub, sub, $($t)+););
    (*, $($t:tt)+) => (_impl_binary_op_internal!(Mul, mul, $($t)+););
    (/, $($t:tt)+) => (_impl_binary_op_internal!(Div, div, $($t)+););
    (%, $($t:tt)+) => (_impl_binary_op_internal!(Rem, rem, $($t)+););
    (&, $($t:tt)+) => (_impl_binary_op_internal!(BitAnd, bitand, $($t)+););
    (|, $($t:tt)+) => (_impl_binary_op_internal!(BitOr, bitor, $($t)+););
    (^, $($t:tt)+) => (_impl_binary_op_internal!(BitXor, bitxor, $($t)+););
    (<<, $($t:tt)+) => (_impl_binary_op_internal!(Shl, shl, $($t)+););
    (>>, $($t:tt)+) => (_impl_binary_op_internal!(Shr, shr, $($t)+););
}

#[doc(hidden)]
#[macro_export]
macro_rules! _impl_binary_op_internal {
    ($ops_trait:ident, $ops_fn:ident, &$lhs:ty, &$rhs:ty, $out:ty, $lhs_i:ident, $rhs_i:ident, $body:block) => {
        _impl_binary_op_borrowed_borrowed!(
            $ops_trait, $ops_fn, $lhs, $rhs, $out, $lhs_i, $rhs_i, $body
        );
    };
    ($ops_trait:ident, $ops_fn:ident, &$lhs:ty, $rhs:ty, $out:ty, $lhs_i:ident, $rhs_i:ident, $body:block) => {
        _impl_binary_op_borrowed_owned!(
            $ops_trait, $ops_fn, $lhs, $rhs, $out, $lhs_i, $rhs_i, $body
        );
    };
    ($ops_trait:ident, $ops_fn:ident, $lhs:ty, &$rhs:ty, $out:ty, $lhs_i:ident, $rhs_i:ident, $body:block) => {
        _impl_binary_op_owned_borrowed!(
            $ops_trait, $ops_fn, $lhs, $rhs, $out, $lhs_i, $rhs_i, $body
        );
    };
    ($ops_trait:ident, $ops_fn:ident, $lhs:ty, $rhs:ty, $out:ty, $lhs_i:ident, $rhs_i:ident, $body:block) => {
        _impl_binary_op_owned_owned!($ops_trait, $ops_fn, $lhs, $rhs, $out, $lhs_i, $rhs_i, $body);
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! _impl_binary_op_owned_owned {
    ($ops_trait:ident, $ops_fn:ident, $lhs:ty, $rhs:ty, $out:ty, $lhs_i:ident, $rhs_i:ident, $body:block) => {
        impl ops::$ops_trait<$rhs> for $lhs {
            type Output = $out;

            fn $ops_fn(self, $rhs_i: $rhs) -> Self::Output {
                let $lhs_i = self;
                $body
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! _impl_binary_op_owned_borrowed {
    ($ops_trait:ident, $ops_fn:ident, $lhs:ty, $rhs:ty, $out:ty, $lhs_i:ident, $rhs_i:ident, $body:block) => {
        impl<'a> ops::$ops_trait<&'a $rhs> for $lhs {
            type Output = $out;

            fn $ops_fn(self, $rhs_i: &$rhs) -> Self::Output {
                let $lhs_i = self;
                $body
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! _impl_binary_op_borrowed_owned {
    ($ops_trait:ident, $ops_fn:ident, $lhs:ty, $rhs:ty, $out:ty, $lhs_i:ident, $rhs_i:ident, $body:block) => {
        impl<'a> ops::$ops_trait<$rhs> for &'a $lhs {
            type Output = $out;

            fn $ops_fn(self, $rhs_i: $rhs) -> Self::Output {
                let $lhs_i = self;
                $body
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! _impl_binary_op_borrowed_borrowed {
    ($ops_trait:ident, $ops_fn:ident, $lhs:ty, $rhs:ty, $out:ty, $lhs_i:ident, $rhs_i:ident, $body:block) => {
        impl<'a, 'b> ops::$ops_trait<&'a $rhs> for &'b $lhs {
            type Output = $out;

            fn $ops_fn(self, $rhs_i: &$rhs) -> Self::Output {
                let $lhs_i = self;
                $body
            }
        }
    };
}
