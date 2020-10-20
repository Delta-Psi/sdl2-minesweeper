pub mod field;
use field::Field;

fn main() {
    const FIELD_WIDTH: u8 = 8;
    const FIELD_HEIGHT: u8 = 8;
    const MINE_COUNT: u16 = 8;

    let mut field = Field::new(FIELD_WIDTH, FIELD_HEIGHT);
    field.populate(MINE_COUNT, None, &mut rand::thread_rng());

    for x in 0..FIELD_WIDTH {
        for y in 0..FIELD_HEIGHT {
            let cell = field.get_cell(x, y);
            if cell.has_mine {
                print!("x")
            } else {
                print!("{}", cell.neighboring_mines);
            }
        }
        println!();
    }
}
