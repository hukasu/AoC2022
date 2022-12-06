fn main() {
    let res = advent_of_code_2022_day2::calculate_strategy_guide_score("./day2/resources/input.txt");
    match res {
        Some(res) => println!("{:?}", res),
        None => println!("No result")
    }
}