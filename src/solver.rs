use std::{fmt, fs, path::Path};
use anyhow::Result;


fn pretty_print_bitmask(bits: u128) -> String {
    let rev = bits.reverse_bits() >> (128-81);
    format!("\n{:09b}\n{:09b}\n{:09b}\n{:09b}\n{:09b}\n{:09b}\n{:09b}\n{:09b}\n{:09b}\n", 
        rev >> 72 & 0x1FF,
        rev >> 63 & 0x1FF,
        rev >> 54 & 0x1FF,
        rev >> 45 & 0x1FF,
        rev >> 36 & 0x1FF,
        rev >> 27 & 0x1FF,
        rev >> 18 & 0x1FF,
        rev >> 9 & 0x1FF,
        rev & 0x1FF,
    )
}

fn get_set_indexes(set: u128) -> Vec<u8> {
    let mut v = vec![];
    for i in 0..81 {
        if (set >> i) % 2 != 0 {
            v.push(i)
        }
    }

    v
}



#[derive(Clone)]
pub struct Board {
    pub candidates: [u128; 9],
    cell_complete: u128,
    rules: Rules,
    rubiks_sets: Option<[u128; 6]> 
}

#[derive(Clone)]
pub struct Rules {
    pub normal_sudoku: bool,
    pub rubiks: bool,
    pub sets: Vec<u128>
}


impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        // this should not ultimately exist in the binary, this is just for reference
        let _ = "
        ┏━━━━━━━┯━━━━━━━┯━━━━━━━┳━━━━━━━┯━━━━━━━┯━━━━━━━┳━━━━━━━┯━━━━━━━┯━━━━━━━┓
        ┃ 1 2 3 │ 1 2 3 │ 1 2 3 ┃ 1 2 3 │ 1 2 3 │ 1 2 3 ┃ 1 2 3 │ 1 2 3 │ 1 2 3 ┃
        ┃ 4 5 6 │ 4 5 6 │ 4 5 6 ┃ 4 5 6 │ 4 5 6 │ 4 5 6 ┃ 4 5 6 │ 4 5 6 │ 4 5 6 ┃
        ┃ 7 8 9 │ 7 8 9 │ 7 8 9 ┃ 7 8 9 │ 7 8 9 │ 7 8 9 ┃ 7 8 9 │ 7 8 9 │ 7 8 9 ┃
        ┠───────┼───────┼───────╂───────┼───────┼───────╂───────┼───────┼───────┨
        ┃ 1 2 3 │ 1 2 3 │ 1 2 3 ┃ 1 2 3 │ 1 2 3 │ 1 2 3 ┃ 1 2 3 │ 1 2 3 │ 1 2 3 ┃
        ┃ 4 5 6 │ 4 5 6 │ 4 5 6 ┃ 4 5 6 │ 4 5 6 │ 4 5 6 ┃ 4 5 6 │ 4 5 6 │ 4 5 6 ┃
        ┃ 7 8 9 │ 7 8 9 │ 7 8 9 ┃ 7 8 9 │ 7 8 9 │ 7 8 9 ┃ 7 8 9 │ 7 8 9 │ 7 8 9 ┃
        ┠───────┼───────┼───────╂───────┼───────┼───────╂───────┼───────┼───────┨
        ┃ 1 2 3 │ 1 2 3 │ 1 2 3 ┃ 1 2 3 │ 1 2 3 │ 1 2 3 ┃ 1 2 3 │ 1 2 3 │ 1 2 3 ┃
        ┃ 4 5 6 │ 4 5 6 │ 4 5 6 ┃ 4 5 6 │ 4 5 6 │ 4 5 6 ┃ 4 5 6 │ 4 5 6 │ 4 5 6 ┃
        ┃ 7 8 9 │ 7 8 9 │ 7 8 9 ┃ 7 8 9 │ 7 8 9 │ 7 8 9 ┃ 7 8 9 │ 7 8 9 │ 7 8 9 ┃
        ┣━━━━━━━┿━━━━━━━┿━━━━━━━╋━━━━━━━┿━━━━━━━┿━━━━━━━╋━━━━━━━┿━━━━━━━┿━━━━━━━┫
        ┃ 1 2 3 │ 1 2 3 │ 1 2 3 ┃ 1 2 3 │ 1 2 3 │ 1 2 3 ┃ 1 2 3 │ 1 2 3 │ 1 2 3 ┃
        ┃ 4 5 6 │ 4 5 6 │ 4 5 6 ┃ 4 5 6 │ 4 5 6 │ 4 5 6 ┃ 4 5 6 │ 4 5 6 │ 4 5 6 ┃
        ┃ 7 8 9 │ 7 8 9 │ 7 8 9 ┃ 7 8 9 │ 7 8 9 │ 7 8 9 ┃ 7 8 9 │ 7 8 9 │ 7 8 9 ┃
        ┠───────┼───────┼───────╂───────┼───────┼───────╂───────┼───────┼───────┨
        ┃ 1 2 3 │ 1 2 3 │ 1 2 3 ┃ 1 2 3 │ 1 2 3 │ 1 2 3 ┃ 1 2 3 │ 1 2 3 │ 1 2 3 ┃
        ┃ 4 5 6 │ 4 5 6 │ 4 5 6 ┃ 4 5 6 │ 4 5 6 │ 4 5 6 ┃ 4 5 6 │ 4 5 6 │ 4 5 6 ┃
        ┃ 7 8 9 │ 7 8 9 │ 7 8 9 ┃ 7 8 9 │ 7 8 9 │ 7 8 9 ┃ 7 8 9 │ 7 8 9 │ 7 8 9 ┃
        ┠───────┼───────┼───────╂───────┼───────┼───────╂───────┼───────┼───────┨
        ┃ 1 2 3 │ 1 2 3 │ 1 2 3 ┃ 1 2 3 │ 1 2 3 │ 1 2 3 ┃ 1 2 3 │ 1 2 3 │ 1 2 3 ┃
        ┃ 4 5 6 │ 4 5 6 │ 4 5 6 ┃ 4 5 6 │ 4 5 6 │ 4 5 6 ┃ 4 5 6 │ 4 5 6 │ 4 5 6 ┃
        ┃ 7 8 9 │ 7 8 9 │ 7 8 9 ┃ 7 8 9 │ 7 8 9 │ 7 8 9 ┃ 7 8 9 │ 7 8 9 │ 7 8 9 ┃
        ┣━━━━━━━┿━━━━━━━┿━━━━━━━╋━━━━━━━┿━━━━━━━┿━━━━━━━╋━━━━━━━┿━━━━━━━┿━━━━━━━┫
        ┃ 1 2 3 │ 1 2 3 │ 1 2 3 ┃ 1 2 3 │ 1 2 3 │ 1 2 3 ┃ 1 2 3 │ 1 2 3 │ 1 2 3 ┃
        ┃ 4 5 6 │ 4 5 6 │ 4 5 6 ┃ 4 5 6 │ 4 5 6 │ 4 5 6 ┃ 4 5 6 │ 4 5 6 │ 4 5 6 ┃
        ┃ 7 8 9 │ 7 8 9 │ 7 8 9 ┃ 7 8 9 │ 7 8 9 │ 7 8 9 ┃ 7 8 9 │ 7 8 9 │ 7 8 9 ┃
        ┠───────┼───────┼───────╂───────┼───────┼───────╂───────┼───────┼───────┨
        ┃ 1 2 3 │ 1 2 3 │ 1 2 3 ┃ 1 2 3 │ 1 2 3 │ 1 2 3 ┃ 1 2 3 │ 1 2 3 │ 1 2 3 ┃
        ┃ 4 5 6 │ 4 5 6 │ 4 5 6 ┃ 4 5 6 │ 4 5 6 │ 4 5 6 ┃ 4 5 6 │ 4 5 6 │ 4 5 6 ┃
        ┃ 7 8 9 │ 7 8 9 │ 7 8 9 ┃ 7 8 9 │ 7 8 9 │ 7 8 9 ┃ 7 8 9 │ 7 8 9 │ 7 8 9 ┃
        ┠───────┼───────┼───────╂───────┼───────┼───────╂───────┼───────┼───────┨
        ┃ 1 2 3 │ 1 2 3 │ 1 2 3 ┃ 1 2 3 │ 1 2 3 │ 1 2 3 ┃ 1 2 3 │ 1 2 3 │ 1 2 3 ┃
        ┃ 4 5 6 │ 4 5 6 │ 4 5 6 ┃ 4 5 6 │ 4 5 6 │ 4 5 6 ┃ 4 5 6 │ 4 5 6 │ 4 5 6 ┃
        ┃ 7 8 9 │ 7 8 9 │ 7 8 9 ┃ 7 8 9 │ 7 8 9 │ 7 8 9 ┃ 7 8 9 │ 7 8 9 │ 7 8 9 ┃
        ┗━━━━━━━┷━━━━━━━┷━━━━━━━┻━━━━━━━┷━━━━━━━┷━━━━━━━┻━━━━━━━┷━━━━━━━┷━━━━━━━┛
        
        
        
        ┠───────┼───────┼───────╂───────┼───────┼───────╂───────┼───────┼───────┨
        ┃  111  │ 2222  │ 33333 ┃ 44 44 │  5555 │ 66666 ┃ 77777 │ 88888 │ 99999 ┃
        ┃   11  │  222  │   333 ┃ 44444 │  555  │ 6666  ┃    77 │  8 8  │ 99999 ┃
        ┃  1111 │  2222 │ 33333 ┃    44 │ 5555  │ 66666 ┃    77 │ 88888 │    99 ┃
        ┠───────┼───────┼───────╂───────┼───────┼───────╂───────┼───────┼───────┨
        
        ";


        for row in 0..9 {

            match row {
                0 =>           {write!(f, "┏━━━━━━━┯━━━━━━━┯━━━━━━━┳━━━━━━━┯━━━━━━━┯━━━━━━━┳━━━━━━━┯━━━━━━━┯━━━━━━━┓\n")?;}
                1|2|4|5|7|8 => {write!(f, "┠───────┼───────┼───────╂───────┼───────┼───────╂───────┼───────┼───────┨\n")?;}
                3|6 =>         {write!(f, "┣━━━━━━━┿━━━━━━━┿━━━━━━━╋━━━━━━━┿━━━━━━━┿━━━━━━━╋━━━━━━━┿━━━━━━━┿━━━━━━━┫\n")?;}
                _ => {unreachable!("got past end of for loop somehow")}
            }

            for line in 0..3 {
                write!(f, "{}", self.generate_line_from_bitmask(row, line))?;
            }
        }


        write!(f, "┗━━━━━━━┷━━━━━━━┷━━━━━━━┻━━━━━━━┷━━━━━━━┷━━━━━━━┻━━━━━━━┷━━━━━━━┷━━━━━━━┛\n")

    }
}

const BIG_CHAR: [[&'static str; 3]; 9] = [
    ["  111  ",
     "   11  ",
     "  1111 "],

    [" 2222  ",
     "  222  ",
     "  2222 "],

    [" 33333 ",
     "   333 ",
     " 33333 ",],

    [" 44 44 ",
     " 44444 ",
     "    44 "],

    ["  5555 ",
     "  555  ",
     " 5555  "],

    [" 66666 ",
     " 6666  ",
     " 66666 "],

    [" 77777 ",
     "    77 ",
     "    77 "],

    [" 88888 ",
     "  8 8  ",
     " 88888 "],

    [" 99999 ",
     " 99999 ",
     "    99 "], 
];


impl fmt::Debug for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for digit in 0..9 {
            write!(f, "\n{}: {}", digit + 1, format!("{:#083b}", self.candidates[digit]))?;
        }

        write!(f, "\ncell_complete: {}", format!("{:#083b}", self.cell_complete))
        
    }
}

impl PartialEq for Board {
    fn eq(&self, other: &Self) -> bool {
        self.candidates == other.candidates
    }
}

impl Eq for Board {}


impl Board {


    // used for fmt::Display ------------------------------------------------------------------------------------

    fn cell_candidates_partial_string(&self, row: u8, column: u8, line: usize) -> String {

        let index = self.bitmask_index(row, column);

        //println!("idx{}: {:#011b}", index, self.get_candidates(index));

        //dbg!(&self);

        let mut code = "\x1b[0;40m";

        if self.rubiks_sets.is_some() {
            for (i, set) in self.rubiks_sets.unwrap().iter().enumerate() {
                if (1 << index) & set != 0 {
                    code = match i {
                        0 => {"\x1b[41m"},
                        1 => {"\x1b[48;5;166m"},
                        2 => {"\x1b[48;5;214m\x1b[30m"},
                        3 => {"\x1b[42m"},
                        4 => {"\x1b[44m"},
                        5 => {"\x1b[47m\x1b[30m"},
                        _ => {unreachable!("more than 6 rubiks cube colors")}
                    }
                }
            }
        }


    

        if self.is_complete(index) {

            format!("{}{}\x1b[0m", code, BIG_CHAR[*self.candidates_vec(index).first().unwrap()][line])


        } else {
            let a = line * 3;
            let b = (line * 3) + 1;
            let c = (line * 3) + 2;
            
            let d = self.digit_is_candidate(index, a) as u8 * (1 + a as u8);
            let e = self.digit_is_candidate(index, b) as u8 * (1 + b as u8);
            let f = self.digit_is_candidate(index, c) as u8 * (1 + c as u8);
            
            let g = if d == 0 {' '} else {char::from_digit(d as u32, 10).unwrap()};
            let h = if e == 0 {' '} else {char::from_digit(e as u32, 10).unwrap()};
            let i = if f == 0 {' '} else {char::from_digit(f as u32, 10).unwrap()};


            return format!("{} {} {} {} \x1b[0m", code,g,h,i);
        }
    }

    fn generate_line_from_bitmask(&self, row: u8, line: usize) -> String {
        let mut s = String::new();

        for column in 0..9 {
            s.push(if column % 3 == 0 {'┃'} else {'│'});
            s.push_str(self.cell_candidates_partial_string(row, column, line).as_str());
        }

        s.push_str("┃\n");

        s
    }




    // initialization ------------------------------------------------------------------------------------------------

    pub fn new(rules: Rules) -> Board {

        let mut brd = Board { 
            candidates: [0x000000000001FFFFFFFFFFFFFFFFFFFF; 9], 
            cell_complete: 0, 
            rules: rules.clone(),
            rubiks_sets: None,
        };

        if brd.rules.normal_sudoku {

            for a in 0..9 {
                let mut row_set: u128 = 0;
                let mut col_set: u128 = 0;

                for b in 0..9 {
                    let i1 = brd.bitmask_index(a, b);
                    let i2 = brd.bitmask_index(b, a);

                    row_set |= 1 << i1;
                    col_set |= 1 << i2; 
                }

                brd.rules.sets.push(row_set);
                brd.rules.sets.push(col_set);
            }

            for a in (0..9).step_by(3) {
                for b in (0..9).step_by(3) {
                    let mut box_set: u128 = 0;

                    // iTS nOt DRY!!11!one1!!!!! - your mom wasn't dry when I saw her last night
                    box_set |= 1 << brd.bitmask_index(a,   b);
                    box_set |= 1 << brd.bitmask_index(a+1, b);
                    box_set |= 1 << brd.bitmask_index(a+2, b);
                    box_set |= 1 << brd.bitmask_index(a,   b+1);
                    box_set |= 1 << brd.bitmask_index(a+1, b+1);
                    box_set |= 1 << brd.bitmask_index(a+2, b+1);
                    box_set |= 1 << brd.bitmask_index(a,   b+2);
                    box_set |= 1 << brd.bitmask_index(a+1, b+2);
                    box_set |= 1 << brd.bitmask_index(a+2, b+2);

                    brd.rules.sets.push(box_set);
                }
            }
        }

        for set in brd.rules.sets.clone() {
            // println!("{:#083b}", set);
            // println!("{}", pretty_print_bitmask(set));
            assert_eq!(set.count_ones(), 9);
        }

        brd
    }

    pub fn from_file(file_path: &Path, rules: Rules) -> Result<Board> {
        let mut s = fs::read_to_string(file_path)?;

        let mut b = Board::from_string(s, rules);

        b.update_cell_complete();

        Ok(b)

    }

    pub fn from_string(s: String, rules: Rules) -> Board {

        let mut b = Board::new(rules);

        b.candidates = [0; 9];


        let mut idx = 0;

        let mut r_set: u128 = 0;
        let mut o_set: u128 = 0;
        let mut y_set: u128 = 0;
        let mut g_set: u128 = 0;
        let mut b_set: u128 = 0;
        let mut w_set: u128 = 0;

        for ch in s.chars() {
            if idx >= 81 {
                if b.rules.rubiks {
                    match ch {
                        'R' => {r_set |= 1 << (idx - 81);}
                        'O' => {o_set |= 1 << (idx - 81);}
                        'Y' => {y_set |= 1 << (idx - 81);}
                        'G' => {g_set |= 1 << (idx - 81);}
                        'B' => {b_set |= 1 << (idx - 81);}
                        'W' => {w_set |= 1 << (idx - 81);}
                        'X' | '*' | '.' | '_' => {},
                        _ => {continue},
                    }
                }

            } else {
                match ch {
                    '-' | '0' | '*' | '.' | '_' => {
                        for c in b.candidates.iter_mut() {
                            *c |= 1 << idx;
                        }
                    },
                    '1'..='9' => {
                        let digit_index = (ch.to_digit(10).unwrap() - 1) as usize;
                        b.candidates[digit_index] |= 1 << idx;
                    },
                    _ => {continue},
                }
    
            }

            idx += 1;

            //println!("\"{}\"", ch);

        }


        if b.rules.rubiks {
            b.rules.sets.push(r_set);
            b.rules.sets.push(o_set);
            b.rules.sets.push(y_set);
            b.rules.sets.push(g_set);
            b.rules.sets.push(b_set);
            b.rules.sets.push(w_set);

            b.rubiks_sets = Some([r_set, o_set, y_set, g_set, b_set, w_set]);
        }

        b
    }


    pub fn short_string(&self) -> String {
        let mut s = String::new();
        for index in 0..81 {
            let v = self.candidates_vec(index);
            let ch = match v.len() {
                0 => 'X',
                1 => char::from_digit(*v.first().unwrap() as u32 + 1, 10).unwrap(),
                _ => '_'
            };
            s.push(ch)
        }

        s
    }





    // utilities ------------------------------------------------------------------------------------------------

    fn bitmask_index(&self, row: u8, column: u8) -> u8 {
        (row * 9) + column 
    }

    fn rc_from_index(&self, index: u8) -> (u8, u8) {
        let row = index / 9;
        let column = index % 9;
        (row, column)
    }


    fn bitmask_get(&self, bitmask: u128, index: u8) -> bool {
        ((bitmask >> index) % 2) != 0
    }


    fn is_complete(&self, index: u8) -> bool {
        ((self.cell_complete >> index) % 2) != 0
    }

    fn digit_is_candidate(&self, index: u8, digit: usize) -> bool {
        ((self.candidates[digit as usize] >> index) % 2) != 0
    }

    fn get_candidates(&self, index: u8) -> u16 {
        let mut return_val: u16 = 0;
        for (i, mask) in self.candidates.iter().enumerate() {
            return_val |= (self.digit_is_candidate(index, i) as u16) << i;
        } 
        return_val
    }

    fn candidates_vec(&self, index: u8) -> Vec<usize> {
        let mut return_val: Vec<usize> = vec![];
        for (i, mask) in self.candidates.iter().enumerate() {
            if self.digit_is_candidate(index, i) {
                return_val.push(i);
            }
        } 
        return_val
    }

    fn find_lowest_candidates_unsolved(&self) -> (u8, u8) {
        let mut minimum: (u8, u8) = (10, 200);
                    
        for index in 0..81 {
            let candidates = self.get_candidates(index);
            let count = candidates.count_ones() as u8;
            if count > 1 && count < minimum.1 {
                minimum = (index, count);   
            }
        }

        minimum
    }

    fn get_box(&self, row: u8, column: u8) -> (u8, u8) {
        let b1 = row / 3;
        let b2 = column / 3;
        (b1, b2)
    }

    pub fn is_legal(&self) -> bool {
        for set in &self.rules.sets {
            for digit in 0..9 {
                let candidates = self.candidates[digit];
                let set_candidates = candidates & set;
                if set_candidates.count_ones() == 0 { // if this digit has no candidates in the set
                    // println!("{}s: {}: set_candidates: {}", digit, pretty_print_bitmask(candidates), pretty_print_bitmask(set_candidates));
                    // println!("No {} found in set {}", digit + 1, pretty_print_bitmask(*set));
                    return false;
                }

                let set_solved_digits = self.cell_complete & set_candidates;

                if set_solved_digits.count_ones() > 1 { // if there's more than one "solved" number in here.
                    // println!("No {} found in set {}", digit + 1, pretty_print_bitmask(*set));
                    return false;
                }
            }
        }

        // probably isn't necessary but whatever.
        for index in 0..81 {
            if self.get_candidates(index).count_ones() == 0 {
                return false;
            }
        }

        true
    }

    pub fn is_solved(&self) -> bool {
        self.cell_complete == 0x000000000001FFFFFFFFFFFFFFFFFFFF
    }


    pub fn set_cell(&mut self, digit: usize, index: u8) {
        for d in 0..9 {
            if d == digit {
                self.candidates[d] |= 1 << index; 
            } else {
                self.candidates[d] &= !(1 << index);
            }
        }
    }




    // update ------------------------------------------------------------------------------------------------

    pub fn update_cell_complete(&mut self) {
        for index in 0..81 {
            if !self.is_complete(index) {
                let candidates = self.get_candidates(index);
                
                let pop = candidates.count_ones();
                if pop == 1 {self.cell_complete |= (1 << index);}
            }
        }
    }

    fn update_candidates(&mut self) {
        for set in &self.rules.sets {

            let complete = set & self.cell_complete; // 1s where there are complete cells in this set

            for digit in 0..9 {
                let cells_containing_digit = complete & self.candidates[digit];
                let set_contains_digit = cells_containing_digit.count_ones() != 0;

                //assert!(cells_containing_digit.count_ones() <= 1);

                let mut eliminate_mask = set * (set_contains_digit as u128);
                eliminate_mask &= !cells_containing_digit;

                let mask = !eliminate_mask;

                self.candidates[digit] &= mask;

            }
        }
    }





    // solve ---------------------------------------------------------------------------------------------

    fn optimize(&mut self) {
        if self.hidden_naked_singles_pairs_triples() {return}

    }

    fn hidden_singles(&mut self) -> bool {
        let mut change = false; 

        let mut masks = [u128::MAX; 9];
        
        for set in &self.rules.sets {

            for (d, c) in self.candidates.iter().enumerate() {
                let set_incomplete_candidates = *c & set & !self.cell_complete;

                if set_incomplete_candidates.count_ones() == 1 { // we have a hidden single

                    let idxs = get_set_indexes(set_incomplete_candidates);
                    let idx = idxs.first().unwrap();
                    
                    for m in masks.iter_mut() {
                        *m &= !(1 << idx); // remove all the candidates for this index
                    }

                    masks[d] |= 1 << idx; // add back all other candidates

                    change = true;
                }
            }
        }

        for (d, c) in self.candidates.iter_mut().enumerate() {
            *c &= masks[d];
        }

        change    
    }


    fn hidden_naked_singles_pairs_triples(&mut self) -> bool {
        let mut changed = false;

        let mut masks = [u128::MAX; 9];

        for set in self.rules.sets.clone() {
            let set_candidates = self.candidates.iter().map(|x| x & set).enumerate();
            
            let one_candidates   = set_candidates.clone().filter(|x| x.1.count_ones() == 1);
            let two_candidates: Vec<(usize, u128)>   = set_candidates.clone().filter(|x| x.1.count_ones() == 2).collect();
            let three_candidates: Vec<(usize, u128)> = set_candidates.clone().filter(|x| x.1.count_ones() == 3).collect();

            // find hidden or naked singles
            for c in one_candidates {
                let eliminate = set ^ c.1;
                let allow = !eliminate;
                masks[c.0] &= allow;
                changed = true;
                // println!("eliminated {} from {}\n{}", c.0 + 1, pretty_print_bitmask(eliminate), self)
            }


            // // find hidden or naked singles
            // for (i, c) in 0..two_candidates.len() {
            //     let eliminate = set ^ c.1;
            //     let allow = !eliminate;
            //     masks[c.0] &= allow;
            //     changed = true;
            //     // println!("eliminated {} from {}\n{}", c.0 + 1, pretty_print_bitmask(eliminate), self)
            // }
            

        }


        for (d, c) in self.candidates.iter_mut().enumerate() {
            *c &= masks[d];
        }

        changed
    }


    pub fn solve(&mut self, recursion_count: &mut u128, stop_if_bifurcate: bool) -> u128 {

        // println!("{}", self.short_string());

        if *recursion_count >= 10000 { // break out of this if it gets too crazy
            return 3;
        }

        let mut i = 0; 

        while i < 10000 {

            let before = self.clone();

            if !self.is_legal() {
                //dbg!(0);
                return 0;
            } else if self.is_solved() {
                //dbg!(1);
                return 1;
            } 

            self.update_cell_complete();
            self.update_candidates();

            if !self.is_legal() {
                //dbg!(0);
                return 0;
            } else if self.is_solved() {
                //dbg!(1);
                return 1;
            } 

            // println!("{}", self.short_string());

            if before == *self {

                self.optimize();
                //println!("{}", self.short_string());

                if before == *self {

                    if stop_if_bifurcate {
                        return 2;
                    }

                    let (index, count) = self.find_lowest_candidates_unsolved();
                    let candidates = self.candidates_vec(index);

                    let mut potential_solution: Board = Board::new(self.rules.clone()); 
                    let mut solutions_found = 0;

                    for c in &candidates {

                        for count in 0..(candidates.len() as u128 - 1) {
                            if recursion_count.count_ones() == 1 {
                                //println!("Recursion: x{}", recursion_count)
                            }
                            *recursion_count += 1;
                        }

                        let mut new_sudoku = self.clone();
                        new_sudoku.set_cell(*c, index);

                        let result = new_sudoku.solve(recursion_count, stop_if_bifurcate);

                        solutions_found += result;

                        // dbg!(result, solutions_found);

                        if result == 1 {
                            potential_solution = new_sudoku;
                        } else if result > 1 || solutions_found > 1 { // if multiple legal solutions were found then return everything early
                            return 2;
                        }
                    }

                    match solutions_found {
                        0 => {return 0},
                        1 => {*self = potential_solution;return 1}
                        _ => {return 2}
                    }
                    


                }
            }


            i += 1;
        
        }

        panic!("Too many iterations")
    }

}