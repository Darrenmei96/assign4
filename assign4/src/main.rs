use std::fmt::{Display, Formatter, Error};
use std::io;
use std::io::BufRead;

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

#[derive(Clone)]
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

#[derive(Clone)]
struct Player{
	name: String,
	powerups: Vec<PowerupType>,
}

impl Player{
	fn new(name: String) -> Player{
		Player{
			name: name,
			powerups: Vec::new(),
		}
	}
	fn get_name(&self) -> &str{
		&self.name
	}
}

struct Game{
	board: Vec<Cell>,
	width: u32,
	players: Vec<Player>,
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
	
	fn do_command(&mut self, line: &str){
		//Do the command given in the line
		//firstly split the string by whitespace, the first is the name of the command
		//splitting returns an iterator, so collect it into a vector of strings
		let stringvec: Vec<&str> = line.split_whitespace().collect();
		
		//Now check the first item in the vector
		match stringvec[0]{
			//if it is a board command, call the board function
			"board" => self.board(stringvec[1].parse::<u32>().unwrap(), 
								  stringvec[2].parse::<u32>().unwrap()),
			"players" => self.players(stringvec[1].parse::<u32>().unwrap()),
			"dice" => {
				let mut s = &stringvec[1..stringvec.len()];
				self.dice(s);
				},
			
								  
			_ => ()
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
	
	fn players(&mut self, player_count: u32){
		//First, make the list of players
		
		let mut player_list = Vec::new();
		let iter = (65..91).take(player_count as usize);
		for i in iter{
			player_list.push(Player::new((i as u8 as char).to_string()));
		}
		self.players = player_list;
		
		//put all the players on the first cell
		for i in 0..self.players.len(){
			let mut x = self.players[i].clone();
			self.move_to(x, 1);
		}
	}
	
	fn move_to(&mut self, mut player: Player, pos: u32){
		//if this cell is occupied
		if &(self.board[(pos-1) as usize].player[..]) != " "{
			//get the player's name
			let name = self.board[(pos-1) as usize].player.clone();
			//find that specific player
			let mut thatplayer: Player = Player::new(" ".to_string());
			for i in 0..self.players.len(){
				if self.players[i].get_name() == name{
					thatplayer = self.players[i].clone();
					break;
				}
			}
			if &thatplayer.name[..] == " "{
				//we didn't find the player for some reason
				//do nothing as we don't know how to resolve this issue
			}else{
				//clear the current board's player string value
				self.board[(pos-1) as usize].player = " ".to_string();
				//and move the player into the next cell
				self.move_to(thatplayer, pos+1);
			}
		}
		//the cell is now cleared and there is no player on this cell now
		//so let's try to land on it
		
		//place the player on to this empty cell
		//set the names
		self.board[(pos-1) as usize].player = player.name.clone();
	}
	
	fn dice(&mut self, dicevec: &mut [&str]){
 		let mut dice_roll = Vec::new();
 		for i in dicevec {
			dice_roll.push(i.parse().unwrap());
		}
 		self.dice = dice_roll;
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
	
	let stdin = io::stdin();
	let mut buffer = String::new();
	for line in stdin.lock().lines(){
		buffer.push_str(&line.unwrap());
		buffer.push_str("\n");	
	}
	let a_game = read_from(&buffer);
	println!("{}", a_game.to_string());
}



fn read_from(input : &str) -> Game {
	let mut game = Game::empty();
	for line in input.lines(){
		game.do_command(line)
	}
	game
}
