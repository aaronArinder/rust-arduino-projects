extern crate pancurses;

use pancurses::{endwin, initscr};

fn main() {
    let window = initscr();
    window.printw("Hello Rust");
    window.refresh();
    window.getch();
    endwin();
}
