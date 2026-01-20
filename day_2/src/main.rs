use std::fs;

fn file_to_list(file: Vec<u8>) -> Vec<(u64, u64)> {
    let mut vec_2d: Vec<(u64, u64)> = Vec::new();
    let str_file = String::from_utf8(file).unwrap_or(String::from("Unable to convert u8 to str"));
    let n = str_file.chars().count();
    let mut vec_2: (u64, u64) = (0, 0);
    let mut l = 0;
    for i in 0..n {
        let char = str_file.chars().nth(i);
        match char {
            Some(val) => {
                if val == '-' {
                    vec_2.0 = str_file[l..i].parse::<u64>().unwrap_or(0);
                    l = i + 1;
                } else if val == ',' {
                    vec_2.1 = str_file[l..i].parse::<u64>().unwrap_or(0);
                    l = i + 1;
                    vec_2d.push(vec_2);
                } else if val == '\n' {
                    vec_2.1 = str_file[l..i].parse::<u64>().unwrap_or(0);
                    vec_2d.push(vec_2);
                }
            }
            None => (),
        }
    }
    println!("{:?}", vec_2d.clone());
    vec_2d
}

fn part_one(start: u64, end: u64) -> u64 {
    let mut sum = 0;
    for i in start..end {
        let mut tem = i;
        let mut cnt = 0;
        while tem > 0 {
            tem = tem / 10;
            cnt = cnt + 1;
        }

        if cnt % 2 == 0 {
            let mid = cnt / 2;
            let ten: u64 = 10;
            tem = i % ten.pow(mid);
            let tem2 = i / ten.pow(mid);

            if tem == tem2 {
                sum = sum + i;
            }
        }
    }

    sum
}

fn part_two(start: u64, end: u64) -> u64 {
    let mut sum = 0;
    let ten: u64 = 10;

    for i in start..end {
        let mut tem = i;
        let mut cnt = 0;
        while tem > 0 {
            tem = tem / 10;
            cnt = cnt + 1;
        }

        if cnt < 2 { continue; }

        for j in 2..=cnt {
            if cnt % j != 0 { continue; }
            let len = cnt / j;
            let base = ten.pow(len);
            let mut temp = i;
            let mut flag = true;
            let rep: u64 = temp % base;
            temp /= base;
            while temp > 0 {
                if temp % base != rep {
                    flag = false;
                    break;
                }
                temp /= base;
            }

            if flag {
                sum = sum + i;
                break;
            }
        }
    }

    sum
}

fn main() {
    let file_path = String::from("input.txt");
    let contents: Vec<u8> =
        fs::read(file_path).unwrap_or(("Unable to read file").as_bytes().to_vec());

    println!(
        "{:?}",
        String::from_utf8(contents.clone()).unwrap_or(String::from("Unable to convert u8 to str"))
    );

    let vec = file_to_list(contents);
    let mut sum_part1 = 0;
    let mut sum_part2 = 0;
    for i in vec {
        sum_part1 = sum_part1 + part_one(i.0, i.1);
        sum_part2 = sum_part2 + part_two(i.0, i.1);
    }

    println!("{}\n{}", sum_part1, sum_part2);
}
