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



#[derive(Clone)]
pub struct Board {
    pub candidates: [u128; 9],
    cell_complete: u128,
    rules: Rules,
}

#[derive(Clone)]
pub struct Rules {
    pub normal_sudoku: bool,
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
    

        if self.is_complete(index) {

            format!("{}", BIG_CHAR[*self.candidates_vec(index).first().unwrap()][line])


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
            
            return format!(" {} {} {} ", g,h,i);
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

        let mut brd = Board { candidates: [0x000000000001FFFFFFFFFFFFFFFFFFFF; 9], cell_complete: 0, rules: rules.clone()};

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

        s = s.replace("\n", "");

        let b = Board::from_string(s, rules);

        Ok(b)

    }

    fn from_string(s: String, rules: Rules) -> Board {

        let mut b = Board::new(rules);

        b.candidates = [0; 9];

        for (idx, ch) in s.chars().enumerate() {
            if idx >= 81 {
                break;
            }

            //println!("\"{}\"", ch);

            match ch {
                '-' | '0' => {
                    for c in b.candidates.iter_mut() {
                        *c |= 1 << idx;
                    }
                },
                '1'..='9' => {
                    let digit_index = (ch.to_digit(10).unwrap() - 1) as usize;
                    b.candidates[digit_index] |= 1 << idx;
                },
                _ => {panic!("Unexpected item in bagging area")},
            }
        }

        b
    }


    fn short_string(&self) -> String {
        let mut s = String::new();
        for index in 0..81 {
            let v = self.candidates_vec(index);
            let ch = match v.len() {
                0 => 'X',
                1 => char::from_digit(*v.first().unwrap() as u32 + 1, 10).unwrap(),
                _ => '-'
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

    fn get_box(&self, row: u8, column: u8) -> (u8, u8) {
        let b1 = row / 3;
        let b2 = column / 3;
        (b1, b2)
    }





    // solve ------------------------------------------------------------------------------------------------

    pub fn is_legal(&self) -> bool {
        for set in &self.rules.sets {
            for digit in 0..9 {
                let candidates = self.candidates[digit];
                let set_candidates = candidates & set;
                if set_candidates.count_ones() == 0 { // if this digit has no candidates in the set
                    // println!("{}s: {}: set_candidates: {}", digit, pretty_print_bitmask(candidates), pretty_print_bitmask(set_candidates));
                    println!("No {} found in set {}", digit + 1, pretty_print_bitmask(*set));
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

    pub fn is_solved(&mut self) -> bool {
        self.update_cell_complete();
        self.cell_complete == 0x000000000001FFFFFFFFFFFFFFFFFFFF
    }

    pub fn update(&mut self) {
        self.update_cell_complete();
        self.update_candidates();
    }

    pub fn update_cell_complete(&mut self) {

        for index in 0..81 {

            if !self.is_complete(index) {
            
                let candidates = self.get_candidates(index);
                let pop = candidates.count_ones();
                if pop == 1 {
                    // dbg!(candidates, index);
                    self.cell_complete |= (1 << index);
                    
                }
            }

        }

    }

    fn update_candidates(&mut self) {
        for set in &self.rules.sets {

            let complete = set & self.cell_complete; // 1s where there are complete cells in this set

            for digit in 0..9 {

                let cells_containing_digit = complete & self.candidates[digit];
                let set_contains_digit = cells_containing_digit.count_ones() != 0;

                assert!(cells_containing_digit.count_ones() <= 1);

                let mut eliminate_mask = set * (set_contains_digit as u128);
                eliminate_mask &= !cells_containing_digit;

                let mask = !eliminate_mask;

                self.candidates[digit] &= mask;
            }
        }
    }


    fn optimize(&mut self) {

    }


    pub fn solve(&mut self) -> i64 {

        let mut i = 0; 
        while !self.is_solved() && i < 10000 {

            let before = self.clone();

            self.update();

            println!("{}", self.short_string());

            self.optimize();

            println!("{}", self.short_string());

            if before == *self {
                println!("{}", self);
                todo!("bifurcation")
            }

            if !self.is_legal() {
                return 0;
            }

            i += 1;
        }

        return -1;

    }

}