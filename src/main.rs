use std::{char, collections::HashMap, sync::mpsc::{channel, Receiver}, thread::{self, sleep}, time::Duration};

use ncurses::{addstr, cbreak, clear, curs_set, getch, getmaxx, getmaxy, initscr, noecho, refresh, stdscr};

type Grid = Vec<(u8, u8)>;

fn iteration(g: &Grid) -> Grid {
    g.iter()
     .flat_map(move |(x, y)| (-1..=1).flat_map(move |dx| (-1..=1).map(move |dy| ((*x as i16 + dx).max(0) as u8, (*y as i16 + dy).max(0) as u8))))
     // [(x + dx, y + dy) | dx <- [-1..1], dy <- [-1..1]]
     .fold(HashMap::new(),
           |mut acc: HashMap<(u8, u8), u8>, x|
           { acc.entry(x).and_modify(|n| *n += 1).or_insert(1); acc })
     .iter()
     .filter_map(|(p, n)| { if *n == 3 || *n == 4 { Some(*p) } else { None } })
     .collect::<Vec<(u8, u8)>>()
}

fn listen_on() -> Receiver<char> {
    let (tx, rx) = channel();
    thread::spawn(move || loop {
        tx.send(char::from_u32(getch() as u32).unwrap()).unwrap();
    });
    rx
}

fn main() {
    let mut grid: Grid = vec!((10, 10), (10, 11), (10, 12));
    initscr();
    noecho();
    cbreak();
    curs_set(ncurses::CURSOR_VISIBILITY::CURSOR_INVISIBLE);
    let here = listen_on();
    loop {
        grid = iteration(&grid);
        clear();
        match here.try_recv() {
            Ok(ch) => addstr(&format!("{}\n", ch)),
            _ => addstr("type something you idiot\n"),
        };

        for x in 0..u8::try_from(getmaxx(stdscr())).unwrap() - 1 {
            for y in 0..u8::try_from(getmaxy(stdscr())).unwrap() {
                addstr(if grid.contains(&(x, y)) { "#" } else { " " } );
            }
            addstr("\n");
        }

        refresh();
        sleep(Duration::from_secs(2));
    }
}
