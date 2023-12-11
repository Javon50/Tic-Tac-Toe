use std::io;
use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::Write;
use std::fs::File;
use std::io::BufRead;
use std::path::Path;

struct Game {
    leaderboard: HashMap<String, PlayerStats>,
    board: Vec<Vec<char>>,
    current_player: char,
    player1_marker: char,
    player2_marker: char,
    player1: String,
    player2: String,
}

struct PlayerStats {
    wins: u32,
    losses: u32,
}

impl PlayerStats {
    fn new() -> PlayerStats {
        PlayerStats { wins: 0, losses: 0 }
    }
}

impl Game {
    fn get_move(&mut self) {
        println!("Current player: {}", self.current_player_name());
        println!("Enter row (0-2) and column (0-2) separated by space:");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        let coordinates: Vec<usize> = input
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();

        if coordinates.len() == 2 {
            let (row, col) = (coordinates[0], coordinates[1]);

            match self.make_move(row, col) {
                Ok(_) => {
                    if let Some(winner) = self.check_winner() {
                        self.draw_board();
                        println!("Player {} wins!", winner);
                        return;
                    } else if self.board_full_check() {
                        self.draw_board();
                        println!("Draw");
                        return;
                    }
                }
                Err(_) => {
                    println!("Invalid move. Try again.");
                }
            }
        } else {
            println!("Invalid input. Try again.");
        }
    }

    fn set_players(&mut self, player1_marker: char, player2_marker: char, player1: String, player2: String) {
        self.player1_marker = player1_marker;
        self.player2_marker = player2_marker;
        self.player1 = player1;
        self.player2 = player2;
    }

    fn new(player1_marker: char, player2_marker: char, player1: String, player2: String) -> Game {
        let board = vec![vec![' '; 3]; 3];
        let current_player = player1_marker;
        let leaderboard = HashMap::new();

        Game { board, current_player, player1_marker, player2_marker, player1, player2, leaderboard }
    }

    fn play_game(&self) {
        println!("Welcome to Tic Tac Toe!");
        println!("{} goes first", self.player1);
    }
    
    fn make_move(&mut self, row: usize, col: usize) -> Result<(), &str> {
        if row < 3 && col < 3 && self.board[row][col] == ' ' {
            self.board[row][col] = self.current_player;
            Ok(())
        } else {
            Err("Invalid move. Try again.")
        }
    }

    fn switch_player(&mut self) {
        if self.current_player == self.player1_marker {
            self.current_player = self.player2_marker;
        } else {
            self.current_player = self.player1_marker;
        }
        self.draw_board();
    }

    fn check_winner(&self) -> Option<String> {
        let mut winner = None;
    
        for row in 0..3 {
            if self.board[row][0] != ' ' && self.board[row][0] == self.board[row][1] && self.board[row][1] == self.board[row][2] {
                winner = Some(self.current_player_name().clone());
            }
        }
    
        for col in 0..3 {
            if self.board[0][col] != ' ' && self.board[0][col] == self.board[1][col] && self.board[1][col] == self.board[2][col] {
                winner = Some(self.current_player_name().clone());
            }
        }
    
        if self.board[0][0] != ' ' && self.board[0][0] == self.board[1][1] && self.board[1][1] == self.board[2][2] {
            winner = Some(self.last_player_name().clone());
        }
        
        if self.board[0][2] != ' ' && self.board[0][2] == self.board[1][1] && self.board[1][1] == self.board[2][0] {
            winner = Some(self.last_player_name().clone());
        }
    
        winner
    }

    fn board_full_check(&self) -> bool {
        self.board.iter().all(|row| row.iter().all(|&cell| cell != ' '))
    }

    fn draw_board(&self) {
        println!();
        println!("   0   1   2  ");
        for (row_num, row) in self.board.iter().enumerate() {
            print!("{} ", row_num);
            for (col_num, &cell) in row.iter().enumerate() {
                print!(" {} ", cell);
                if col_num < 2 {
                    print!("|");
                }
            }
            println!();
            if row_num < 2 {
                println!("  ---+---+---");
            }
        }
        println!();
    }

    fn set_player_names(&mut self, player1_name: String, player2_name: String) -> Result<(), &'static str> {
        let path = Path::new("namelog.txt");
        let file = File::open(&path).expect("Failed to open namelog.txt");
        let reader = io::BufReader::new(file);
    
        for line in reader.lines() {
            let line = line.expect("Failed to read line").to_lowercase().trim().to_string();
            if line.is_empty() {
                continue;
            }
            if line == player1_name.to_lowercase() || line == player2_name.to_lowercase() {
                return Err("This name is already in use.");
            }
        }
    
        self.player1 = player1_name;
        self.player2 = player2_name;
    
        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .open("namelog.txt")
            .expect("Failed to open namelog.txt");
    
        writeln!(file, "{}", self.player1).expect("Failed to write player1 name to namelog.txt");
        writeln!(file, "{}", self.player2).expect("Failed to write player2 name to namelog.txt");
    
        Ok(())
    }

    fn update_leaderboard(&mut self, winner: &str, loser: &str) {
        let winner_stats = self.leaderboard.entry(winner.to_string()).or_insert(PlayerStats::new());
        winner_stats.wins += 1;

        let loser_stats = self.leaderboard.entry(loser.to_string()).or_insert(PlayerStats::new());
        loser_stats.losses += 1;
    }

    fn write_leaderboard(&self) -> io::Result<()> {
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .open("leaderboard.txt")?;

        let mut leaderboard: Vec<(&String, &PlayerStats)> = self.leaderboard.iter().collect();
        leaderboard.sort_by(|a, b| b.1.wins.cmp(&a.1.wins));

        for (rank, (player, stats)) in leaderboard.iter().enumerate() {
            let medal = match rank {
                0 => "🥇",
                1 => "🥈",
                2 => "🥉",
                _ => "",
            };

            writeln!(file, "{} {} - Wins: {}, Losses: {}", medal, player, stats.wins, stats.losses)?;
        }

        Ok(())
    }

}

fn main() {
    println!("Enter player 1 name:");
    let mut player1 = String::new();
    io::stdin().read_line(&mut player1).expect("Failed to read line");
    let _player1 = player1.trim();

    println!("Enter player 2 name:");
    let mut player2 = String::new();
    io::stdin().read_line(&mut player2).expect("Failed to read line");
    let _player2 = player2.trim();

    let player1_name = String::from(player1.clone());
    let player2_name = String::from(player2.clone());

    let mut game = Game::new('X', 'O', "Player 1".to_string(), "Player 2".to_string());
    game.set_players('X', 'O', player1.to_string(), player2.to_string());
    match game.set_player_names(player1_name, player2_name) {
        Ok(_) => println!("Player names set successfully."),
        Err(err) => println!("Failed to set player names: {}", err),
    }
    
    game.play_game();

    loop {
        game.draw_board();
        game.get_move();

        if let Some(winner_name) = game.check_winner() {
            let loser_name = if winner_name == game.player1 { game.player2.clone() } else { game.player1.clone() };
            game.update_leaderboard(&winner_name, &loser_name);
            match game.set_player_names(game.player1.clone(), game.player2.clone()) {
                Ok(_) => println!("Player names set successfully."),
                Err(err) => println!("Failed to set player names: {}", err),
            }
            match game.write_leaderboard() {
                Ok(_) => println!("Leaderboard updated successfully."),
                Err(e) => println!("Failed to update leaderboard: {}", e),
            }
            game.write_leaderboard().expect("Failed to write leaderboard");
            break;
        } else if game.board_full_check() {
            println!("It's a draw!");
            game.set_player_names(game.player1.clone(), game.player2.clone()).unwrap();
            break;
        }

        game.switch_player();
    
        println!("Enter row (0-2) and column (0-2) separated by space:");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        let coordinates: Vec<usize> = input
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();

        if coordinates.len() == 2 {
            let (row, col) = (coordinates[0], coordinates[1]);

            match game.make_move(row, col) {
                Ok(_) => {
                    game.switch_player();

                    if let Some(winner) = game.check_winner() {
                        game.draw_board();
                        println!("Player {} wins!", winner);
                        break;
                    } else if game.board_full_check() {
                        game.draw_board();
                        println!("Draw");
                        break;
                    }
                }
                Err(_) => {
                    println!("Invalid move. Try again.");
                }
            }
        } else {
            println!("Invalid input. Try again.");
        }
    } 
} 


