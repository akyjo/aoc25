fn main () {
    // let input = include_str!("../../data/test2");
    let input = include_str!("../../data/d2");
    let answer = input.split(",")
        .map(|range| 
        {
            let mut range_it = range.split("-");
            let first = range_it.next().expect("invalid range in data").trim().parse::<u64>().expect("failed to parse into u64");
            let second = range_it.next().expect("invalid range in data").trim().parse::<u64>().expect("failed to parse into u64");

            (first, second)
        }).fold(0, |mut acc, range| 
        {
            let digits = (range.0.ilog(10) + 1, range.1.ilog(10) + 1);
            let pad_d = |x, y| {
                x * 10u64.pow(y) + x
            };
            let lower = digits.0 / 2;
            let upper = (digits.1 / 2) + if digits.1.is_multiple_of(2) {1} else {2};
            // dbg!(lower, upper);

            let res = (10u64.pow((lower as i32 - 1).max(0) as u32)..=(10u64.pow(upper) - 1))
                .map(|x| pad_d(x, x.ilog(10) + 1))
                .filter(|x| range.0 <= *x &&  *x <= range.1)
               .sum::<u64>();
            acc += res;
            acc

        });

    println!("{answer}");
}
