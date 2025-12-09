use std::collections::HashSet;
use std::time::Duration;
use crate::Position;

pub fn display_grid<'a>(collections: impl IntoIterator<Item = (&'a HashSet<Position>, char)>) {
    let collections: Vec<_> = collections.into_iter().collect();

    let all_positions: Vec<&Position> = collections
        .iter()
        .flat_map(|(set, _)| set.iter())
        .collect();

    let (max_x, max_y) = all_positions
        .iter()
        .fold((0, 0), |(mx, my), pos| { (mx.max(pos.x), my.max(pos.y)) });

    for y in 0..=max_y {
        for x in 0..=max_x {
            let pos = Position { x, y };
            let ch = collections
                .iter()
                .find(|(set, _)| set.contains(&pos))
                .map(|(_, c)| *c)
                .unwrap_or('.');
            print!("{}", ch);
        }
        println!();
    }
}

pub fn display_grid_animated<'a>(
    collections: impl IntoIterator<Item = (&'a HashSet<Position>, char)>,
    step: usize,
    delay_ms: u64
) {
    clear_screen_and_move_cursor();
    println!("Step {}", step);
    display_grid(collections);
    std::io::Write::flush(&mut std::io::stdout()).unwrap();
    std::thread::sleep(Duration::from_millis(delay_ms));
}

pub fn clear_screen_and_move_cursor() {
    print!("\x1B[2J\x1B[1;1H");
}
