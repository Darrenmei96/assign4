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
	
	fn new_board(&mut self, width: i32, height: i32) -> Game{
		let players = self.players.clone();
		let dice = self.dice.clone();
		let cell_num = width * height;
		let mut board = HashMap::new();
		for x in 1..cell_num{
			board.insert(x, Cell::new());
		}
		Game{
			board: board,
			width: width,
			players: players,
			dice: dice,
		}
	}
	
	fn to_string(&self){
		//Convert the board to a string
		let mut placeholder = "".to_string();
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
	println!("{}", game.width);
}



fn readFrom(input : &str) -> Game {
	let game = Game::empty();
	for line in input.lines(){
		//game.do_command(line)
	}
	game
}
