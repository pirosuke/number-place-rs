use std::collections::HashSet;

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

fn is_in_same_block(row_index1: usize, col_index1: usize, row_index2: usize, col_index2: usize) -> bool {
    let row_range: Vec<usize> = match row_index1 {
        0..=2 => vec![0,1,2],
        3..=5 => vec![3,4,5],
        _ => vec![6,7,8],
    };

    let col_range = match col_index1 {
        0..=2 => vec![0,1,2],
        3..=5 => vec![3,4,5],
        _ => vec![6,7,8],
    };

    row_range.contains(&row_index2) && col_range.contains(&col_index2)
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

fn selectable_values_in_block(cell_selectable_values: &Vec<SelectableColValues>, row_index: usize, col_index: usize) -> Vec<SelectableColValues> {
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

    let other_cell_values: Vec<SelectableColValues> = cell_selectable_values.into_iter()
        .filter(|cell_values| row_range.contains(&cell_values.row_index))
        .filter(|cell_values| col_range.contains(&cell_values.col_index))
        .filter(|x| !(x.col_index == col_index && x.row_index == row_index))
        .map(|x| x.clone())
        .collect();

    other_cell_values
}

fn is_value_contained_in_block(cell_selectable_values: &Vec<SelectableColValues>, row_index: usize, col_index: usize, value: &i32) -> bool {
    let other_cell_values: Vec<SelectableColValues> = selectable_values_in_block(cell_selectable_values, row_index, col_index);

    let mut is_contained = false;
    if other_cell_values.iter().any(|x| x.values.contains(value)) {
        is_contained = true;
    }

    is_contained
}

// いずれにしても理論: ブロックごとにセル単位ではなく列または行単位で確実に数値が入る箇所を抽出し、他のブロックの候補から消す
fn filter_anyhow_values(cell_selectable_values: &Vec<SelectableColValues>) -> Vec<SelectableColValues> {
    // ある行について、対象ブロックに複数のゼロセルがあり、それらのセルのいずれかに必ず入る値がある場合(他のブロックセルの候補に含まれない値がある場合)、いずれのセルに入るかは確定できなくても、他のブロックの同じ行のセル候補から除外する
    // 列についても同じ
    let mut cell_values_to_exclude: Vec<SelectableColValues> = Vec::new();
    for cell_values in cell_selectable_values {
        let other_selectable_values: Vec<SelectableColValues> = selectable_values_in_block(cell_selectable_values, cell_values.row_index, cell_values.col_index);
        for value in &cell_values.values {

            let is_value_in_row = other_selectable_values.iter()
                .filter(|x| x.values.contains(value))
                .all(|x| x.row_index == cell_values.row_index);
            if is_value_in_row {
                let cells_to_exclude: Vec<SelectableColValues> = cell_selectable_values.iter()
                    .filter(|x| x.row_index == cell_values.row_index)
                    .filter(|x| !is_in_same_block(x.row_index, x.col_index, cell_values.row_index, cell_values.col_index))
                    .filter(|x| x.values.contains(value))
                    .map(|x| SelectableColValues{
                        row_index: x.row_index,
                        col_index: x.col_index,
                        values: vec![value.clone()]
                    })
                    .collect();
                cell_values_to_exclude.extend_from_slice(&cells_to_exclude);
            }

            let is_value_in_col = other_selectable_values.iter()
                .filter(|x| x.values.contains(value))
                .all(|x| x.col_index == cell_values.col_index);
            if is_value_in_col {
                let cells_to_exclude: Vec<SelectableColValues> = cell_selectable_values.iter()
                    .filter(|x| x.col_index == cell_values.col_index)
                    .filter(|x| !is_in_same_block(x.row_index, x.col_index, cell_values.row_index, cell_values.col_index))
                    .filter(|x| x.values.contains(value))
                    .map(|x| SelectableColValues{
                        row_index: x.row_index,
                        col_index: x.col_index,
                        values: vec![value.clone()]
                    })
                    .collect();
                cell_values_to_exclude.extend_from_slice(&cells_to_exclude);
            }
        }
    }

    let filtered_list: Vec<SelectableColValues> = cell_selectable_values.iter()
        .map(|x| {
            let values_to_exclude: Vec<i32> = cell_values_to_exclude.iter()
                .filter(|y| y.row_index == x.row_index && y.col_index == x.col_index)
                .fold(Vec::new(), |mut acc, y| { acc.push(y.values[0]); acc });
            let filtered_values: Vec<i32> = x.values.iter()
                .filter(|y| !values_to_exclude.contains(y))
                .map(|y| *y)
                .collect();
            SelectableColValues{
                row_index: x.row_index,
                col_index: x.col_index,
                values: filtered_values,
            }
        })
        .collect();
    
    filtered_list
}

// 予約: あるブロックで2つの数値が2つのセルのどちらかにそれぞれ入り、他のセルには入らない場合、それらのセルの候補から他の数値は省いて良い。また他の候補からはそれらの数値を省いて良い。
fn filter_reserved_values(cell_selectable_values: &Vec<SelectableColValues>) -> Vec<SelectableColValues> {

    let mut cell_values_to_exclude: Vec<SelectableColValues> = Vec::new();

    // あるブロックについて、同じ2つの候補だけを持つ2つのセルがある場合、他のブロックセルの候補からその2つの数値を除外する
    for cell_values in cell_selectable_values {
        let cell_value_num = cell_values.values.len();
        let cell_value_set: HashSet<i32> = cell_values.values.iter().cloned().collect();
        let cells_with_same_values: Vec<&SelectableColValues> = cell_selectable_values.iter()
            .filter(|x| is_in_same_block(x.row_index, x.col_index, cell_values.row_index, cell_values.col_index))
            .filter(|x| !(x.row_index == cell_values.row_index && x.col_index == cell_values.col_index))
            .filter(|x| {
                let x_value_set: HashSet<i32> = x.values.iter().cloned().collect();
                let diff: HashSet<&i32> = x_value_set.difference(&cell_value_set).collect();
                diff.len() == 0
            })
            .collect();
        
        if cell_value_num == cells_with_same_values.len() {
            // 同じブロックでcell_valuesとcell_with_same_valuesに含まれないセルからcell_value_setの内容を除外する
            let cells_to_exclude: Vec<SelectableColValues> = cell_selectable_values.iter()
                .filter(|x| is_in_same_block(x.row_index, x.col_index, cell_values.row_index, cell_values.col_index))
                .filter(|x| !(x.row_index == cell_values.row_index && x.col_index == cell_values.col_index))
                .filter(|x| cells_with_same_values.iter().any(|y| !(x.row_index == y.row_index && x.col_index == y.col_index)))
                .map(|x| SelectableColValues{
                    row_index: x.row_index,
                    col_index: x.col_index,
                    values: cell_values.values.clone(),
                })
            .collect();
            cell_values_to_exclude.extend_from_slice(&cells_to_exclude);
        }
    }

    // TODO: あるブロックについて、候補に2つの同じ数値を持つ2つのセルがあり、その2つの数値が他のブロックセルに含まれない場合、その2つのセルに含まれる他の候補は除外する

    let filtered_list: Vec<SelectableColValues> = cell_selectable_values.iter()
        .map(|x| {
            let values_to_exclude: Vec<&SelectableColValues> = cell_values_to_exclude.iter()
                .filter(|y| y.row_index == x.row_index && y.col_index == x.col_index)
                .collect();
            let filtered_values: Vec<i32> = x.values.iter()
                .filter(|y| !values_to_exclude.iter().any(|z| z.values.contains(y)))
                .map(|y| *y)
                .collect();
            SelectableColValues{
                row_index: x.row_index,
                col_index: x.col_index,
                values: filtered_values,
            }
        })
        .collect();
    
    filtered_list
}

pub fn solve_problem(problem_lines: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut lines: Vec<Vec<i32>> = problem_lines.clone();
    let mut zero_count: usize = count_zero(&lines);
    let mut prev_zero_count: usize = 0;
    while zero_count > 0 && zero_count != prev_zero_count {
        prev_zero_count = zero_count;

        let mut cell_selectable_values: Vec<SelectableColValues> = fetch_usable_values(&lines);
        cell_selectable_values = filter_anyhow_values(&cell_selectable_values);
        cell_selectable_values = filter_reserved_values(&cell_selectable_values);

        let mut are_lines_updated = false;

        let unique_cell_values: Vec<SelectableColValues> = cell_selectable_values.iter()
            .filter(|x| x.values.len() == 1)
            .cloned()
            .collect();

        if unique_cell_values.len() > 0 {
            let values = &unique_cell_values[0];
            lines[values.row_index][values.col_index] = values.values[0];
            are_lines_updated = true;
            //println!("unique matched {:}-{:}", values.row_index, values.col_index);
        }

        if !are_lines_updated {
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
                        //are_lines_updated = true;
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

pub fn check_solved(lines: &Vec<Vec<i32>>) -> bool {
    let mut is_solved = true;
    'outer: for (row_index, row) in lines.iter().enumerate() {
        let row_values: HashSet<i32> = row.into_iter()
            .map(|x| *x)
            .collect();
        if row_values.len() < 9 {
            is_solved = false;
            break;
        }
        for (col_index, _) in row.iter().enumerate() {
            let col_values: HashSet<i32> = col_values(lines, col_index).into_iter().collect();
            if col_values.len() < 9 {
                is_solved = false;
                break 'outer;
            }
            let block_values: HashSet<i32> = block_values(lines, row_index, col_index).into_iter().collect();
            if block_values.len() < 9 {
                is_solved = false;
                break 'outer;
            }
        }
    }

    if count_zero(lines) != 0 {
        is_solved = false;
    }

    is_solved
}
