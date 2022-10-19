
pub fn calc_points(input: &[&str]) -> i32 
{
    let mut out: Vec<i32> = vec![];

    let mut deleted = 0;

    for ch in input {
        let val0 = ch.parse::<i32>();

        match val0 {
            Ok(val) => {
                out.push(val);
            },
            Err(_e) => {
                if *ch == "D" {
                    let crt = out.pop().unwrap();
                    out.push(crt);
                    out.push(crt * deleted);
                } else if *ch == "C" {
                    deleted = out.pop().unwrap();
                } else if *ch == "+"{
                    let x1 = out.pop().unwrap();
                    let x0 = out.pop().unwrap();
                    out.push(x0);
                    out.push(x1);
                    out.push(x0 + x1);
                }
            },
        }
    }

    let mut sum = 0;

    for elem in out {
        sum += elem;
    }

    sum
}
