extern crate number_place_lib;

use rand::seq::SliceRandom;
use clap::{App, Arg};
//use chrono::prelude::*;
use std::path::Path;
use std::fs::File;
use std::io::{BufWriter, Write};

#[derive(Debug, Clone)]
struct SelectableColValues {
    col_index: usize,
    values: Vec<i32>,
}

fn generate_line(lines: &Vec<Vec<i32>>) -> Vec<i32> {
    // 各列ごとにブロックと列の入力済みの値を確認して入力可能な値を抽出する
    // 入力可能な値が存在しない場合は成り立たないのでこの関数は終了して前の行に戻るか1行目から再算出する
    // 他の列に存在しない固有値を持つ列がある場合はその列に固有値を設定する
    // 入力可能な値が最も少ない列の値を設定し、再度他の列の入力可能な値を抽出する
    let row_index = lines.len();

    let mut remaining_values_list: Vec<SelectableColValues> = Vec::new();
    for col_index in 0..=8 {
        let mut used_values = number_place_lib::col_values(lines, col_index);
        used_values.extend_from_slice(&number_place_lib::block_values(lines, row_index, col_index));
        let remaining_values: Vec<i32> = vec![1,2,3,4,5,6,7,8,9]
            .iter()
            .filter(|x| !used_values.contains(x))
            .map(|x| x.clone() as i32)
            .collect();
        remaining_values_list.push(SelectableColValues{
            col_index: col_index,
            values: remaining_values,
        });
    }

    let mut line: Vec<i32> = vec![0,0,0,0,0,0,0,0,0];
    let mut rng = rand::thread_rng();

    let mut col_list: Vec<SelectableColValues> = Vec::new();
    let mut loop_count = 0;
    let max_loop_count = 100;
    while (line.contains(&0) || col_list.len() > 0) && loop_count < max_loop_count {
        col_list = remaining_values_list.clone().into_iter()
            .filter(|scv| scv.values.len() >= 1)
            .collect();
        col_list.sort_by(|a, b| a.values.len().partial_cmp(&b.values.len()).unwrap());
        for col_selectable_info in &col_list {
            let mut selectable_values = col_selectable_info.values.clone();
            selectable_values.shuffle(&mut rng);
            for selectable_value in selectable_values {
                if !line.contains(&selectable_value) {
                    line[col_selectable_info.col_index] = selectable_value;

                    let mut filtered_remaining_values_list: Vec<SelectableColValues> = Vec::new();
                    for remaining_value in remaining_values_list.iter() {
                        let filtered_values: Vec<i32> = remaining_value.values.clone().into_iter()
                            .filter(|&v| v != selectable_value)
                            .collect();
                        filtered_remaining_values_list.push(SelectableColValues{
                            col_index: remaining_value.col_index,
                            values: filtered_values,
                        });
                    }
                    remaining_values_list = filtered_remaining_values_list;
                    break;
                }
            }
        }
        loop_count += 1;
    }

    line
}

fn generate_pattern() -> Vec<Vec<i32>> {
    let mut try_count = 0;
    let max_try_count = 1000;
    let mut are_numbers_fulfilled = false;

    let mut lines: Vec<Vec<i32>> = Vec::new();
    while !are_numbers_fulfilled && try_count < max_try_count {
        lines = Vec::new();

        // 1行目はシャッフルして生成
        let mut line1 = vec![1,2,3,4,5,6,7,8,9];
        let mut rng = rand::thread_rng();
        line1.shuffle(&mut rng);

        lines.push(line1);

        // 2行目以降はまず1〜9のシャッフルしたリストを作成し、
        // 1マスごとに列とブロックにセット可能な数字かチェックしてセット可能な数字ならセットする
        let mut contains_zero = false;
        for _i in 1..=8 {
            let line = generate_line(&lines);
            if line.contains(&0) {
                contains_zero = true;
            }
            lines.push(line);

            if contains_zero {
                break;
            }
        }

        if !contains_zero {
            are_numbers_fulfilled = true;
        }
        try_count += 1;
    }

    lines
}

fn main() {
    let cli_options = App::new("number_place_pattern_generator")
        .about("Number Place Pattern Generator")
        .arg(Arg::with_name("output_dir")
            .help("Pattern JSON Output Dir Path")
            .long("output_dir")
            .short("o")
            .required(true)
            .takes_value(true)
        )
        .arg(Arg::with_name("num")
            .help("Number Of Patterns")
            .long("num")
            .short("n")
            .required(true)
            .takes_value(true)
        )
        .get_matches();
    
    let p_pattern_output_dir_path = cli_options.value_of("output_dir").unwrap();
    let p_num_of_patterns = cli_options.value_of("num").unwrap();

    //let ts = Local::now().format("%Y%m%d");
    let output_dir_path = Path::new(p_pattern_output_dir_path);

    let num_of_patterns = p_num_of_patterns.parse().unwrap();
    let mut num_outputted = 0;
    while num_outputted < num_of_patterns {
        let lines: Vec<Vec<i32>> = generate_pattern();

        if number_place_lib::count_zero(&lines) == 0 {
            let json = serde_json::to_string(&lines).unwrap()
                .replace("[[", "[\n    [")
                .replace("],", "],\n    ")
                .replace("]]", "]\n]");

            let output_file_path = output_dir_path.join(format!("{}.json", num_outputted));
            let mut f = BufWriter::new(File::create(output_file_path).unwrap());
            f.write_all(json.as_bytes()).unwrap();
            f.flush().unwrap();
        
            num_outputted += 1;
        }
    }
}
