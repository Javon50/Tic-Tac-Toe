use std::io;
use std::fs;
use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::Write;
use std::fs::File;
use std::io::BufRead;
use std::path::Path;
use std::io::BufReader;
use std::collections::HashSet;
use std::error::Error;

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
    #[allow(dead_code)]
    fn new() -> PlayerStats {
        PlayerStats { wins: 0, losses: 0 }
    }
}

impl Game {
    fn get_move(&mut self) {
        println!("Current player: {}", self.current_player_name());
        println!("Enter row (0-2) and column (0-2) separated by space:");

        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {},
            Err(e) => {
                println!("Failed to read line: {}", e);
                return;
            }
        }

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
    
    fn current_player_name(&self) -> &str {
        if self.current_player == self.player1_marker {
            &self.player1
        } else {
            &self.player2
        }
    }

    fn last_player_name(&self) -> &str {
        if self.current_player == self.player1_marker {
            &self.player2
        } else {
            &self.player1
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
                winner = Some(self.current_player_name().to_string()); 
            }
        }
    
        for col in 0..3 {
            if self.board[0][col] != ' ' && self.board[0][col] == self.board[1][col] && self.board[1][col] == self.board[2][col] {
                winner = Some(self.current_player_name().to_string());
            }
        }
    
        if self.board[0][0] != ' ' && self.board[0][0] == self.board[1][1] && self.board[1][1] == self.board[2][2] {
            winner = Some(self.last_player_name().to_string());
        }
        
        if self.board[0][2] != ' ' && self.board[0][2] == self.board[1][1] && self.board[1][1] == self.board[2][0] {
            winner = Some(self.last_player_name().to_string());
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

    fn set_player_names(&mut self, player1: String, player2: String) -> Result<(), Box<dyn Error>> {
        let namelog = fs::read_to_string("namelog.txt")?;
        let names: Vec<String> = namelog
            .lines()
            .filter(|line| !line.trim().is_empty())
            .map(|line| line.trim().to_string())
            .collect();
    
        let player1_trimmed = player1.trim().to_string();
        let player2_trimmed = player2.trim().to_string();
    
        if names.contains(&player1_trimmed) || names.contains(&player2_trimmed) {
            return Err("Name already logged.".into());
        }
    
        let mut file = OpenOptions::new().append(true).open("namelog.txt")?;
        writeln!(file, "{}", player1_trimmed)?;
        writeln!(file, "{}", player2_trimmed)?;
    
        self.player1 = player1_trimmed.clone();
        self.player2 = player2_trimmed.clone();
    
        self.leaderboard.entry(player1_trimmed.clone()).or_insert(PlayerStats { wins: 0, losses: 0 });
        self.leaderboard.entry(player2_trimmed.clone()).or_insert(PlayerStats { wins: 0, losses: 0 });
    
        Ok(())
    }

    fn check_player_names(&self, player1: &str, player2: &str) -> Result<(), Box<dyn Error>> {
        let namelog = fs::read_to_string("namelog.txt")?;
        let names: HashSet<String> = namelog
            .lines()
            .filter(|line| !line.trim().is_empty())
            .map(|line| line.trim().to_string())
            .collect();
    
        let player1_trimmed = player1.trim().to_string();
        let player2_trimmed = player2.trim().to_string();
    
        if !names.contains(&player1_trimmed) || !names.contains(&player2_trimmed) {
            return Err("Name not found.".into());
        }
    
        Ok(())
    }

    fn update_leaderboard(&mut self, winner: &str, loser: &str) -> Result<(), Box<dyn Error>> {
        let winner_stats = self.leaderboard.entry(winner.to_string()).or_insert(PlayerStats { wins: 0, losses: 0 });
        winner_stats.wins += 1;
    
        let loser_stats = self.leaderboard.entry(loser.to_string()).or_insert(PlayerStats { wins: 0, losses: 0 });
        loser_stats.losses += 1;
    
        Ok(())
    }

    fn load_leaderboard(&mut self) -> io::Result<()> {
        let path = Path::new("leaderboard.txt");
        let file = File::open(&path)?;
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let line = line?;
            let parts: Vec<&str> = line.split(',').collect();
            if parts.len() < 3 {
                continue;
            }
            let name = parts[0].to_string();
            let wins = parts[1].parse().unwrap_or(0);
            let losses = parts[2].parse().unwrap_or(0);
            self.leaderboard.insert(name, PlayerStats { wins, losses });
        }

        Ok(())
    }

    fn write_leaderboard(&self) -> io::Result<()> {
        let mut leaderboard: Vec<(&String, &PlayerStats)> = self.leaderboard.iter().collect();
        leaderboard.sort_by(|a, b| b.1.wins.cmp(&a.1.wins));
    
        let mut file = File::create("leaderboard.txt")?;
    
        // Write the header
        writeln!(file, "{:<5}{:<10}{:<10}{:<10}", "Rank", "Name", "Wins", "Losses")?;
    
        // Write the leaderboard
        for (rank, (name, stats)) in leaderboard.iter().enumerate() {
            let name = name.replace("\n", "").replace("\r", ""); // Remove newline characters from the name
            writeln!(file, "{:<5}{:<10}{:<10}{:<10}", rank + 1, name, stats.wins, stats.losses)?;
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
    let mut game_over = false; 



    let mut game = Game::new('X', 'O', "Player 1".to_string(), "Player 2".to_string());
    game.set_players('X', 'O', player1.to_string(), player2.to_string());

    match game.check_player_names(&player1_name, &player2_name) {
        Ok(_) => println!("Player names checked successfully."),
        Err(err) => println!("Failed to check player names: {}", err),
    }
    match game.set_player_names(player1.clone(), player2.clone()) {
        Ok(_) => println!("Player names set successfully."),
        Err(err) => println!("Failed to set player names: {}", err),
    }
    
    game.play_game();

    'game_loop: loop {
        game.draw_board();
        game.get_move();
    
        if let Some(winner_name) = game.check_winner() {
            let loser_name = if winner_name == game.player1 { game.player2.clone() } else { game.player1.clone() };
            match game.update_leaderboard(&winner_name, &loser_name){
                Ok(_) => println!("Leaderboard updated successfully."),
                Err(e) => println!("Failed to update leaderboard: {}", e),
            }
            match game.write_leaderboard() {
                Ok(_) => {},
                Err(e) => println!("Failed to write leaderboard: {}", e),
            }
            match game.load_leaderboard() {
                Ok(_) => {},
                Err(e) => println!("Failed to load leaderboard: {}", e),
            }
            break 'game_loop; 
        } else {
            println!("No winner yet.");
        }
    
        if game.board_full_check() {
            break 'game_loop;
        }
    
        game.switch_player();
    
        println!("Enter row (0-2) and column (0-2) separated by space:");
    
        let mut input = String::new();
    
        match io::stdin().read_line(&mut input) {
            Ok(_) => {},
            Err(e) => {
                println!("Failed to read line: {}", e);
                continue;
            }
        }

        let coordinates: Vec<usize> = input
            .trim()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

            if coordinates.len() == 2 {
                let (row, col) = (coordinates[0], coordinates[1]);
                match game.make_move(row, col) {
                    Ok(_) => {
                        // Check for a winner immediately after a move is made
                        if let Some(winner) = game.check_winner() {
                            game.draw_board();
                            println!("Player {} wins!", winner);
            
                            let _ = game.update_leaderboard(&winner, &game.current_player.to_string());
            
                            match game.write_leaderboard() {
                                Ok(_) => println!("Leaderboard updated successfully."),
                                Err(e) => println!("Failed to update leaderboard: {}", e),
                            }
            
                            if game_over {
                                break 'game_loop; 
                            }
                        }
            
                        // Check if the game is over immediately after a move is made
                        if game_over {
                            break; // Break out of the main game loop if the game is over
                        }
            
                        // Switch players only if the game is not over
                        game.switch_player();
                    }
                    Err(_) => {
                        println!("Invalid move. Try again.");
                    }
                }
            }
            
            if game.board_full_check() {
                game.draw_board();
                println!("Draw");
                game_over = true; 
            }
            
            if game_over {
                break 'game_loop; // Break out of the main game loop if the game is over
            }
    }
}