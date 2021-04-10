extern crate number_place_lib;

use std::fs::{read_dir, File};
use std::path::{Path, PathBuf};
use std::io::{BufReader, BufWriter, Write};
use std::ffi::OsStr;

use rand::seq::SliceRandom;
use clap::{App, Arg};

struct Cell {
    row_index: usize,
    col_index: usize,
    value: i32,
}

fn fit_template(pattern_lines: &Vec<Vec<i32>>, template_lines: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
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

fn add_blank_to_template(template_lines: &Vec<Vec<i32>>, number_of_blanks: usize) -> Vec<Vec<i32>> {
    let mut cells: Vec<Cell> = Vec::new();
    for (row_index, row) in template_lines.iter().enumerate() {
        for (col_index, col) in row.iter().enumerate() {
            cells.push(Cell{
                row_index: row_index,
                col_index: col_index,
                value: *col,
            });
        }
    }

    let mut filled_cells: Vec<&Cell> = cells.iter()
        .filter(|x| x.value == 1)
        .collect();
    let mut rng = rand::thread_rng();
    filled_cells.shuffle(&mut rng);

    let updated_lines: Vec<Vec<i32>> = filled_cells.iter()
        .take(number_of_blanks)
        .fold(template_lines.clone(), |mut acc, x| { acc[x.row_index][x.col_index] = 0; acc});

    updated_lines
}

fn main() {
    let cli_options = App::new("number_place_problem_generator")
        .about("Number Place Problem Generator")
        .arg(Arg::with_name("pattern_dir")
            .help("Pattern JSON Dir Path")
            .long("pattern_dir")
            .short("p")
            .required(true)
            .takes_value(true)
        )
        .arg(Arg::with_name("output_dir")
            .help("Output Dir Path")
            .long("output")
            .short("o")
            .required(true)
            .takes_value(true)
        )
        .get_matches();
    
    let p_pattern_dir_path = cli_options.value_of("pattern_dir").unwrap();
    let pattern_path_list: Vec<PathBuf> = read_dir(p_pattern_dir_path).unwrap()
        .map(|res| res.unwrap().path())
        .filter(|path| !path.is_dir() && path.extension().unwrap_or(OsStr::new("")) == "json")
        .collect();

    let template_lines = vec![
        vec![1,1,1,1,1,1,1,1,1],
        vec![1,1,1,1,1,1,1,1,1],
        vec![1,1,1,1,1,1,1,1,1],
        vec![1,1,1,1,1,1,1,1,1],
        vec![1,1,1,1,1,1,1,1,1],
        vec![1,1,1,1,1,1,1,1,1],
        vec![1,1,1,1,1,1,1,1,1],
        vec![1,1,1,1,1,1,1,1,1],
        vec![1,1,1,1,1,1,1,1,1],
    ];

    let p_output_dir_path = cli_options.value_of("output_dir").unwrap();
    let output_dir_path = Path::new(p_output_dir_path);

    for pattern_path in pattern_path_list {
        let pattern_file = File::open(&pattern_path).unwrap();
        let pattern_reader = BufReader::new(pattern_file);
        let pattern_lines: Vec<Vec<i32>> = serde_json::from_reader(pattern_reader).unwrap();
        let pattern_file_name = pattern_path.file_name().unwrap().to_str().unwrap();

        let mut hint_lines: Vec<Vec<i32>> = template_lines.clone();

        let mut number_of_blanks = 5;
        hint_lines = add_blank_to_template(&hint_lines, 5);

        let mut is_solved = true;
        let mut prev_fit_lines: Vec<Vec<i32>> = Vec::new();
        while is_solved {
            number_of_blanks = number_of_blanks + 2;
            hint_lines = add_blank_to_template(&hint_lines, 2);
        
            let fit_lines = fit_template(&pattern_lines, &hint_lines);
        
            is_solved = is_solvable(&fit_lines);
            prev_fit_lines = fit_lines.clone();
        }

        let json = serde_json::to_string(&prev_fit_lines).unwrap()
            .replace("[[", "[\n    [")
            .replace("],", "],\n    ")
            .replace("]]", "]\n]");

        let output_file_path = output_dir_path.join(pattern_file_name);
        let mut f = BufWriter::new(File::create(output_file_path).unwrap());
        f.write_all(json.as_bytes()).unwrap();
        f.flush().unwrap();

        println!("{}_{}", number_of_blanks, pattern_file_name);
    }
}
