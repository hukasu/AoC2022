fn main() {
    let res = advent_of_code_2022::find_top_three_elfs_carrying_most("./day1/resources/input.txt");
    match res {
        Some(res) => {
            println!("{:?}", res);
            println!("{}", res.0.1 + res.1.1 + res.2.1)
        },
        None => println!("No result")
    }
}