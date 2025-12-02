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
            dbg!(range, digits);
            let ev_n_digits = (digits.0.is_multiple_of(2) , digits.1.is_multiple_of(2) );
            let dub_d = |x, y| (x as f64 / 10.0_f64.powi(y as i32)).floor();
            let pad_d = |x, y| {
                let half = dub_d(x,y);
                (half * 10.0_f64.powi(y as i32) + half) as u64
            };
            let doubled = (pad_d(range.0, digits.0 / 2), pad_d(range.1, digits.1 / 2));
            dbg!(doubled);

            match ev_n_digits {
                (true, true) => {
                    if doubled.0 >= range.0 && doubled.0 <= range.1 { acc+=doubled.0; };
                    if doubled.0 != doubled.1 && doubled.1 >= range.0 && doubled.1 <= range.1 { acc+=doubled.1; };
                    acc
                },
                (false, true) =>{
                    if doubled.1 >= range.0 && doubled.1 <= range.1 { acc+=doubled.1; };
                    acc
                },
                (true, false) =>{
                    if doubled.0 >= range.0 && doubled.0 <= range.1 { acc+=doubled.0; };
                    acc
                },
                (false, false) => acc,
            }
        });
    println!("{answer}");
}
