mod parsers;
use parsers::*;
mod evaluator;
use evaluator::*;
mod utils;
use utils::*;

fn main() {
    let input = r"
    { This is a comment }

    { Define bases }
    1 {}
    + {}
    _ {}

    { Define bases with parameters }
    $1 + $2 {}
    $ + 1 {}

    { Define constant function '2' }
    2 {}
    2 { 1 + 1 }
    dbg! { 2 }

    { Define constant function '3' }
    3 {}
    3 { 2 + 1 }
    dbg! { 3 }

    { Define function }
    mul_2_add_3 {}
    mul_2_add_3 $var {
        mul_2 {}
        mul_2 { $var + $var }

        3 + mul_2
    }

    { Print the result with the built-in function 'dbg!' }
    dbg! { mul_2_add_3 1 }
    ";
    input
        .char_indices()
        .src_code()
        .lexer()
        .parse_syn()
        .parse_sem()
        .evaluate_with_debug(EvalDebugOption::NONE);
}
