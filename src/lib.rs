#[macro_export]
macro_rules! impl_op {
    (($lhs:path) $op:tt ($rhs:path) -> ($out:path), $fn:expr) => (_parse_binary_op!($op, $lhs, $rhs, $out, $fn););
}

#[macro_export]
macro_rules! impl_op_closure {
    ($op:tt |$lhs_i:ident : &$lhs:path, $rhs_i:ident : &$rhs:path| -> $out:path {$body:expr}) => (_parse_binary_op!($op, $lhs, $rhs, $out, |$lhs_i : &$lhs, $rhs_i : &$rhs| -> $out {$body}););
}

#[macro_export]
macro_rules! impl_op_commutative {
    (($lhs:path) $op:tt ($rhs:path) -> ($out:path), $fn:expr) => (_parse_binary_op!($op, [commutative], $lhs, $rhs, $out, $fn););
}

#[macro_export]
macro_rules! _parse_binary_op {
    (+, $($t:tt)+) => (_impl_binary_op_internal!(Add, add, $($t)+););
    (-, $($t:tt)+) => (_impl_binary_op_internal!(Sub, sub, $($t)+););
    (*, $($t:tt)+) => (_impl_binary_op_internal!(Mul, mul, $($t)+););
    (/, $($t:tt)+) => (_impl_binary_op_internal!(Div, div, $($t)+););
    (&, $($t:tt)+) => (_impl_binary_op_internal!(BitAnd, bitand, $($t)+););
    (|, $($t:tt)+) => (_impl_binary_op_internal!(BitOr, bitor, $($t)+););
    (^, $($t:tt)+) => (_impl_binary_op_internal!(BitXor, bitxor, $($t)+););
    (%, $($t:tt)+) => (_impl_binary_op_internal!(Rem, rem, $($t)+););
    (<<, $($t:tt)+) => (_impl_binary_op_internal!(Shl, shl, $($t)+););
    (>>, $($t:tt)+) => (_impl_binary_op_internal!(Shr, shr, $($t)+););
}

#[macro_export]
macro_rules! _impl_binary_op_internal {
    ($ops_trait:ident, $ops_fn:ident, $lhs:ty, $rhs:ty, $out:ty, $fn:expr) => (
        impl ops::$ops_trait<$rhs> for $lhs {
            type Output = $out;

            fn $ops_fn(self, rhs: $rhs) -> Self::Output {
                $fn(&self, &rhs)
            }
        }

        impl<'a> ops::$ops_trait<&'a $rhs> for $lhs {
            type Output = $out;

            fn $ops_fn(self, rhs: &'a $rhs) -> Self::Output {
                $fn(&self, rhs)
            }
        }

        impl<'a> ops::$ops_trait<$rhs> for &'a $lhs {
            type Output = $out;

            fn $ops_fn(self, rhs: $rhs) -> Self::Output {
                $fn(self, &rhs)
            }
        }

        impl<'a> ops::$ops_trait<&'a $rhs> for &'a $lhs {
            type Output = $out;

            fn $ops_fn(self, rhs: &'a $rhs) -> Self::Output {
                $fn(self, rhs)
            }
        }
    );
    ($ops_trait:ident, $ops_fn:ident, [commutative], $lhs:ty, $rhs:ty, $out:ty, $fn:expr) => (
        _impl_binary_op_internal!($ops_trait, $ops_fn, $lhs, $rhs, $out, $fn);

        impl ops::$ops_trait<$lhs> for $rhs {
            type Output = $out;

            fn $ops_fn(self, rhs: $lhs) -> Self::Output {
                $fn(&rhs, &self)
            }
        }

        impl<'a> ops::$ops_trait<&'a $lhs> for $rhs {
            type Output = $out;

            fn $ops_fn(self, rhs: &'a $lhs) -> Self::Output {
                $fn(rhs, &self)
            }
        }

        impl<'a> ops::$ops_trait<$lhs> for &'a $rhs {
            type Output = $out;

            fn $ops_fn(self, rhs: $lhs) -> Self::Output {
                $fn(&rhs, self)
            }
        }

        impl<'a> ops::$ops_trait<&'a $lhs> for &'a $rhs {
            type Output = $out;

            fn $ops_fn(self, rhs: &'a $lhs) -> Self::Output {
                $fn(rhs, self)
            }
        }
    );
}
