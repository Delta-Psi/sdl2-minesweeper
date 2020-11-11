#[derive(Debug, Clone)]
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
pub enum RevealResult {
    Nothing,
    Success(Vec<(u8, u8)>),
    Mine,
}

#[derive(Debug)]
pub enum ToggleFlagResult {
    Nothing,
    Flagged,
    Unflagged,
}

#[derive(Debug)]
pub struct Field {
    cells: Vec<Cell>,
    width: u8,
    height: u8,

    mine_count: u16,
    flagged_cells: u16,

    populated: bool,
}

impl Field {
    pub fn new(width: u8, height: u8, mine_count: u16) -> Self {
        Self {
            cells: vec![Default::default(); width as usize * height as usize],
            width,
            height,

            mine_count,
            flagged_cells: 0,

            populated: false,
        }
    }

    pub fn new_populated(width: u8, height: u8, mine_count: u16) -> Self {
        let mut field = Field::new(width, height, mine_count);
        field.populate(None);
        field
    }

    fn populate(&mut self, safe_cell: Option<(u8, u8)>) {
        let cell_count =
            self.width as u16 * self.height as u16 - if safe_cell.is_some() { 1 } else { 0 };
        assert!(self.mine_count <= cell_count);

        use rand::distributions::{Distribution, Uniform};
        let x_distr = Uniform::new(0, self.width);
        let y_distr = Uniform::new(0, self.height);
        let rng = &mut rand::thread_rng();

        let mut remaining = self.mine_count;
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

        self.populated = true;
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

    pub fn reveal(&mut self, x: u8, y: u8) -> RevealResult {
        if !self.populated {
            self.populate(Some((x, y)));
        }

        let cell = self.get_cell_mut(x, y);
        if cell.revealed || cell.flagged {
            return RevealResult::Nothing;
        }

        cell.revealed = true;
        if cell.has_mine {
            RevealResult::Mine
        } else if cell.neighboring_mines == 0 {
            let mut revealed = vec![(x, y)];
                
            for x in x.saturating_sub(1)..=(x + 1).min(self.width - 1) {
                for y in y.saturating_sub(1)..=(y + 1).min(self.height - 1) {
                    match self.reveal(x, y) {
                        RevealResult::Success(mut revealed_sub) => revealed.append(&mut revealed_sub),
                        RevealResult::Nothing => (),
                        RevealResult::Mine => unreachable!(),
                    }
                }
            }

            RevealResult::Success(revealed)
        } else {
            RevealResult::Success(vec![(x, y)])
        }
    }

    /// Only performs the toggle if the cell isn't
    /// revealed. Returns true if so.
    pub fn toggle_flag(&mut self, x: u8, y: u8) -> ToggleFlagResult {
        let cell = self.get_cell_mut(x, y);

        if !cell.revealed {
            cell.flagged ^= true;

            if cell.flagged {
                self.flagged_cells += 1;

                ToggleFlagResult::Flagged
            } else {
                self.flagged_cells -= 1;

                ToggleFlagResult::Unflagged
            }
        } else {
            ToggleFlagResult::Nothing
        }
    }
}
