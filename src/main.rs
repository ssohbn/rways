use rand::prelude::*;

const WIDTH: usize = 15;
const HEIGHT: usize = 15;
const CHANCE: u32 = 8;

fn main() {
    let mut board = new_board(WIDTH, HEIGHT);

    if CHANCE != 0 {
        for _ in 0..((WIDTH * HEIGHT) as u32 / CHANCE) {
            let mut rng = rand::thread_rng();
            let x: u32  = rng.gen_range(0..(WIDTH as u32));
            let y: u32  = rng.gen_range(0..(WIDTH as u32));
            birth(&mut board, x, y)
        }
    }

    let (life, death) = check_cells(&board);
    let (updated_life, updated_death) = purge(&mut board, life, death);
    draw_board(&board, updated_life, updated_death);

}

fn draw_board( cells: &Vec<Vec<Cell>>, _updated_life: Vec<(u32, u32)>, _updated_death: Vec<(u32, u32)>) {
    for row in cells {
        for cell in row {

            if matches!(cell.state, CellState::ALIVE) {
                print!("#", )
            } else {
                print!(" ", )

            }
        }

        println!();
    }

}

fn check_cells(cells: &Vec<Vec<Cell>>) -> (Vec<(u32,u32)>, Vec<(u32, u32)>) {
    let mut life: Vec<(u32, u32)> = Vec::new();
    let mut death: Vec<(u32, u32)> = Vec::new();
    
    for row in cells {
        for cell in row {
            if cell.neighbors == 3 {
                life.push((cell.x, cell.y));
            } else if matches!(cell.state, CellState::ALIVE) {
                if cell.neighbors == 2 {
                    life.push((cell.x, cell.y))
                } else {
                    death.push((cell.x, cell.y));
                }
            }
        }
    }

    (life, death)
}

fn birth(cells: &mut Vec<Vec<Cell>>, x: u32, y: u32) {
    if matches!(cells.get(y as usize).unwrap().get(x as usize).unwrap().state, CellState::DEAD) {
        for neighbor_pos in neighboring_positions(x, y, cells.get(y as usize).unwrap().len(), cells.len()) {
            let nx = neighbor_pos.0;
            let ny = neighbor_pos.1;

            let (nx, ny) = wrap_board(nx, ny, cells.len(), cells.get(0).unwrap().len());
            cells.get_mut(ny as usize).unwrap().get_mut(nx as usize).unwrap().inc_neighbors();
        }
    }

    cells.get_mut(y as usize).unwrap().get_mut(x as usize).unwrap().state = CellState::ALIVE;
}

fn kill(cells: &mut Vec<Vec<Cell>>, x: u32, y: u32) {
    if matches!(cells.get(y as usize).unwrap().get(x as usize).unwrap().state, CellState::ALIVE) {
        for neighbor_pos in neighboring_positions(x, y, cells.get(y as usize).unwrap().len(), cells.len()) {
            let nx = neighbor_pos.0;
            let ny = neighbor_pos.1;

            let (nx, ny) = wrap_board(nx, ny, cells.len(), cells.get(0).unwrap().len());
            cells.get_mut(ny as usize).unwrap().get_mut(nx as usize).unwrap().dec_neighbors();
        }
    }

    cells.get_mut(y as usize).unwrap().get_mut(x as usize).unwrap().state = CellState::DEAD;
}

fn purge(cells: &mut Vec<Vec<Cell>>, life: Vec<(u32, u32)>,  death: Vec<(u32, u32)>) -> (Vec<(u32, u32)>, Vec<(u32, u32)>){
    let mut changed_life: Vec<(u32, u32)> = Vec::new();
    let mut changed_death: Vec<(u32, u32)> = Vec::new();

    for cell in death {
        let x = cell.0;
        let y = cell.1;
        if matches!(cells.get(y as usize).unwrap().get(x as usize).unwrap().state, CellState::ALIVE) {
            changed_death.push((x, y));
        }

        kill(cells, x, y)

    }

    for cell in life {
        let x = cell.0;
        let y = cell.1;

        if matches!(cells.get(y as usize).unwrap().get(x as usize).unwrap().state, CellState::DEAD) {
            changed_life.push((x, y));
        }

        birth(cells, x, y)
    }

    (changed_life, changed_death)
}

fn new_board(width: usize, height: usize) -> Vec<Vec<Cell>> {
    let mut board = Vec::new();

    for y in 0..height {
        let mut row = Vec::new();
        for x in 0..width {
            row.push( Cell::new(x as u32, y as u32, 0, CellState::DEAD) );
    
        };
        board.push(row);
    };

    board
}

fn wrap_board(x: u32, y: u32, width: usize, height: usize) -> (u32, u32) {
    let mut x = x;
    let mut y = y;

    if x == width as u32 {
        x = 0
    } else if x == 0 {
        x = (width-1) as u32
    }

    if y == height as u32 {
        y = 0
    } else if y == 0 {
        y = (height-1) as u32
    }

    (x, y)
}

fn neighboring_positions(x: u32, y: u32, width: usize, height: usize) -> Vec<(u32, u32)> {
    let (x, y) = wrap_board(x, y, width, height);

    vec![
        (x-1, y-1),(x, y-1),(x+1, y-1),
        (x-1, y  ),         (x+1, y  ),
        (x-1, y+1),(x, y+1),(x+1, y+1),
    ]
}

#[derive(Debug)]
struct Cell {
    x: u32,
    y: u32,
    neighbors: u32,
    state: CellState,
}

impl Cell {
    fn new(x: u32, y: u32, neighbors: u32, state: CellState) -> Cell {
        Cell { x , y, neighbors, state }
    }

    fn read_coords(&self) -> (u32, u32){
        (self.x, self.y)
    }

    fn inc_neighbors(&mut self) {
        self.neighbors +=1 ;
    }

    fn dec_neighbors(&mut self) {
        self.neighbors -= 1;
    }
}

#[derive(Debug)]
enum CellState {
    ALIVE,
    DEAD,
}
