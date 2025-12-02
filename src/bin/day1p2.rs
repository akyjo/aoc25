fn main() {
    let mut zero_count = 0;
    let mut comb_num: i32 = 50;
    // let input = include_str!("../../data/test1").lines();
    let input = include_str!("../../data/d1").lines();

    for line in input {

        // dbg!(line);
        let mut it = line.chars();
        let sign = it.next();
        let amount = it.as_str().parse::<i32>().expect("puzzle input should be consistent");
        let mut initial_zero = false;

        match sign {
            Some('L') => {
                if comb_num == 0 {
                    initial_zero = true;
                }
                comb_num -= amount % 100;
                zero_count += amount / 100;
                if comb_num < 0 {
                    comb_num += 100;
                    if !initial_zero {
                        zero_count += 1;
                    }
                }
                if comb_num == 0 {
                    zero_count += 1;
                    initial_zero = true;
                } else {
                    initial_zero = false;
                }
            },
            Some('R') => {
                comb_num += amount;
                zero_count += comb_num / 100;
                comb_num %= 100;
                //if comb_num == 0 { zero_count += 1; }
            }
            _ => panic!("první char nebyl R nebo L")
        };
        //dbg!(comb_num, zero_count);
    }
    println!("{}", zero_count);
    println!("{}", -505i32%100i32);
}


