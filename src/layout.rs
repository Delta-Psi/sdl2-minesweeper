use sdl2::rect::Rect;

const STATUS_BAR_HEIGHT: u32 = 24;

#[derive(Debug)]
pub struct FieldLayout {
    field_rect: Rect,
    status_bar_rect: Rect,
}

impl FieldLayout {
    pub fn new(window_size: (u32, u32), field_size: (u8, u8)) -> Self {
        let mut layout = Self {
            field_rect: Rect::new(0, 0, 1, 1),
            status_bar_rect: Rect::new(0, 0, 1, 1),
        };
        layout.recalculate(window_size, field_size);
        layout
    }

    pub fn recalculate(&mut self, window_size: (u32, u32), field_size: (u8, u8)) {
        // NOTE: only does pillarboxing
        let field_display_height = window_size.1 - STATUS_BAR_HEIGHT;
        let cell_size = field_display_height as f32 / field_size.1 as f32;
        let field_display_width = cell_size * field_size.0 as f32;

        self.field_rect = Rect::new(
            ((window_size.0 as f32 - field_display_width) / 2.0) as i32,
            STATUS_BAR_HEIGHT as i32,
            field_display_width as u32,
            field_display_height as u32,
        );
    }

    pub fn field_rect(&self) -> Rect {
        self.field_rect
    }

    pub fn status_bar_rect(&self) -> Rect {
        self.status_bar_rect
    }
}
