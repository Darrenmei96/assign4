use std::fmt::{Display, Formatter, Error};
use std::io;
use std::io::BufRead;
use std::collections::HashMap;

#[derive(Clone, Eq, PartialEq)]
enum CellType{
	NormalCell,
	LadderCell,
	SnakeCell,
}

impl Display for CellType{
	fn fmt(&self, f:&mut Formatter) -> Result<(), Error> {
		match *self{
			CellType::NormalCell => write!(f," "),
			CellType::LadderCell => write!(f,"L"),
			CellType::SnakeCell => write!(f,"S"),
		}
	}
}

struct Cell{
	player: String,
	cell_type: CellType,
	powerup_type: PowerupType,
	offset: i32,
}

impl Cell{
	fn new() -> Cell{
		Cell{
			player: " ".to_string(),
			cell_type: CellType::NormalCell, 
			powerup_type: PowerupType::Nothing,
			offset: 0,
		}
	}
	fn new_with(player: &str, cell_type: CellType, offset: i32, powerup_type: PowerupType) -> Cell{
		Cell{
			player: player.to_string(),
			cell_type: cell_type,
			powerup_type: powerup_type,
			offset: offset,
		}
	}
}


impl Display for Cell{
	fn fmt(&self, f:&mut Formatter) -> Result<(), Error> {		
		write!(f, "|{}{}{}", &self.player, self.powerup_type.to_string(), self.cell_type.to_string());
		
		Ok(())
	}
}

#[derive(Clone, Eq, PartialEq, Hash)]
enum PowerupType{
	Antivenom,
	Nothing,
	Escalator,
	Doubleroll,
}

impl Display for PowerupType{
	fn fmt(&self, f:&mut Formatter) -> Result<(), Error> {
		match *self{
			PowerupType::Antivenom => write!(f,"a"),			
			PowerupType::Doubleroll => write!(f,"d"),
			PowerupType::Escalator => write!(f,"e"),
			PowerupType::Nothing => write!(f," "),
		}
	}
}

#[derive(Clone)]
struct Player{
	name: String,
	powerups: HashMap<PowerupType, bool>,
	position: u32,
}

impl Player{
	fn new(name: String) -> Player{
		let mut mappings = HashMap::new();
		mappings.insert(PowerupType::Antivenom, false);
		mappings.insert(PowerupType::Doubleroll, false);
		mappings.insert(PowerupType::Escalator, false);
		Player{
			name: name,
			powerups: mappings,
			position: 1,
		}
	}
	fn get_name(&self) -> &str{
		&self.name
	}
	fn get_position(&self) -> u32{
		self.position
	}
	fn set_position(&mut self, i: u32){
		self.position = i;
	}
	fn add_powerup(&mut self, powerup: PowerupType){
		//if we aren't holding it already, add it in
		if *self.powerups.get(&powerup).unwrap() == false{
			*self.powerups.get_mut(&powerup).unwrap() = true;
		}
	}
	fn consume_powerup(&mut self, powerup: PowerupType) -> bool{
		if *self.powerups.get(&powerup).unwrap() == true{
			*self.powerups.get_mut(&powerup).unwrap() = false;
			true
		}else{
			false
		}
	}
}

struct Game{
	board: Vec<Cell>,
	width: u32,
	players: Vec<Player>,
	dice: Vec<u32>,
	turns: u32,
}

impl Game{
	fn empty() -> Game{
		Game{
			board: Vec::new(),
			width: 0,
			players: Vec::new(),
			dice: Vec::new(),
			turns: 0,
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
					let mut s = &stringvec[1..];
					self.dice(s);
				}
			"snake" => self.abnormal_cell(stringvec[1].parse::<u32>().unwrap(), 
								  		   stringvec[2].parse::<u32>().unwrap()),	 
			"ladder" => self.abnormal_cell(stringvec[1].parse::<u32>().unwrap(), 
								  		   stringvec[2].parse::<u32>().unwrap()),
			"powerup" => {
				match stringvec[1]{
					"antivenom" => self.powerup_cells(PowerupType::Antivenom,&stringvec[2..]),
					"double" => self.powerup_cells(PowerupType::Doubleroll,&stringvec[2..]),
					"escalator" => self.powerup_cells(PowerupType::Escalator,&stringvec[2..]),
					_ => ()
				}			
			}
			//"turns" => self.turns(stringvec[1].parse::<u32>().unwrap()),
								  
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
	
	fn abnormal_cell(&mut self, from: u32, to: u32){
		//get the diff
		let offset = to as i32 - from as i32;
		let mut cell_type = CellType::NormalCell;
		
		if offset < 0{
			cell_type = CellType::SnakeCell;
		}else if offset > 0 {
			cell_type = CellType::LadderCell;
		}
		
		self.board[(from-1) as usize].offset = offset;
		self.board[(from-1) as usize].cell_type = cell_type;
	}
	
	fn powerup_cells(&mut self, powerup_type: PowerupType, list: &[&str]){
		for cellNum in list{
			self.board[cellNum.parse::<usize>().unwrap()-1].powerup_type = powerup_type.clone();
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
				//clear the current board cell's player string value
				self.board[(pos-1) as usize].player = " ".to_string();
				//and move the player into the next cell
				self.move_to(thatplayer, pos+1);
			}
		}
		//the cell is now cleared and there is no player on this cell now
		//so let's place the player on to this empty cell
		
		//set the names
		self.board[(pos-1) as usize].player = player.name.clone();
		//set the player's position
		player.position = pos;
		
		
		//Are there any powerups to be collected?
		if self.board[(pos-1) as usize].powerup_type != PowerupType::Nothing{
			//if there is a powerup
			//add it into the player's inventory
			player.add_powerup(self.board[(pos-1) as usize].powerup_type.clone());
		}
		
		
		//Are there any snakes or ladders to worry about?
		let offset = self.board[(pos-1) as usize].offset;
		let mut newpos = (pos as i32 + offset) as u32;
		if newpos > 0 && newpos <= self.board.len() as u32{
			if offset != 0{
				//does the player have any protection or perks that can be used?
				//check if the antivenom can be used
				if offset < 0{
					if !player.consume_powerup(PowerupType::Antivenom){
						//it returned false, we dont have the powerup -- move
						//clear the current board cell's player string value
						self.board[(pos-1) as usize].player = " ".to_string();
						//and move the player into the next cell
						self.move_to(player, newpos);
					}
				}else if offset > 0{
					if player.consume_powerup(PowerupType::Escalator){
						//it returned true, we have the escalator powerup
						//set our new position to new heights
						newpos += offset as u32;
						if newpos >= self.board.len() as u32{
							newpos = self.board.len() as u32
						}
					}
					//clear the current board cell's player string value
					self.board[(pos-1) as usize].player = " ".to_string();
					//and move the player into the next cell
					self.move_to(player, newpos);
				}else{
					//clear the current board's player string value
					self.board[(pos-1) as usize].player = " ".to_string();
					//and move the player into the next cell
					self.move_to(player, newpos);
				}
			}
		}
	}
	
	fn dice(&mut self, dicevec: &[&str]){
 		let mut dice_roll = Vec::new();
 		for i in dicevec {
			dice_roll.push(i.parse().unwrap());
		}
 		self.dice = dice_roll;
 		println!("{:?}",self.dice);
 	}
	
	fn roll(&mut self) -> u32 {
		self.dice[self.players.len() % self.dice.len()]
	}
	
	fn simulator(&self, player: Player){
		for i in player {
			self.move_to(i, player.get_position + self.roll);
		}
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
	/*
	let mut game = Game::empty();
	game.board(5,5);
	println!("{}", game.to_string());
	println!("{}", game.width);
	*/
	
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
