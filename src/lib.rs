//#![feature(trace_macros)]
//trace_macros!(true);

#[macro_export]
macro_rules! impl_op {
    ($op:tt |$lhs_i:ident : &mut $lhs:path, $rhs_i:ident : &$rhs:path| $body:block) => (
        _parse_assignment_op!($op, $lhs, &$rhs, lhs, rhs, {|$lhs_i : &mut $lhs, $rhs_i : &$rhs| -> () {$body} (lhs, rhs);});
    );
    ($op:tt |$lhs_i:ident : &mut $lhs:path, $rhs_i:ident : $rhs:path| $body:block) => (
        _parse_assignment_op!($op, $lhs, $rhs, lhs, rhs, {|$lhs_i : &mut $lhs, $rhs_i : $rhs| -> () {$body} (lhs, rhs);});
    );
    ($op:tt |$lhs_i:ident : &$lhs:path| -> $out:path $body:block) => (
        _parse_unary_op!($op, &$lhs, $out, lhs, {|$lhs_i : &$lhs| -> $out {$body} (lhs)});
    );
    ($op:tt |$lhs_i:ident : &$lhs:path, $rhs_i:ident : &$rhs:path| -> $out:path $body:block) => (
        _parse_binary_op!($op, &$lhs, &$rhs, $out, lhs, rhs, {|$lhs_i : &$lhs, $rhs_i : &$rhs| -> $out {$body} (lhs, rhs)});
    );
    ($op:tt |$lhs_i:ident : &$lhs:path, $rhs_i:ident : $rhs:path| -> $out:path $body:block) => (
        _parse_binary_op!($op, &$lhs, $rhs, $out, lhs, rhs, {|$lhs_i : &$lhs, $rhs_i : $rhs| -> $out {$body} (lhs, rhs)});
    );
    ($op:tt |$lhs_i:ident : $lhs:path| -> $out:path $body:block) => (
        _parse_unary_op!($op, $lhs, $out, lhs, {|$lhs_i : $lhs| -> $out {$body} (lhs)});
    );
    ($op:tt |$lhs_i:ident : $lhs:path, $rhs_i:ident : &$rhs:path| -> $out:path $body:block) => (
        _parse_binary_op!($op, $lhs, &$rhs, $out, lhs, rhs, {|$lhs_i : $lhs, $rhs_i : &$rhs| -> $out {$body} (lhs, rhs)});
    );
    ($op:tt |$lhs_i:ident : $lhs:path, $rhs_i:ident : $rhs:path| -> $out:path $body:block) => (
        _parse_binary_op!($op, $lhs, $rhs, $out, lhs, rhs, {|$lhs_i : $lhs, $rhs_i : $rhs| -> $out {$body} (lhs, rhs)});
    );
}

#[macro_export]
macro_rules! impl_op_ex {
    ($op:tt |$lhs_i:ident : &mut $lhs:path, $rhs_i:ident : &$rhs:path| $body:block) => (
        _parse_assignment_op!($op, $lhs, &$rhs, lhs, rhs, {|$lhs_i : &mut $lhs, $rhs_i : &$rhs| -> () {$body} (lhs, rhs);});
        _parse_assignment_op!($op, $lhs, $rhs, lhs, rhs, {|$lhs_i : &mut $lhs, $rhs_i : &$rhs| -> () {$body} (lhs, &rhs);});
    );
    ($op:tt |$lhs_i:ident : &mut $lhs:path, $rhs_i:ident : $rhs:path| $body:block) => (
        _parse_assignment_op!($op, $lhs, $rhs, lhs, rhs, {|$lhs_i : &mut $lhs, $rhs_i : $rhs| -> () {$body} (lhs, rhs);});
    );
    ($op:tt |$lhs_i:ident : &$lhs:path| -> $out:path $body:block) => (
        _parse_unary_op!($op, &$lhs, $out, lhs, {|$lhs_i : &$lhs| -> $out {$body} (lhs)});
        _parse_unary_op!($op, $lhs, $out, lhs, {|$lhs_i : &$lhs| -> $out {$body} (&lhs)});
    );
    ($op:tt |$lhs_i:ident : &$lhs:path, $rhs_i:ident : &$rhs:path| -> $out:path $body:block) => (
        impl_op!($op |$lhs_i : &$lhs, $rhs_i : &$rhs| -> $out $body);
        _parse_binary_op!($op, &$lhs, $rhs, $out, lhs, rhs, {|$lhs_i : &$lhs, $rhs_i : &$rhs| -> $out {$body} (lhs, &rhs)});
        _parse_binary_op!($op, $lhs, &$rhs, $out, lhs, rhs, {|$lhs_i : &$lhs, $rhs_i : &$rhs| -> $out {$body} (&lhs, rhs)});
        _parse_binary_op!($op, $lhs, $rhs, $out, lhs, rhs, {|$lhs_i : &$lhs, $rhs_i : &$rhs| -> $out {$body} (&lhs, &rhs)});
    );
    ($op:tt |$lhs_i:ident : &$lhs:path, $rhs_i:ident : $rhs:path| -> $out:path $body:block) => (
        impl_op!($op |$lhs_i : &$lhs, $rhs_i : $rhs| -> $out $body);
        _parse_binary_op!($op, $lhs, $rhs, $out, lhs, rhs, {|$lhs_i : &$lhs, $rhs_i : $rhs| -> $out {$body} (&lhs, rhs)});
    );
    ($op:tt |$lhs_i:ident : $lhs:path|  -> $out:path $body:block) => (
        _parse_unary_op!($op, $lhs, $out, lhs, {|$lhs_i : $lhs| -> $out {$body} (lhs)});
    );
    ($op:tt |$lhs_i:ident : $lhs:path, $rhs_i:ident : &$rhs:path| -> $out:path $body:block) => (
        impl_op!($op |$lhs_i : $lhs, $rhs_i : &$rhs| -> $out $body);
        _parse_binary_op!($op, $lhs, $rhs, $out, lhs, rhs, {|$lhs_i : $lhs, $rhs_i : &$rhs| -> $out {$body} (lhs, &rhs)});
    );
    ($op:tt |$lhs_i:ident : $lhs:path, $rhs_i:ident : $rhs:path| -> $out:path $body:block) => (
        impl_op!($op |$lhs_i : $lhs, $rhs_i : $rhs| -> $out $body);
    );
}

#[macro_export]
macro_rules! impl_op_commutative {
    ($op:tt |$lhs_i:ident : &$lhs:path, $rhs_i:ident : &$rhs:path| -> $out:path $body:block) => (
        impl_op!($op |$lhs_i : &$lhs, $rhs_i : &$rhs| -> $out $body);
        _parse_binary_op!($op, &$rhs, &$lhs, $out, lhs, rhs, {|$lhs_i : &$lhs, $rhs_i : &$rhs| -> $out {$body} (rhs, lhs)});
    );
    ($op:tt |$lhs_i:ident : &$lhs:path, $rhs_i:ident : $rhs:path| -> $out:path $body:block) => (
        impl_op!($op |$lhs_i : &$lhs, $rhs_i : $rhs| -> $out $body);
        _parse_binary_op!($op, $rhs, &$lhs, $out, lhs, rhs, {|$lhs_i : &$lhs, $rhs_i : $rhs| -> $out {$body} (rhs, lhs)});
    );
    ($op:tt |$lhs_i:ident : $lhs:path, $rhs_i:ident : &$rhs:path| -> $out:path $body:block) => (
        impl_op!($op |$lhs_i : $lhs, $rhs_i : &$rhs| -> $out $body);
        _parse_binary_op!($op, &$rhs, $lhs, $out, lhs, rhs, {|$lhs_i : $lhs, $rhs_i : &$rhs| -> $out {$body} (rhs, lhs)});
    );
    ($op:tt |$lhs_i:ident : $lhs:path, $rhs_i:ident : $rhs:path| -> $out:path $body:block) => (
        impl_op!($op |$lhs_i : $lhs, $rhs_i : $rhs| -> $out $body);
        _parse_binary_op!($op, $rhs, $lhs, $out, lhs, rhs, {|$lhs_i : $lhs, $rhs_i : $rhs| -> $out {$body} (rhs, lhs)});
    );
}

#[macro_export]
macro_rules! impl_op_ex_commutative {
    ($op:tt |$lhs_i:ident : &$lhs:path, $rhs_i:ident : &$rhs:path| -> $out:path $body:block) => (
        impl_op_ex!($op |$lhs_i : &$lhs, $rhs_i : &$rhs| -> $out $body);

        _parse_binary_op!($op, &$rhs, &$lhs, $out, lhs, rhs, {|$lhs_i : &$lhs, $rhs_i : &$rhs| -> $out {$body} (rhs, lhs)});
        _parse_binary_op!($op, &$rhs, $lhs, $out, lhs, rhs, {|$lhs_i : &$lhs, $rhs_i : &$rhs| -> $out {$body} (&rhs, lhs)});
        _parse_binary_op!($op, $rhs, &$lhs, $out, lhs, rhs, {|$lhs_i : &$lhs, $rhs_i : &$rhs| -> $out {$body} (rhs, &lhs)});
        _parse_binary_op!($op, $rhs, $lhs, $out, lhs, rhs, {|$lhs_i : &$lhs, $rhs_i : &$rhs| -> $out {$body} (&rhs, &lhs)});
    );
    ($op:tt |$lhs_i:ident : &$lhs:path, $rhs_i:ident : $rhs:path| -> $out:path $body:block) => (
        impl_op_ex!($op |$lhs_i : &$lhs, $rhs_i : $rhs| -> $out $body);

        _parse_binary_op!($op, $rhs, &$lhs, $out, lhs, rhs, {|$lhs_i : &$lhs, $rhs_i : $rhs| -> $out {$body} (rhs, lhs)});
        _parse_binary_op!($op, $rhs, $lhs, $out, lhs, rhs, {|$lhs_i : &$lhs, $rhs_i : $rhs| -> $out {$body} (&rhs, lhs)});
    );
    ($op:tt |$lhs_i:ident : $lhs:path, $rhs_i:ident : &$rhs:path| -> $out:path $body:block) => (
        impl_op_ex!($op |$lhs_i : $lhs, $rhs_i : &$rhs| -> $out $body);

        _parse_binary_op!($op, &$rhs, $lhs, $out, lhs, rhs, {|$lhs_i : $lhs, $rhs_i : &$rhs| -> $out {$body} (rhs, lhs)});
        _parse_binary_op!($op, $rhs, $lhs, $out, lhs, rhs, {|$lhs_i : $lhs, $rhs_i : &$rhs| -> $out {$body} (rhs, &lhs)});
    );
    ($op:tt |$lhs_i:ident : $lhs:path, $rhs_i:ident : $rhs:path| -> $out:path $body:block) => (
        impl_op_commutative!($op |$lhs_i : $lhs, $rhs_i : $rhs| -> $out $body);
    );
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
    ($ops_trait:ident, $ops_fn:ident, &$lhs:ty, &$rhs:ty, $out:ty, $lhs_i:ident, $rhs_i:ident, $body:block) => (
        _impl_binary_op_borrowed_borrowed!($ops_trait, $ops_fn, $lhs, $rhs, $out, $lhs_i, $rhs_i, $body);
    );
    ($ops_trait:ident, $ops_fn:ident, &$lhs:ty, $rhs:ty, $out:ty, $lhs_i:ident, $rhs_i:ident, $body:block) => (
        _impl_binary_op_borrowed_owned!($ops_trait, $ops_fn, $lhs, $rhs, $out, $lhs_i, $rhs_i, $body);
    );
    ($ops_trait:ident, $ops_fn:ident, $lhs:ty, &$rhs:ty, $out:ty, $lhs_i:ident, $rhs_i:ident, $body:block) => (
        _impl_binary_op_owned_borrowed!($ops_trait, $ops_fn, $lhs, $rhs, $out, $lhs_i, $rhs_i, $body);
    );
    ($ops_trait:ident, $ops_fn:ident, $lhs:ty, $rhs:ty, $out:ty, $lhs_i:ident, $rhs_i:ident, $body:block) => (
        _impl_binary_op_owned_owned!($ops_trait, $ops_fn, $lhs, $rhs, $out, $lhs_i, $rhs_i, $body);
    );
}

#[macro_export]
macro_rules! _impl_binary_op_owned_owned {
    ($ops_trait:ident, $ops_fn:ident, $lhs:ty, $rhs:ty, $out:ty, $lhs_i:ident, $rhs_i:ident, $body:block) => (
        impl ops::$ops_trait<$rhs> for $lhs {
            type Output = $out;

            fn $ops_fn(self, $rhs_i: $rhs) -> Self::Output {
                let $lhs_i = self;
                $body
            }
                
        }
    );
}

#[macro_export]
macro_rules! _impl_binary_op_owned_borrowed {
    ($ops_trait:ident, $ops_fn:ident, $lhs:ty, $rhs:ty, $out:ty, $lhs_i:ident, $rhs_i:ident, $body:block) => (
        impl<'a> ops::$ops_trait<&'a $rhs> for $lhs {
            type Output = $out;

            fn $ops_fn(self, $rhs_i: &'a $rhs) -> Self::Output {
                let $lhs_i = self;
                $body
            }
                
        }
    );
}

#[macro_export]
macro_rules! _impl_binary_op_borrowed_owned {
    ($ops_trait:ident, $ops_fn:ident, $lhs:ty, $rhs:ty, $out:ty, $lhs_i:ident, $rhs_i:ident, $body:block) => (
        impl<'a> ops::$ops_trait<$rhs> for &'a $lhs {
            type Output = $out;

            fn $ops_fn(self, $rhs_i: $rhs) -> Self::Output {
                let $lhs_i = self;
                $body
            }
                
        }
    );
}

#[macro_export]
macro_rules! _impl_binary_op_borrowed_borrowed {
    ($ops_trait:ident, $ops_fn:ident, $lhs:ty, $rhs:ty, $out:ty, $lhs_i:ident, $rhs_i:ident, $body:block) => (
        impl<'a> ops::$ops_trait<&'a $rhs> for &'a $lhs {
            type Output = $out;

            fn $ops_fn(self, $rhs_i: &'a $rhs) -> Self::Output {
                let $lhs_i = self;
                $body
            }
                
        }
    );
}

#[macro_export]
macro_rules! _impl_assignment_op_internal {
    ($ops_trait:ident, $ops_fn:ident, $lhs:ty, &$rhs:ty, $lhs_i:ident, $rhs_i:ident, $body:block) => (        
        impl<'a> ops::$ops_trait<&'a $rhs> for $lhs {
            fn $ops_fn(&mut self, $rhs_i: &$rhs) {
                let mut $lhs_i = self;
                $body
            }
        }
    );
    ($ops_trait:ident, $ops_fn:ident, $lhs:ty, $rhs:ty, $lhs_i:ident, $rhs_i:ident, $body:block) => (        
        impl ops::$ops_trait<$rhs> for $lhs {
            fn $ops_fn(&mut self, $rhs_i: $rhs) {
                let mut $lhs_i = self;
                $body
            }
        }
    );
}

#[macro_export]
macro_rules! _impl_unary_op_internal {
    ($ops_trait:ident, $ops_fn:ident, &$lhs:ty, $out:ty, $lhs_i:ident, $body:block) => (        
        impl<'a> ops::$ops_trait for &'a $lhs {
            type Output = $out;

            fn $ops_fn(self) -> Self::Output {
                let $lhs_i = self;
                $body
            }
        }
    );
    ($ops_trait:ident, $ops_fn:ident, $lhs:ty, $out:ty, $lhs_i:ident, $body:block) => (        
        impl ops::$ops_trait for $lhs {
            type Output = $out;

            fn $ops_fn(self) -> Self::Output {
                let $lhs_i = self;
                $body
            }
        }
    );
}
