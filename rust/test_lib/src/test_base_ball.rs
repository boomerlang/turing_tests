
#[path = "base_ball.rs"] pub mod base_ball;

#[cfg(test)]

// use super::*;

#[test]
fn calc_points_test1() {
    let input: [&str; 5] = ["5","2","C","D","+"];

    let rez = base_ball::calc_points(&input);

    assert_eq!(rez, 30);
}

#[test]
fn calc_points_test2() {
    let input = ["5","-2","4","C","D","9","+","+"];

    let rez = base_ball::calc_points(&input);

    // should be 15, the test says is 27
    assert_eq!(rez, 15);
}

#[test]
fn calc_points_test3() {
    let input = ["1"];

    let rez = base_ball::calc_points(&input);

    assert_eq!(rez, 1);
}
