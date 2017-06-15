mod traits;
pub use self::traits::*;

#[macro_export]
macro_rules! impl_op {
    (($lhs:path) + ($rhs:path) = ($out:path)) => (_impl_binary_op_internal!(Add, add, $lhs, $rhs, $out););
    (($lhs:path) + ($rhs:path) = ($out:path), $fn:path) => (_impl_binary_op_internal!(Add, add, $lhs, $rhs, $out, $fn););
    (($lhs:path) - ($rhs:path) = ($out:path)) => (_impl_binary_op_internal!(Sub, sub, $lhs, $rhs, $out););
    (($lhs:path) - ($rhs:path) = ($out:path), $fn:path) => (_impl_binary_op_internal!(Sub, sub, $lhs, $rhs, $out, $fn););
    (($lhs:path) * ($rhs:path) = ($out:path)) => (_impl_binary_op_internal!(Mul, mul, $lhs, $rhs, $out););
    (($lhs:path) * ($rhs:path) = ($out:path), $fn:path) => (_impl_binary_op_internal!(Mul, mul, $lhs, $rhs, $out, $fn););
    (($lhs:path) / ($rhs:path) = ($out:path)) => (_impl_binary_op_internal!(Div, div, $lhs, $rhs, $out););
    (($lhs:path) / ($rhs:path) = ($out:path), $fn:path) => (_impl_binary_op_internal!(Div, div, $lhs, $rhs, $out, $fn););
    (($lhs:path) & ($rhs:path) = ($out:path)) => (_impl_binary_op_internal!(BitAnd, bitand, $lhs, $rhs, $out););
    (($lhs:path) & ($rhs:path) = ($out:path), $fn:path) => (_impl_binary_op_internal!(BitAnd, bitand, $lhs, $rhs, $out, $fn););
    (($lhs:path) | ($rhs:path) = ($out:path)) => (_impl_binary_op_internal!(BitOr, bitor, $lhs, $rhs, $out););
    (($lhs:path) | ($rhs:path) = ($out:path), $fn:path) => (_impl_binary_op_internal!(BitOr, bitor, $lhs, $rhs, $out, $fn););
    (($lhs:path) ^ ($rhs:path) = ($out:path)) => (_impl_binary_op_internal!(BitXor, bitxor, $lhs, $rhs, $out););
    (($lhs:path) ^ ($rhs:path) = ($out:path), $fn:path) => (_impl_binary_op_internal!(BitXor, bitxor, $lhs, $rhs, $out, $fn););
    (($lhs:path) % ($rhs:path) = ($out:path)) => (_impl_binary_op_internal!(Rem, rem, $lhs, $rhs, $out););
    (($lhs:path) % ($rhs:path) = ($out:path), $fn:path) => (_impl_binary_op_internal!(Rem, rem, $lhs, $rhs, $out, $fn););
    (($lhs:path) << ($rhs:path) = ($out:path)) => (_impl_binary_op_internal!(Shl, shl, $lhs, $rhs, $out););
    (($lhs:path) << ($rhs:path) = ($out:path), $fn:path) => (_impl_binary_op_internal!(Shl, shl, $lhs, $rhs, $out, $fn););
    (($lhs:path) >> ($rhs:path) = ($out:path)) => (_impl_binary_op_internal!(Shr, shr, $lhs, $rhs, $out););
    (($lhs:path) >> ($rhs:path) = ($out:path), $fn:path) => (_impl_binary_op_internal!(Shr, shr, $lhs, $rhs, $out, $fn););
}

#[macro_export]
macro_rules! _impl_binary_op_internal {
    ($ops_trait:ident, $ops_fn:ident, $lhs:ty, $rhs:ty, $out:ty) => (_impl_binary_op_internal!($ops_trait, $ops_fn, $lhs, $rhs, $out, impl_ops::$ops_trait::$ops_fn););
    ($ops_trait:ident, $ops_fn:ident, $lhs:ty, $rhs:ty, $out:ty, $fn:path) => (
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
    )
}
