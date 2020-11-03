use rand::Rng;

#[derive(Debug)]
pub struct Cell {
    pub has_mine: bool,
    pub revealed: bool,
    pub flagged: bool,
    pub neighboring_mines: u8,
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            has_mine: false,
            revealed: false,
            flagged: false,
            neighboring_mines: 0,
        }
    }
}

#[derive(Debug)]
pub struct Field {
    cells: Vec<Cell>,
    width: u8,
    height: u8,

    mine_count: u16,
    flagged_cells: u16,
}

impl Field {
    pub fn new(width: u8, height: u8) -> Self {
        let mut field = Self {
            cells: Vec::new(),
            width,
            height,

            mine_count: 0,
            flagged_cells: 0,
        };
        field
            .cells
            .resize_with(width as usize * height as usize, Default::default);
        field
    }

    pub fn populate<R: Rng + ?Sized>(
        &mut self,
        mine_count: u16,
        safe_cell: Option<(u8, u8)>,
        rng: &mut R,
    ) {
        let cell_count = self.width as u16 * self.height as u16
            - self.mine_count
            - if safe_cell.is_some() { 1 } else { 0 };
        assert!(mine_count <= cell_count);

        use rand::distributions::{Distribution, Uniform};
        let x_distr = Uniform::new(0, self.width);
        let y_distr = Uniform::new(0, self.height);

        let mut remaining = mine_count;
        while remaining > 0 {
            let x = x_distr.sample(rng);
            let y = y_distr.sample(rng);
            if let Some((safe_x, safe_y)) = safe_cell {
                if x == safe_x && y == safe_y {
                    continue;
                }
            }

            let cell = self.get_cell_mut(x, y);
            if cell.has_mine {
                continue;
            }

            cell.has_mine = true;
            for x in x.saturating_sub(1)..=(x + 1).min(self.width - 1) {
                for y in y.saturating_sub(1)..=(y + 1).min(self.height - 1) {
                    self.get_cell_mut(x, y).neighboring_mines += 1;
                }
            }
            remaining -= 1;
        }
        self.mine_count += mine_count;
    }

    pub fn mine_count(&self) -> u16 {
        self.mine_count
    }

    pub fn flagged_cells(&self) -> u16 {
        self.flagged_cells
    }

    pub fn width(&self) -> u8 {
        self.width
    }

    pub fn height(&self) -> u8 {
        self.height
    }

    pub fn get_cell(&self, x: u8, y: u8) -> &Cell {
        &self.cells[self.cell_index(x, y)]
    }

    fn get_cell_mut(&mut self, x: u8, y: u8) -> &mut Cell {
        let index = self.cell_index(x, y);
        &mut self.cells[index]
    }

    fn cell_index(&self, x: u8, y: u8) -> usize {
        assert!(x < self.width);
        assert!(y < self.height);

        x as usize + y as usize * self.width as usize
    }

    /// Returns true if the cell is a mine. Doesn't
    /// modify anything and returns false if it's flagged.
    pub fn reveal(&mut self, x: u8, y: u8) -> bool {
        let cell = self.get_cell_mut(x, y);
        if cell.revealed {
            return cell.has_mine;
        }
        if cell.flagged {
            return false;
        }

        cell.revealed = true;
        if cell.has_mine {
            true
        } else if cell.neighboring_mines == 0 {
            for x in x.saturating_sub(1) ..= (x+1).min(self.width-1) {
                for y in y.saturating_sub(1) ..= (y+1).min(self.height-1) {
                    self.reveal(x, y);
                }
            }

            false
        } else {
            false
        }
    }

    /// Only performs the toggle if the cell isn't
    /// revealed.
    pub fn toggle_flag(&mut self, x: u8, y: u8) {
        let cell = self.get_cell_mut(x, y);

        if !cell.revealed {
            cell.flagged ^= true;

            if cell.flagged {
                self.flagged_cells += 1;
            } else {
                self.flagged_cells -= 1;
            }
        }
    }
}
