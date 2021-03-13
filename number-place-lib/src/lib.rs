#[derive(Debug, Clone)]
struct SelectableColValues {
    row_index: usize,
    col_index: usize,
    values: Vec<i32>,
}

pub fn col_values(lines: &Vec<Vec<i32>>, col_index: usize) -> Vec<i32> {
    let mut values: Vec<i32> = Vec::new();
    for line in lines {
        values.push(line[col_index]);
    }

    values
}

pub fn block_values(lines: &Vec<Vec<i32>>, row_index: usize, col_index: usize) -> Vec<i32> {
    let mut values: Vec<i32> = Vec::new();

    let row_range = match row_index {
        0..=2 => (0..=2),
        3..=5 => (3..=5),
        _ => (6..=8),
    };

    let col_range = match col_index {
        0..=2 => (0..=2),
        3..=5 => (3..=5),
        _ => (6..=8),
    };

    for row in row_range {
        if lines.len() <= row {
            break;
        }
        let line = &lines[row];
        for col in col_range.clone() {
            values.push(line[col]);
        }
    }

    values
}

fn fetch_usable_values(lines: &Vec<Vec<i32>>) -> Vec<SelectableColValues> {
    let mut cell_selectable_values: Vec<SelectableColValues> = Vec::new();
    for (row_index, row) in lines.iter().enumerate() {
        for (col_index, col) in row.iter().enumerate() {
            if col == &0 {
                let mut used_values = col_values(lines, col_index);
                used_values.extend_from_slice(&row);
                used_values.extend_from_slice(&block_values(lines, row_index, col_index));
                let filtering_values: Vec<i32> = used_values.into_iter()
                    .filter(|x| *x != 0)
                    .collect();
                let usable_values: Vec<i32> = vec![1,2,3,4,5,6,7,8,9]
                    .iter()
                    .filter(|x| !filtering_values.contains(x))
                    .map(|x| x.clone() as i32)
                    .collect();
                cell_selectable_values.push(SelectableColValues{
                    row_index: row_index,
                    col_index: col_index,
                    values: usable_values,
                });
            }
        }
    }
    cell_selectable_values
}

pub fn count_zero(lines: &Vec<Vec<i32>>) -> usize {
    let mut zero_count: usize = 0;
    for row in lines {
        zero_count += row.iter().filter(|&&x| x == 0).count();
    }

    zero_count
}

fn is_value_contained_in_block(cell_selectable_values: &Vec<SelectableColValues>, row_index: usize, col_index: usize, value: &i32) -> bool {
    let row_range: Vec<usize> = match row_index {
        0..=2 => vec![0,1,2],
        3..=5 => vec![3,4,5],
        _ => vec![6,7,8],
    };

    let col_range = match col_index {
        0..=2 => vec![0,1,2],
        3..=5 => vec![3,4,5],
        _ => vec![6,7,8],
    };

    let other_cell_values: Vec<&SelectableColValues> = cell_selectable_values.iter()
        .filter(|cell_values| row_range.contains(&cell_values.row_index))
        .filter(|cell_values| col_range.contains(&cell_values.col_index))
        .filter(|x| !(x.col_index == col_index && x.row_index == row_index))
        .collect();

    let mut is_contained = false;
    for cell_values in other_cell_values {
        if cell_values.values.contains(value) {
            is_contained = true;
            break;
        }
    }

    is_contained
}

pub fn solve_problem(problem_lines: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut lines: Vec<Vec<i32>> = problem_lines.clone();
    let mut zero_count: usize = count_zero(&lines);
    let mut prev_zero_count: usize = 0;
    while zero_count > 0 && zero_count != prev_zero_count {
        prev_zero_count = zero_count;
        let cell_selectable_values: Vec<SelectableColValues> = fetch_usable_values(&lines);

        let unique_cell_values: Vec<SelectableColValues> = cell_selectable_values.iter()
            .filter(|x| x.values.len() == 1)
            .cloned()
            .collect();

        if unique_cell_values.len() > 0 {
            let values = &unique_cell_values[0];
            lines[values.row_index][values.col_index] = values.values[0];
            //println!("unique matched {:}-{:}", values.row_index, values.col_index);
        }
    
        if unique_cell_values.len() == 0 {
            //println!("{:?}", cell_selectable_values);
            'outer: for cell_values in &cell_selectable_values {
                let other_col_values: Vec<&SelectableColValues> = cell_selectable_values.iter()
                    .filter(|x| x.row_index == cell_values.row_index)
                    .filter(|x| x.col_index != cell_values.col_index)
                    .collect();
                let other_row_values: Vec<&SelectableColValues> = cell_selectable_values.iter()
                    .filter(|x| x.col_index == cell_values.col_index)
                    .filter(|x| x.row_index != cell_values.row_index)
                    .collect();
                for value in &cell_values.values {
                    let no_dup_in_col = !other_col_values.iter().any(|x| x.values.contains(value));
                    let no_dup_in_row = !other_row_values.iter().any(|x| x.values.contains(value));
                    let no_dub_in_block = !is_value_contained_in_block(&cell_selectable_values, cell_values.row_index, cell_values.col_index, value);
                    if no_dup_in_row || no_dup_in_col || no_dub_in_block {
                        lines[cell_values.row_index][cell_values.col_index] = *value;
                        //println!("no dup row: {:}, col: {:}, block: {:}", no_dup_in_row, no_dup_in_col, no_dub_in_block);
                        //println!("matched {:}-{:}", cell_values.row_index, cell_values.col_index);
                        break 'outer;
                    }
                }
            }
        }

        //println!("{:?}", lines);
        zero_count = count_zero(&lines);
    }
    
    lines
}
