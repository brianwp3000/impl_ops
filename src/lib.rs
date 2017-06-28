//#![feature(trace_macros)]
//trace_macros!(true);

mod assignment;
mod binary;
mod unary;

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
