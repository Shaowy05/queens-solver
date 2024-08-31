use regex::Regex;
use scraper::{selectable::Selectable, Html};
use std::{cmp::max, collections::HashMap};

mod html;

type SquareMap = HashMap<(usize, usize), Square>;

#[derive(Debug, Clone, Copy)]
enum Mark {
    Empty,
    Cross,
    Queen,
}

#[derive(Debug, Clone, Copy)]
struct RGB(u8, u8, u8);

#[derive(Debug, Clone, Copy)]
struct Square {
    colour: RGB,
    mark: Mark,
}

impl Square {
    fn new(colour: RGB) -> Self {
        Self {
            colour,
            mark: Mark::Empty,
        }
    }

    fn mark_with(&self, mark: Mark) -> Self {
        Self { mark, ..*self }
    }

    fn get_mark(&self) -> Mark {
        self.mark
    }
}

#[derive(Debug)]
struct Board {
    squares: SquareMap,
}

impl Board {
    fn new(squares: SquareMap) -> Self {
        Board { squares }
    }

    fn mark_square(&self, x: usize, y: usize, mark: Mark) -> Self {
        Board::new(
            self.squares
                .iter()
                .map(|(&coord, &square)| {
                    if coord == (x, y) {
                        (coord, square.mark_with(mark))
                    } else {
                        (coord, square)
                    }
                })
                .collect(),
        )
    }

    fn check_validity(&self) -> bool {
        for (&coord, &square) in self.squares.iter() {
            if let Mark::Queen = square.get_mark() {}
        }

        true
    }
}

#[derive(Debug)]
struct Game {
    boards: Vec<Board>,
}

impl Game {
    fn new(starting_board: Board) -> Self {
        Game {
            boards: vec![starting_board],
        }
    }
}

fn main() {
    use html::{BOARD_HTML, BOARD_SELECTOR, ROW_SELECTOR, TILE_SELECTOR};

    let document = Html::parse_fragment(BOARD_HTML);
    let board_selector = scraper::Selector::parse(BOARD_SELECTOR).unwrap();
    let board = document.select(&board_selector).next().unwrap();

    let row_selector = scraper::Selector::parse(ROW_SELECTOR).unwrap();
    let tile_selector = scraper::Selector::parse(TILE_SELECTOR).unwrap();

    let mut square_map: SquareMap = HashMap::new();
    let mut width = 0;
    let mut height = 0;

    for (y, row) in board.select(&row_selector).enumerate() {
        height = max(y, height);
        for (x, tile) in row.select(&tile_selector).enumerate() {
            width = max(x, width);
            let style = tile.value().attr("style").unwrap();
            let re = Regex::new(r"background-color:\s*rgb\((\d+),\s*(\d+),\s*(\d+)\)").unwrap();
            let caps = re.captures(style).unwrap();

            let r: u8 = caps.get(1).unwrap().as_str().parse().unwrap();
            let g: u8 = caps.get(2).unwrap().as_str().parse().unwrap();
            let b: u8 = caps.get(3).unwrap().as_str().parse().unwrap();
            square_map.insert((x, y), Square::new(RGB(r, g, b)));
        }
    }

    let starting_board: Board = Board::new(square_map);
    let game = Game::new(starting_board);
}
