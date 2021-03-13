extern crate number_place_lib;

use std::fs::File;
use std::io::BufReader;

use clap::{App, Arg};

fn fit_template(pattern_lines: Vec<Vec<i32>>, template_lines: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut fit_lines: Vec<Vec<i32>> = Vec::new();
    for row_zip in pattern_lines.iter().zip(template_lines.iter()) {
        let (pattern_row, template_row) = row_zip;
        let mut fit_row: Vec<i32> = Vec::new();
        for col_zip in pattern_row.iter().zip(template_row.iter()) {
            let (pattern_col, template_col) = col_zip;
            fit_row.push(match template_col {
                1 => *pattern_col,
                _ => 0,
            });
        }
        fit_lines.push(fit_row);
    }

    fit_lines
}

fn contains_zero(lines: &Vec<Vec<i32>>) -> bool {
    let mut zero_exists = false;
    for row in lines {
        zero_exists = row.iter().any(|x| x == &0);
        if zero_exists {
            break;
        }
    }
    zero_exists
}

fn is_solvable(problem_lines: &Vec<Vec<i32>>) -> bool {
    let lines: Vec<Vec<i32>> = number_place_lib::solve_problem(problem_lines);
    !contains_zero(&lines)
}

fn main() {
    let cli_options = App::new("number_place_problem_generator")
        .about("Number Place Problem Generator")
        .arg(Arg::with_name("pattern")
            .help("Pattern JSON File Path")
            .long("pattern")
            .short("p")
            .required(true)
            .takes_value(true)
        )
        .arg(Arg::with_name("template")
            .help("Template JSON File Path")
            .long("template")
            .short("t")
            .required(true)
            .takes_value(true)
        )
        .get_matches();
    
    let p_pattern_file_path = cli_options.value_of("pattern").unwrap();
    let pattern_file = File::open(p_pattern_file_path).unwrap();
    let pattern_reader = BufReader::new(pattern_file);
    let pattern_lines: Vec<Vec<i32>> = serde_json::from_reader(pattern_reader).unwrap();

    let p_template_file_path = cli_options.value_of("template").unwrap();
    let template_file = File::open(p_template_file_path).unwrap();
    let template_reader = BufReader::new(template_file);
    let template_lines: Vec<Vec<i32>> = serde_json::from_reader(template_reader).unwrap();

    let fit_lines = fit_template(pattern_lines, template_lines);

    if is_solvable(&fit_lines) {
        let json = serde_json::to_string(&fit_lines);
        println!("{:}", json.unwrap());
    } else {
        println!("unsolvable");
    }
}
