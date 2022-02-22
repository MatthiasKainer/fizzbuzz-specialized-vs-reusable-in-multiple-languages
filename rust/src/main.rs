use std::time::Instant;

fn fizzbuzz(i: i128) -> String {
    match i {
        n if (n % 15 == 0) => "FizzBuzz".into(),
        n if (n % 3 == 0) => "Fizz".into(),
        n if (n % 5 == 0) => "Buzz".into(),
        _ => i.to_string()
    }
}

struct Rule {
    result: &'static str,
    test: fn(i128) -> bool,
}

fn number_or_all_matching_rules(i: i128, rules: Vec<&Rule>) -> String {
    let result = rules
        .iter()
        .map(|rule| if (rule.test)(i) { rule.result } else { "" })
        .collect::<Vec<&str>>()
        .join("");
    if result.is_empty() {
        i.to_string()
    } else {
        result
    }
}

fn main() {
    let range = 1..=1000000;

    let simple_now = Instant::now();
    let simple = range
        .clone()
        .map(fizzbuzz)
        .collect::<Vec<String>>()
        .join(";");
    let simple_elapsed = simple_now.elapsed();
    println!("Elapsed specialized:\t\t{:.2?}", simple_elapsed);

    let now = Instant::now();
    let rules = vec![
        &Rule {
            test: |i| i % 3 == 0,
            result: "Fizz",
        },
        &Rule {
            test: |i| i % 5 == 0,
            result: "Buzz",
        },
    ];
    let indirect = range
        .clone()
        .map(|i| {
            number_or_all_matching_rules(
                i,
                rules.clone(),
            )
        })
        .collect::<Vec<String>>()
        .join(";");
    let indirect_elapsed = now.elapsed();
    println!("Elapsed reusable:\t\t{:.2?}", indirect_elapsed);
    println!("Same results?\t\t\t{}", simple == indirect);
}