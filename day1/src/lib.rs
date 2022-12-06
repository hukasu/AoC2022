pub fn find_elf_carrying_most(input_path: &str) -> Option<(usize, u32)> {
    let mut elf_ind = 0;
    let mut elf_ind_max = 0;
    let mut elf_backpack = 0;

    let mut elf_end_count = |cal_count: &mut u32| {
        if *cal_count > elf_backpack {
            elf_ind_max = elf_ind;
            elf_backpack = *cal_count;
        }
        elf_ind = elf_ind + 1;
        *cal_count = 0;
    };
    
    if let Ok(input) = std::fs::read_to_string(input_path) {
        let mut cal_count = 0;
        for food_item in input.split_terminator("\n") {
            if let Ok(cal) = food_item.trim().parse::<u32>() {
                cal_count = cal_count + cal;
            } else {
                elf_end_count(&mut cal_count);
            }
        }
        elf_end_count(&mut cal_count);
        Some((elf_ind_max, elf_backpack))
    } else { None }

}

pub fn find_top_three_elfs_carrying_most(input_path: &str) -> Option<((usize, u32), (usize, u32), (usize, u32))> {
    if let Ok(input) = std::fs::read_to_string(input_path) {
        let cals = input.split_terminator("\n")
            .into_iter()
            .map(
                |c| {
                    c.trim().parse::<u32>()
                }
            );
        let mut elf_ind: usize = 0;
        let mut elf_backpack = 0;
        let mut elfs = vec![];
        for cal in cals {
            match cal {
                Ok(cal) => {
                    elf_backpack = elf_backpack + cal
                },
                Err(_) => {
                    let ind = elfs.binary_search_by_key(
                        &elf_backpack,
                        |(_, kcal)| { *kcal }
                    );
                    match ind {
                        Ok(ind) => elfs.insert(ind + 1,(elf_ind, elf_backpack)),
                        Err(ind) => elfs.insert(ind, (elf_ind, elf_backpack))
                    }
                    elf_ind = elf_ind + 1;
                    elf_backpack = 0;
                }
            }
        }
        if let (_, [a, b, c]) = elfs.split_at(elfs.len() - 3) {
            Some((*a, *b, *c))
        } else {
            None
        }
    } else {
        None
    }
}