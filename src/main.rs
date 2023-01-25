use rand::{self, distributions::Standard, prelude::Distribution, random};
use std::{io, thread::sleep, time::Duration};
enum Cell {
    Dead,
    Alive,
}

impl Cell {
    fn to_str(&self) -> &str {
        match *self {
            Cell::Dead => ".",
            Cell::Alive => "X",
        }
    }
}

impl Distribution<Cell> for Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Cell {
        match rng.gen_range(0, 2) {
            0 => Cell::Dead,
            1 => Cell::Alive,
            _ => Cell::Dead,
        }
    }
}

struct Grid {
    rows: usize,
    cols: usize,
    grid: Vec<Vec<Cell>>,
}

impl Grid {
    fn new(rows: usize, cols: usize) -> Self {
        let mut grid: Vec<Vec<Cell>> = vec![];
        for r in 0..rows {
            grid.push(vec![]);
            for _ in 0..cols {
                let cell: Cell = random();
                grid[r].push(cell);
            }
        }
        Self { rows, cols, grid }
    }
    fn show(&self) {
        let Grid { rows, cols, grid } = self;
        let mut res = String::new();
        for r in 0..*rows {
            for c in 0..*cols {
                res.push_str(&(grid[r][c].to_str().to_owned() + " "));
            }
            res.push_str("\n");
        }
        println!("{}", res);
    }
    fn update(&mut self) {
        let Grid { rows, cols, grid } = self;
        let mut newgrid: Vec<Vec<Cell>> = vec![];
        for r in 0..*rows {
            newgrid.push(vec![]);
            for c in 0..*cols {
                newgrid[r].push(Cell::Dead);
                let mut alive = 0;
                for dr in r.saturating_sub(1)..=r + 1 {
                    for dc in c.saturating_sub(1)..=c + 1 {
                        if dr == r && dc == c {
                            continue;
                        }
                        match grid
                            .get(dr)
                            .unwrap_or(&vec![])
                            .get(dc)
                            .unwrap_or(&Cell::Dead)
                        {
                            Cell::Dead => (),
                            Cell::Alive => alive += 1,
                        }
                    }
                }
                if matches!(&grid[r][c], Cell::Alive) {
                    if alive < 2 || alive > 3 {
                        newgrid[r][c] = Cell::Dead;
                    } else {
                        newgrid[r][c] = Cell::Alive;
                    }
                } else {
                    if alive == 3 {
                        newgrid[r][c] = Cell::Alive;
                    } else {
                        newgrid[r][c] = Cell::Dead;
                    }
                }
            }
        }
        self.grid = newgrid;
    }
}

fn main() {
    let mut rows = String::new();
    let mut cols = String::new();
    let mut row_size: usize = 20;
    let mut col_size: usize = 20;
    let std = io::stdin();
    println!("please type in number of rows:");
    std.read_line(&mut rows).unwrap();
    println!("please type number of cols:");
    std.read_line(&mut cols).unwrap();
    if let Ok(i) = rows.trim().parse() {
        row_size = i;
    } else {
        println!("row input invalid, default 20 used")
    }
    if let Ok(i) = cols.trim().parse() {
        col_size = i;
    } else {
        println!("col input invalid, default 20 used")
    }
    println!("simulation will start");
    sleep(Duration::from_millis(1000));
    let mut universe = Grid::new(row_size, col_size);
    for _ in 0..200 {
        universe.show();
        universe.update();
        sleep(Duration::from_millis(50));
    }
}
