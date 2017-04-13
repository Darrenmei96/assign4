use std::ptr;
use std::collections::HashMap;

enum Celltype{
	Normal_cell,
	Snake_cell,
	Ladder_cell,
}

enum Poweruptype{
	Nothing,
	Doubleroll,
	Antivenom,
	Escalator,
}

struct Game{
	board: HashMap<i32,Cell>,
	width: i32,
	players: Vec<char>,
	dice: Vec<i32>,
}

impl Game{
	fn empty() -> Game{
		Game{
			board: HashMap::new(),
			width: 0,
			players: Vec::new(),
			dice: Vec::new(),
		}
	}
	
	fn board(&mut self, width: i32, height: i32){
		//Make a board with dimensions of width and height
		let cell_num = width * height;
		let mut board = HashMap::new();
		for x in 1..cell_num{
			board.insert(x, Cell::new());
		}
		self.board = board;
		self.width = width;
	}
	
	fn to_string(&self) -> String{
		//Convert the board to a string
		let mut placeholder = "".to_string();
		for i in 1..(self.board.len() as u32)/self.width{
			if i / 2 == 0{
				placeholder.push_str(self.row_to_string_reg(self.width, i));
			}else{
				placeholder.push_str(self.row_to_string_rev(self.width, i));
			}

		}
		placeholder;
	}
	
	fn row_to_string_reg(&self, width: i32, row_num: i32) -> &str{
		let peak = width * row_num;
		let base = peak - width + 1;
		let mut accumulator = "".to_string();
		
		for i in base..peak{
			let x = i as i32;
			accumulator.push_str(x.to_string());
		}
		
		accumulator.push_str("\n");
		
		accumulator.push_str("+");
		for i in base..peak{
			accumulator.push_str("---+");
		}
		accumulator;
	}
	
	fn row_to_string_rev(&self, width: i32, row_num: i32) -> &str{
		"".to_string();
	}
	
	fn do_command(&mut self, line: &str){
		
	}
}

struct Cell{
	cell_type: Celltype,
	powerup_type: Poweruptype,
	player: String,
}

impl Cell{
	fn new() -> Cell{
		Cell{
			cell_type: Celltype::Normal_cell, 
			powerup_type: Poweruptype::Nothing, 
			player: "".to_string(),
		}
	}
}

struct Player{
	name: char,
	powerups: Vec<Poweruptype>,
}


fn main(){
	let mut game = Game::empty();
	game.board(3,4);
	println!("{}", game.width);
}



fn readFrom(input : &str) -> Game {
	let game = Game::empty();
	for line in input.lines(){
		//game.do_command(line)
	}
	game
}
