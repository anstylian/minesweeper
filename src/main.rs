use rand::Rng;
use std::cmp::{max, min};
use std::fmt;
use std::io;
use std::io::Write;

#[derive(PartialEq)]
enum GameState {
    Cont,
    Win,
    Lost,
}

impl std::fmt::Display for GameState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GameState::Cont => write!(f, "Continue"),
            GameState::Win => write!(f, "Win"),
            GameState::Lost => write!(f, "Lost"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum CellContents {
    Bomb,
    Neighbors(u8),
}

#[derive(Debug, Clone, Copy)]
struct Cell {
    cell_content: Option<CellContents>,
    reveal: bool,
}

impl Cell {
    fn new() -> Self {
        Cell {
            cell_content: None,
            reveal: false,
        }
    }

    fn reveal(&self) -> bool {
        self.reveal
    }

    fn set_reveal(&mut self) {
        self.reveal = true;
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
        let c = match self.reveal {
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
    hidden_cells: u32,
}

impl Board {
    pub fn new(size: usize, bombs: u32) -> Self {
        let mut board = vec![vec![Cell::new(); size]; size];

        add_bombs(&mut board, bombs);
        add_adjacent_bombs(&mut board);

        Board {
            board,
            hidden_cells: (size * size) as u32 - bombs,
        }
    }

    pub fn cell(&self, i: usize, j: usize) -> &Cell {
        &self.board[i][j]
    }

    pub fn mut_cell(&mut self, i: usize, j: usize) -> &mut Cell {
        &mut self.board[i][j]
    }

    pub fn reveal_cell(&mut self, i: usize, j: usize) {
        self.mut_cell(i, j).set_reveal();
        self.hidden_cells -= 1;
    }

    pub fn reveal(&mut self) {
        let width = self.board.len() as usize;
        let hight = self.board[0].len() as usize;
        for i in 0..width {
            for j in 0..hight {
                self.mut_cell(i, j).set_reveal();
            }
        }
    }

    pub fn play_one_round(&mut self, i: isize, j: isize) -> GameState {
        self.select(i, j);
        self.game_state(self.cell(i as usize, j as usize))
    }

    fn game_state(&self, cell: &Cell) -> GameState {
        if let Some(x) = cell.cell_content() {
            if x == CellContents::Bomb {
                return GameState::Lost;
            }
        }

        if self.hidden_cells == 0 {
            GameState::Win
        } else {
            GameState::Cont
        }
    }

    fn select(&mut self, i: isize, j: isize) {
        if i < 0
            || j < 0
            || i >= self.board.len() as isize
            || j >= self.board[0].len() as isize
            || self.cell(i as usize, j as usize).reveal()
        {
            return;
        }

        self.reveal_cell(i as usize, j as usize);

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
        let width = self.board.len() as usize;
        let hight = self.board[0].len() as usize;
        for i in 0..hight {
            for j in 0..width {
                write!(f, "{} ", self.board[i][j])?;
            }
            writeln!(f)?;
        }

        writeln!(f)
    }
}

fn add_bombs(board: &mut Vec<Vec<Cell>>, bombs: u32) {
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
        for j in 0..width {
            match board[i][j].cell_content {
                Some(CellContents::Bomb) => {}
                _ => {
                    let bombs = add_cell_adjacent_bombs(board, i, j);

                    if bombs != 0 {
                        board[i][j].set_cell_content(Some(CellContents::Neighbors(bombs)));
                    }
                }
            }
        }
    }
}

fn add_cell_adjacent_bombs(board: &mut Vec<Vec<Cell>>, i: usize, j: usize) -> u8 {
    let width: usize = board[0].len();
    let height: usize = board.len();
    let mut bombs = 0;

    for line in board
        .iter()
        .take(min(i + 2, width))
        .skip(max(i as isize - 1, 0) as usize)
    {
        for cell in line
            .iter()
            .take(min(j + 2, height))
            .skip(max(j as isize - 1, 0) as usize)
        {
            bombs += match cell.cell_content {
                Some(CellContents::Bomb) => 1,
                _ => 0,
            };
        }
    }

    bombs
}

fn read_input() -> (isize, isize) {
    io::stdout().flush().unwrap();
    let mut command = String::new();
    if io::stdin().read_line(&mut command).is_ok() {
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
    let mut board = Board::new(8, 8);
    let mut game_state = GameState::Cont;

    while game_state == GameState::Cont {
        println!("{}", board);
        let (i, j) = read_input();
        game_state = board.play_one_round(i, j);
    }

    board.reveal();
    println!("{}", board);
    println!("Game finished: {}", game_state);
}
