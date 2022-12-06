fn main() {
    let res = advent_of_code_2022::find_elf_carrying_most("./day1/resources/input.txt");
    match res {
        Some(res) => println!("{:?}", res),
        None => println!("No result")
    }
}