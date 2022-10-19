#[path = "paranthesis.rs"] pub mod paranthesis;

#[cfg(test)]

// use super::*;

#[test]
fn match_paranthesis_test1()
{
    let input = "()";

    let rez = paranthesis::match_paranthesis(input);

    assert_eq!(rez, true);
}

#[test]
fn match_paranthesis_test2()
{
    let input = "(]";

    let rez = paranthesis::match_paranthesis(input);

    assert_eq!(rez, false);
}

#[test]
fn match_paranthesis_test3()
{
    let input = "([)]";

    let rez = paranthesis::match_paranthesis(input);

    assert_eq!(rez, false);
}

#[test]
fn match_paranthesis_test4()
{
    let input = "()";

    let rez = paranthesis::match_paranthesis(input);

    assert_eq!(rez, true);
}

#[test]
fn match_paranthesis_test5()
{
    let input = "{[]}";

    let rez = paranthesis::match_paranthesis(&input);

    assert_eq!(rez, true);
}

#[test]
fn match_paranthesis_test6()
{
    let input = "()[]{}";

    let rez = paranthesis::match_paranthesis(input);

    assert_eq!(rez, true);
}

#[test]
fn match_paranthesis_test7()
{
    let input = "()[({})]{[({})]}";

    let rez = paranthesis::match_paranthesis(input);

    assert_eq!(rez, true);
}

#[test]
fn match_paranthesis_test8()
{
    let input = "";

    let rez = paranthesis::match_paranthesis(input);

    assert_eq!(rez, false);
}
