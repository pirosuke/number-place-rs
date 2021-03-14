extern crate number_place_lib;

use rand::seq::SliceRandom;
use std::fs::{read_dir, File};
use std::path::{Path, PathBuf};
use std::io::{BufReader, BufWriter, Write};
use std::ffi::OsStr;

use clap::{App, Arg};

fn fit_template(pattern_lines: &Vec<Vec<i32>>, template_lines: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
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
        .arg(Arg::with_name("pattern_dir")
            .help("Pattern JSON Dir Path")
            .long("pattern_dir")
            .short("p")
            .required(true)
            .takes_value(true)
        )
        .arg(Arg::with_name("template_dir")
            .help("Template JSON Dir Path")
            .long("template")
            .short("t")
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

    let p_template_dir_path = cli_options.value_of("template_dir").unwrap();
    let mut template_path_list: Vec<PathBuf> = read_dir(p_template_dir_path).unwrap()
        .map(|res| res.unwrap().path())
        .filter(|path| !path.is_dir() && path.extension().unwrap_or(OsStr::new("")) == "json")
        .collect();

    let p_output_dir_path = cli_options.value_of("output_dir").unwrap();
    let output_dir_path = Path::new(p_output_dir_path);

    let mut rng = rand::thread_rng();

    for pattern_path in pattern_path_list {
        let pattern_file = File::open(&pattern_path).unwrap();
        let pattern_reader = BufReader::new(pattern_file);
        let pattern_lines: Vec<Vec<i32>> = serde_json::from_reader(pattern_reader).unwrap();
        let pattern_file_name = pattern_path.file_name().unwrap().to_str().unwrap();

        template_path_list.shuffle(&mut rng);
        for template_path in &template_path_list {
            let template_file = File::open(&template_path).unwrap();
            let template_reader = BufReader::new(template_file);
            let template_lines: Vec<Vec<i32>> = serde_json::from_reader(template_reader).unwrap();
            let template_index = template_path.file_name().unwrap().to_str().unwrap().replace(".json", "");
        
            let fit_lines = fit_template(&pattern_lines, template_lines);
        
            if is_solvable(&fit_lines) {
                let json = serde_json::to_string(&fit_lines).unwrap()
                    .replace("[[", "[\n    [")
                    .replace("],", "],\n    ")
                    .replace("]]", "]\n]");
    
                let output_file_path = output_dir_path.join(format!("{}_{}", template_index, pattern_file_name));
                let mut f = BufWriter::new(File::create(output_file_path).unwrap());
                f.write_all(json.as_bytes()).unwrap();
                f.flush().unwrap();
                println!("resolved: {}_{}", template_index, pattern_file_name);
                break;
            } else {
                println!("unsolvable: {}_{}", template_index, pattern_file_name);
            }
        }
    }
}
