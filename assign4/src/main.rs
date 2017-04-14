use std::fmt::{Display, Formatter, Error};
use std::ptr;
use std::collections::HashMap;

enum CellType{
	NormalCell,
	SnakeCell,
	LadderCell,
}

impl Display for CellType{
	fn fmt(&self, f:&mut Formatter) -> Result<(), Error> {
		match *self{
			CellType::NormalCell => write!(f," "),
			CellType::SnakeCell => write!(f,"S"),
			CellType::LadderCell => write!(f,"L")
		}
	}
}

enum PowerupType{
	Nothing,
	Doubleroll,
	Antivenom,
	Escalator,
}

impl Display for PowerupType{
	fn fmt(&self, f:&mut Formatter) -> Result<(), Error> {
		match *self{
			PowerupType::Nothing => write!(f," "),
			PowerupType::Doubleroll => write!(f,"d"),
			PowerupType::Antivenom => write!(f,"a"),
			PowerupType::Escalator => write!(f,"e")
		}
	}
}

struct Cell{
	player: String,
	cell_type: CellType,
	powerup_type: PowerupType,
}

impl Cell{
	fn new() -> Cell{
		Cell{
			player: " ".to_string(),
			cell_type: CellType::NormalCell, 
			powerup_type: PowerupType::Nothing,
		}
	}
	fn new_with(player: &str, cell_type: CellType, powerup_type: PowerupType) -> Cell{
		Cell{
			player: player.to_string(),
			cell_type: cell_type,
			powerup_type: powerup_type,
		}
	}
}


impl Display for Cell{
	fn fmt(&self, f:&mut Formatter) -> Result<(), Error> {		
		write!(f, "|{}{}{}", &self.player, self.cell_type.to_string(), self.powerup_type.to_string());
		
		Ok(())
	}
}


struct Player{
	name: char,
	powerups: Vec<PowerupType>,
}

struct Game{
	board: Vec<Cell>,
	width: u32,
	players: Vec<char>,
	dice: Vec<u32>,
}

impl Game{
	fn empty() -> Game{
		Game{
			board: Vec::new(),
			width: 0,
			players: Vec::new(),
			dice: Vec::new(),
		}
	}
	
	fn board(&mut self, width: u32, height: u32){
		//Make a board with dimensions of width and height
		let cell_num = width * height;
		let mut board = Vec::new();
		for x in 0..cell_num{
			board.push(Cell::new());
		}
		self.board = board;
		self.width = width;
	}
	
	fn to_string(&self) -> String{
		//Convert the board to a string
		let mut placeholder = String::from(get_grid(self.width));
		let height = ((self.board.len()+1) as u32)/self.width;
		//println!("{}", height);
		for i in (1..height+1).rev() {
			if i % 2 != 0{
				placeholder.push_str(&(self.row_to_string_reg(self.width, i)));
			}else{
				placeholder.push_str(&(self.row_to_string_rev(self.width, i)));
			}

		}
		placeholder
	}
	
	fn row_to_string_reg(&self, width: u32, row_num: u32) -> String{
		let peak = width * row_num;
		let base = peak - width + 1;
		let mut accumulator = String::new();
		
		for i in base..peak+1{
			let x = format!("|{:3}", i);
			accumulator.push_str(&x);
		}
		accumulator.push_str("|\n");
		
		for i in base..peak+1{
			accumulator.push_str(&(self.board[(i-1) as usize].to_string()))
		}
		accumulator.push_str("|\n");
		
		accumulator.push_str(&get_grid(width));
		accumulator
	}
	
	fn row_to_string_rev(&self, width: u32, row_num: u32) -> String{
		let peak = width * row_num;
		let base = peak - width + 1;
		let mut accumulator = String::new();
		
		for i in (base..peak+1).rev(){
			let x = format!("|{:3}", i);
			accumulator.push_str(&x);
		}
		accumulator.push_str("|\n");
		
		for i in (base..peak+1).rev(){
			accumulator.push_str(&(self.board[(i-1) as usize].to_string()));
		}
		accumulator.push_str("|\n");
		
		accumulator.push_str(&get_grid(width));
		accumulator
	}
	
	fn do_command(&mut self, line: &str){
		
	}
}

fn get_grid(width: u32) -> String{
	let mut accumulator = String::from("+");
	for i in 0..width{
		accumulator.push_str("---+");
	}
	accumulator.push_str("\n");
	accumulator
}

fn main(){
	let mut game = Game::empty();
	game.board(5,5);
	println!("{}", game.to_string());
	println!("{}", game.width);
}



fn readFrom(input : &str) -> Game {
	let game = Game::empty();
	for line in input.lines(){
		//game.do_command(line)
	}
	game
}
