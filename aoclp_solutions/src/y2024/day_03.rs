use aoclp::regex::CapturesHelper;
use aoclp::solvers_impl::input::safe_get_input;
use regex::Regex;

pub fn part_1() -> i64 {
    sum_of_muls(false)
}

pub fn part_2() -> i64 {
    sum_of_muls(true)
}

fn sum_of_muls(dos_and_donts: bool) -> i64 {
    let input = safe_get_input(2024, 3);

    let re =
        Regex::new(r"(?<mul>mul)\((?<a>\d{1,3}),(?<b>\d{1,3})\)|(?<do>do)\(\)|(?<dont>don't)\(\)")
            .unwrap();
    re.captures_iter(&input)
        .fold((0, true), |(mut sum, mut enabled), cap| {
            if dos_and_donts {
                if cap.name("do").is_some() {
                    enabled = true;
                } else if cap.name("dont").is_some() {
                    enabled = false;
                }
            }
            if enabled && cap.name("mul").is_some() {
                sum += cap.ez_get::<i64>("a") * cap.ez_get::<i64>("b");
            }
            (sum, enabled)
        })
        .0
}
