#![feature(trace_macros)]
#![feature(log_syntax)]

//trace_macros!(true);

#[macro_export]
macro_rules! impl_op {
    ($op:tt |$lhs_i:ident : &mut $lhs:path, $rhs_i:ident : &$rhs:path| $body:block) => (_parse_assignment_op!($op, $lhs, $rhs, |$lhs_i : &mut $lhs, $rhs_i : &$rhs| $body););
    ($op:tt |$lhs_i:ident : &$lhs:path|  -> $out:path $body:block) => (_parse_unary_op!($op, $lhs, $out, |$lhs_i : &$lhs| -> $out {$body}););
    ($op:tt |$lhs_i:ident : &$lhs:path, $rhs_i:ident : &$rhs:path| -> $out:path $body:block) => (_parse_binary_op!($op, $lhs, $rhs, $out, |$lhs_i : &$lhs, $rhs_i : &$rhs| -> $out {$body}););
}

#[macro_export]
macro_rules! impl_op_commutative {
    ($op:tt |$lhs_i:ident : &$lhs:path, $rhs_i:ident : &$rhs:path| -> $out:path $body:block) => (_parse_binary_op!($op, [commutative], $lhs, $rhs, $out, |$lhs_i : &$lhs, $rhs_i : &$rhs| -> $out {$body}););
}

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

#[macro_export]
macro_rules! _parse_assignment_op {
    (+=, $($t:tt)+) => (_impl_assignment_op_internal!(AddAssign, add_assign, $($t)+););
    (-=, $($t:tt)+) => (_impl_assignment_op_internal!(SubAssign, sub_assign, $($t)+););
    (*=, $($t:tt)+) => (_impl_assignment_op_internal!(MulAssign, mul_assign, $($t)+););
    (/=, $($t:tt)+) => (_impl_assignment_op_internal!(DivAssign, div_assign, $($t)+););
    (%=, $($t:tt)+) => (_impl_assignment_op_internal!(RemAssign, rem_assign, $($t)+););
    (&=, $($t:tt)+) => (_impl_assignment_op_internal!(BitAndAssign, bitand_assign, $($t)+););
    (|=, $($t:tt)+) => (_impl_assignment_op_internal!(BitOrAssign, bitor_assign, $($t)+););
    (^=, $($t:tt)+) => (_impl_assignment_op_internal!(BitXorAssign, bitxor_assign, $($t)+););
    (<<=, $($t:tt)+) => (_impl_assignment_op_internal!(ShlAssign, shl_assign, $($t)+););
    (>>=, $($t:tt)+) => (_impl_assignment_op_internal!(ShrAssign, shr_assign, $($t)+););
}

#[macro_export]
macro_rules! _parse_unary_op {
    (-, $($t:tt)+) => (_impl_unary_op_internal!(Neg, neg, $($t)+););
    (!, $($t:tt)+) => (_impl_unary_op_internal!(Not, not, $($t)+););
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

#[macro_export]
macro_rules! _impl_assignment_op_internal {
    ($ops_trait:ident, $ops_fn:ident, $lhs:ty, $rhs:ty, $fn:expr) => (        
        impl ops::$ops_trait<$rhs> for $lhs {
            fn $ops_fn(&mut self, rhs: $rhs) {
                $fn(self, &rhs)
            }
        }

        impl<'a> ops::$ops_trait<&'a $rhs> for $lhs {
            fn $ops_fn(&mut self, rhs: &$rhs) {
                $fn(self, rhs)
            }
        }
        
    );
}

#[macro_export]
macro_rules! _impl_unary_op_internal {
    ($ops_trait:ident, $ops_fn:ident, $lhs:ty, $out:ty, $fn:expr) => (        
        impl ops::$ops_trait for $lhs {
            type Output = $out;

            fn $ops_fn(self) -> Self::Output {
                $fn(&self)
            }
        }

        impl<'a> ops::$ops_trait for &'a $lhs {
            type Output = $out;

            fn $ops_fn(self) -> Self::Output {
                $fn(self)
            }
        }
    );
}
