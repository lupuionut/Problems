// 3484. Design Spreadsheet
// ------------------------

struct Spreadsheet {
    cells: Vec<Vec<i32>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Spreadsheet {
    fn new(rows: i32) -> Self {
        let mut cells = vec![vec![0; 26]; (rows + 1) as usize];
        Spreadsheet { cells }
    }

    fn set_cell(&mut self, cell: String, value: i32) {
        let [col, row] = self.s_to_pos(&cell);
        self.cells[row][col] = value;
    }

    fn reset_cell(&mut self, cell: String) {
        let [col, row] = self.s_to_pos(&cell);
        self.cells[row][col] = 0;
    }

    fn get_value(&self, formula: String) -> i32 {
        let mut ans = 0;
        let mut chrs = formula.chars();
        let mut parts = vec![String::new(); 2];
        let mut t = [false, false];
        let mut p = 0usize;
        while let Some(c) = chrs.next() {
            if c == '=' {
                continue;
            }
            if c == '+' {
                p += 1;
                continue;
            }
            if !c.is_ascii_digit() {
                t[p] = true;
            }
            parts[p].push(c);
        }
        match t {
            // both numbers
            [false, false] => {
                let a: i32 = parts[0].parse().unwrap();
                let b: i32 = parts[1].parse().unwrap();
                ans = a + b;
            }
            // number + cell
            [false, true] => {
                let a: i32 = parts[0].parse().unwrap();
                let [col, row] = self.s_to_pos(&parts[1]);
                ans = a + self.cells[row][col];
            }
            // cell + number
            [true, false] => {
                let a: i32 = parts[1].parse().unwrap();
                let [col, row] = self.s_to_pos(&parts[0]);
                ans = a + self.cells[row][col];
            }
            // both cells
            [true, true] => {
                let [c0, r0] = self.s_to_pos(&parts[0]);
                let [c1, r1] = self.s_to_pos(&parts[1]);
                ans = self.cells[r0][c0] + self.cells[r1][c1];
            }
        }
        ans
    }

    fn s_to_pos(&self, s: &String) -> [usize; 2] {
        let chrs = s.chars();
        let mut ans = [0; 2];
        let mut row = 0;
        chrs.enumerate().for_each(|(k, v)| {
            if k == 0 {
                ans[0] = (v as usize) - ('A' as usize)
            } else {
                row *= 10;
                row += (v as usize) - 48;
            }
        });
        ans[1] = row;
        ans
    }
}
