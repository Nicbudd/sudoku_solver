fn row_groups() -> Vec<Vec<(usize, usize)>> {
    let mut ret_vec = vec![];
    
    for row in 0..9 {
        let mut this_group = vec![];
        for column in 0..9 {
            this_group.push((row, column))
        }
        ret_vec.push(this_group);
    }

    ret_vec
}

fn column_groups() -> Vec<Vec<(usize, usize)>> {
    let mut ret_vec = vec![];
    
    for column in 0..9 {
        let mut this_group = vec![];
        for row in 0..9 {
            this_group.push((row, column))
        }
        ret_vec.push(this_group);
    }

    ret_vec
}

fn box_groups() -> Vec<Vec<(usize, usize)>> {
    let mut ret_vec = vec![];
    
    for sudoku_box in 0..9 {
        let mut this_group = vec![];
        let starting_cell = (
            ((sudoku_box * 3) / 9) * 3, // row
            (sudoku_box * 3) % 9 // column
        );

        for cell in 0..9 {
            this_group.push((
                (cell / 3) + starting_cell.0, // row
                (sudoku_box % 3) + starting_cell.1 // column
            ));
        }

        ret_vec.push(this_group);
    }

    ret_vec
}