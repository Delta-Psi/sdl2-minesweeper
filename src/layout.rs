use sdl2::rect::Rect;

const STATUS_BAR_HEIGHT: u32 = 32;

#[derive(Debug)]
pub struct FieldLayout {
    field_size: (u8, u8),
    field_rect: Rect,
    cell_rects: Vec<Rect>,
    status_bar_rect: Rect,
}

impl FieldLayout {
    pub fn new(window_size: (u32, u32), field_size: (u8, u8)) -> Self {
        let mut layout = Self {
            field_size,
            field_rect: Rect::new(0, 0, 1, 1),
            cell_rects: Vec::new(),
            status_bar_rect: Rect::new(0, 0, 1, 1),
        };
        layout.recalculate(window_size, field_size);
        layout
    }

    pub fn recalculate(&mut self, window_size: (u32, u32), field_size: (u8, u8)) {
        // NOTE: only does pillarboxing
        self.field_size = field_size;

        let field_display_height = window_size.1 - STATUS_BAR_HEIGHT;
        let cell_size = field_display_height as f32 / field_size.1 as f32;
        let field_display_width = cell_size * field_size.0 as f32;

        let field_display_left = (window_size.0 as f32 - field_display_width) / 2.0;
        let field_display_top = STATUS_BAR_HEIGHT as f32;
        
        self.field_rect = Rect::new(
            field_display_left as i32,
            field_display_top as i32,
            field_display_width as u32,
            field_display_height,
        );

        let boundary_x = |x| {
            (field_display_left + field_display_width*x as f32/field_size.0 as f32) as i32
        };
        let boundary_y = |y| {
            (field_display_top + field_display_height as f32*y as f32/field_size.1 as f32) as i32
        };

        self.cell_rects.clear();
        for y in 0 .. field_size.1 {
            for x in 0 .. field_size.0 {
                self.cell_rects.push(Rect::new(
                        boundary_x(x),
                        boundary_y(y),
                        (boundary_x(x+1) - boundary_x(x)) as u32,
                        (boundary_y(y+1) - boundary_y(y)) as u32,
                ));
            }
        }
    }

    pub fn field_rect(&self) -> Rect {
        self.field_rect
    }

    pub fn cell_rect(&self, cell_index: (u8, u8)) -> Rect {
        assert!(cell_index.0 < self.field_size.0);
        assert!(cell_index.1 < self.field_size.1);

        self.cell_rects[cell_index.0 as usize + self.field_size.0 as usize*cell_index.1 as usize]
    }

    pub fn status_bar_rect(&self) -> Rect {
        self.status_bar_rect
    }
}
