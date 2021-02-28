use rand::Rng;
use std::cmp::{max, min};
use std::fmt;
use std::io;
use std::io::Write;

#[derive(Debug, Clone, Copy)]
enum CellContents {
    Bomb,
    Neighbors(u8),
}

#[derive(Debug, Clone, Copy)]
struct Cell {
    cell_content: Option<CellContents>,
    reveale: bool,
}

impl Cell {
    fn new() -> Self {
        Cell {
            cell_content: None,
            reveale: false,
        }
    }

    fn reveale(&self) -> bool {
        self.reveale
    }

    fn set_reveale(&mut self) {
        self.reveale = true;
    }

    fn cell_content(&self) -> Option<CellContents> {
        self.cell_content
    }

    fn set_cell_content(&mut self, cell_content: Option<CellContents>) {
        self.cell_content = cell_content;
    }
}

impl std::fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let c = match self.reveale {
            true => match self.cell_content {
                Some(CellContents::Bomb) => 'B',
                Some(CellContents::Neighbors(x)) => char::from(x + b'0'),
                None => '_',
            },
            false => 'X',
        };

        write!(f, "{}", c)
    }
}

#[derive(Debug)]
struct Board {
    board: Vec<Vec<Cell>>,
}

impl Board {
    pub fn new() -> Self {
        let mut board = vec![vec![Cell::new(); 8]; 8];

        add_bombs(&mut board, 8);
        add_adjacent_bombs(&mut board);

        Board { board }
    }

    pub fn cell(&mut self, i: usize, j: usize) -> &Cell {
        &self.board[i][j]
    }

    pub fn mut_cell(&mut self, i: usize, j: usize) -> &mut Cell {
        &mut self.board[i][j]
    }

    pub fn select(&mut self, i: isize, j: isize) {
        if i < 0
            || j < 0
            || i >= self.board.len() as isize
            || j >= self.board[0].len() as isize
            || self.cell(i as usize, j as usize).reveale()
        {
            return;
        }

        self.mut_cell(i as usize, j as usize).set_reveale();

        if self.cell(i as usize, j as usize).cell_content().is_none() {
            // up
            self.select(i - 1, j);

            // up right
            self.select(i - 1, j + 1);

            // right
            self.select(i, j + 1);

            // down right
            self.select(i + 1, j + 1);

            // down
            self.select(i + 1, j);

            // down left
            self.select(i + 1, j - 1);

            // left
            self.select(i, j - 1);

            // up left
            self.select(i - 1, j - 1);
        }
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for i in 0..8 {
            for j in 0..8 {
                write!(f, "{} ", self.board[i][j])?;
            }
            write!(f, "\n")?;
        }

        write!(f, "\n")
    }
}

fn add_bombs(board: &mut Vec<Vec<Cell>>, bombs: u16) {
    let mut rng = rand::thread_rng();
    for _ in 0..bombs {
        loop {
            let i = rng.gen_range(0..board.len());
            let j = rng.gen_range(0..board[0].len());

            if board[i][j].cell_content().is_none() {
                board[i][j].set_cell_content(Some(CellContents::Bomb));
                break;
            }
        }
    }
}

fn add_adjacent_bombs(board: &mut Vec<Vec<Cell>>) {
    let width: usize = board[0].len();
    let height: usize = board.len();

    for i in 0..height {
        for j in 0..width as usize {
            match board[i][j].cell_content {
                Some(CellContents::Bomb) => {}
                _ => {
                    let mut bombs = 0;
                    for x in max(i as isize - 1, 0) as usize..min(i + 2, width) {
                        for y in max(j as isize - 1, 0) as usize..min(j + 2, height) {
                            bombs += match board[x][y].cell_content {
                                Some(CellContents::Bomb) => 1,
                                _ => 0,
                            };
                        }
                    }
                    if bombs != 0 {
                        board[i][j].set_cell_content(Some(CellContents::Neighbors(bombs)));
                    }
                }
            }
        }
    }
}

fn read_input() -> (isize, isize) {
    io::stdout().flush().unwrap();
    let mut command = String::new();
    if let Ok(_) = io::stdin().read_line(&mut command) {
        let values: Vec<&str> = command.trim().split(' ').collect();
        if let (Ok(x), Ok(y)) = (values[0].parse::<isize>(), values[1].parse::<isize>()) {
            (x, y)
        } else {
            println!("error");
            (0, 0)
        }
    } else {
        println!("error");
        (0, 0)
    }
}

fn main() {
    let mut board = Board::new();
    loop {
        println!("{}", board);
        let (i, j) = read_input();
        board.select(i, j);
    }
}
