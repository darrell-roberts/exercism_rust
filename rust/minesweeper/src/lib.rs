use std::char::from_digit;

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let board: Result<Board, _> = minefield.try_into();

    match board {
        Ok(mut b) => {
            b.update_counts();
            b.into()
        }
        Err(_) => minefield.iter().map(|s| s.to_string()).collect(),
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Pos {
    x: u8,
    y: u8,
}

impl Pos {
    fn adjacent(&self, max_rows: u8, max_columns: u8) -> impl Iterator<Item = Pos> + '_ {
        let &Pos { x, y } = &self;
        let edges = |n: u8| [n - 1, n, n + 1].into_iter();

        edges(*x)
            .flat_map(move |x1| edges(*y).map(move |y1| (x1, y1)))
            .filter(move |&(x1, y1)| x1 > 0 && y1 > 0 && (x1, y1) != (*x, *y))
            .filter(move |&(x, y)| y <= max_rows && x <= max_columns)
            .map(|(x, y)| Pos { x, y })
    }
}

#[derive(Debug)]
enum CellState {
    Closed { adjacent_mines: u8 },
    Mined,
}

#[derive(Debug)]
struct Cell {
    index: usize,
    state: CellState,
    pos: Pos,
}

impl PartialEq for Cell {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index
    }
}

impl Eq for Cell {}

impl Ord for Cell {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.index.cmp(&other.index)
    }
}

impl PartialOrd for Cell {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug)]
struct Board {
    cells: Vec<Cell>,
    max_rows: u8,
    max_columns: u8,
}

impl Board {
    fn update_counts(&mut self) {
        self.cells.sort_by(|a, b| a.pos.cmp(&b.pos));
        let adjacent_indexes = self
            .cells
            .iter()
            .filter(|c| matches!(c.state, CellState::Mined))
            .flat_map(|mined| {
                mined
                    .pos
                    .adjacent(self.max_rows, self.max_columns)
                    .flat_map(|adjacent| self.cells.binary_search_by(|c| c.pos.cmp(&adjacent)))
            })
            .collect::<Vec<_>>();

        for adjacent in adjacent_indexes {
            if let Some(cell) = self.cells.get_mut(adjacent) {
                match &mut cell.state {
                    CellState::Closed { adjacent_mines } => *adjacent_mines += 1,
                    CellState::Mined => (),
                }
            }
        }
    }
}

#[derive(Debug)]
struct InvalidBoard;

impl TryFrom<&[&str]> for Board {
    type Error = InvalidBoard;

    fn try_from(value: &[&str]) -> Result<Self, Self::Error> {
        let cells = value
            .iter()
            .zip(1..)
            .flat_map(|(row, y)| row.chars().zip(1..).map(move |(c, x)| (Pos { x, y }, c)))
            .zip(1..)
            .map(|((pos, c), index)| Cell {
                index,
                pos,
                state: if c == ' ' {
                    CellState::Closed { adjacent_mines: 0 }
                } else {
                    CellState::Mined
                },
            })
            .collect::<Vec<_>>();

        let max_columns = cells.iter().map(|c| c.pos.x).max().ok_or(InvalidBoard)?;
        let max_rows = cells.iter().map(|c| c.pos.y).max().ok_or(InvalidBoard)?;

        Ok(Board {
            cells,
            max_rows,
            max_columns,
        })
    }
}

impl From<Board> for Vec<String> {
    fn from(mut value: Board) -> Self {
        value.cells.sort_unstable();
        let column_width = value.max_columns;
        let mut char_iter = value.cells.into_iter().map(|c| match c.state {
            CellState::Closed { adjacent_mines } => {
                if adjacent_mines == 0 {
                    ' '
                } else {
                    from_digit(adjacent_mines as u32, 10).unwrap_or_default()
                }
            }
            CellState::Mined => '*',
        });

        (0..)
            .map(|_| {
                char_iter
                    .by_ref()
                    .take(column_width as usize)
                    .collect::<String>()
            })
            .take_while(|s| !s.is_empty())
            .collect::<Vec<_>>()
    }
}
