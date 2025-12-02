fn main() {
    let mut zero_count = 0;
    let mut comb_num: i32 = 50;
    //let input = include_str!("../../data/test1").lines();
    let input = include_str!("../../data/d1").lines();

    for line in input {

        let mut it = line.chars();
        let sign = it.next();
        let amount = it.as_str().parse::<i32>().expect("puzzle input should be consistent");

        match sign {
            Some('L') => {
                comb_num -= amount % 100;
                if comb_num < 0 { comb_num += 100; }
                if comb_num == 0 { zero_count += 1; }
            },
            Some('R') => {
                comb_num = (comb_num + amount) % 100;
                if comb_num == 0 { zero_count += 1; }
            }
            _ => panic!("první char nebyl R nebo L")
        };
    }
    println!("{}", zero_count);
}


