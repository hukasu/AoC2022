fn main() {
    let res = advent_of_code_2022_day10::decode_cpu_clock(
        "./day10/resources/input.txt",
        advent_of_code_2022_day10::default_probe
    );
    println!("{:?}", res)
}