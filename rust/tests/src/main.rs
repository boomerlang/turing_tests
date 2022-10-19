use test_lib;

fn main() 
{
    let inputs: [&[&str]; 3] = [
        &["5","2","C","D","+"],
        &["5","-2","4","C","D","9","+","+"],
        &["1"],
    ];

    for input in inputs {
        println!("{}", test_lib::base_ball::calc_points(&input));
    }

    let inputs = [
        "(]",
        "([)]",
         "()",
        "{[]}",
        "()[]{}",
        "()[({})]{[({})]}",
        "([{}])",
        "",
    ];
        
    for input in inputs {
        println!("{}", test_lib::paranthesis::match_paranthesis(&input));
    }
}
