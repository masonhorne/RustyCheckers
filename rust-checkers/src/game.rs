pub use crate::board::Board;
pub use crate::iohandler::read_int;

pub struct Game {
    player_1_score: i8,
    player_2_score: i8,
    player_1_turn: bool,
    game_over: bool,
    board: Board
}

impl Game {
    
    pub fn new() -> Game {
        // Initialize game with requires fields
        Game {
            player_1_score: 0,
            player_2_score: 0,
            player_1_turn: true,
            game_over: false,
            board: Board::new()
        }
    }

    pub fn play(&mut self) {
        // Game Loop
        while !self.game_over {
            // Display the board
            self.board.display();
            // Prompt for proper players move
            if self.player_1_turn {
                println!("Player one's turn.");
            } else {
                println!("Player two's turn.");
            }
            // Initialize the values to make the move with
            let mut row1: i32 = 9;
            let mut col1: i32 = 9;
            let mut row2: i32 = 9;
            let mut col2: i32 = 9;
            let player: i8 = if self.player_1_turn { 1 } else { 2 };
            // Request input until a valid move is made
            let mut flag: bool = false;
            while !self.board.move_piece((row1 - 1) as usize, (col1 - 1) as usize, (row2 - 1) as usize, (col2 - 1) as usize, player) {
                if flag {
                    println!("Invalid move, please try again.")
                }
                flag = true;
                row1 = read_int("Input the pieces current row.".to_string());
                col1 = read_int("Input the pieces current column.".to_string());
                row2 = read_int("Input the row you want to move this piece to.".to_string());
                col2 = read_int("Input the column you want to move this piece to.".to_string());
            }
            // Update player move and scores based on piece taken
            if self.board.get_piece_taken() {
                if self.player_1_turn {
                    self.player_1_score += 1;
                } else {
                    self.player_2_score += 1;
                }
                if self.player_1_score == 8 || self.player_2_score == 8 {
                    self.game_over = true;
                }
            } else {
                self.player_1_turn = !self.player_1_turn;
            }
            // King pieces that reach other edge of board
            self.board.king_pieces();
        }
        // Display the final positions
        self.board.display();
        // Output the winner
        if self.player_1_score == 8 {
            println!("Player 1 wins!");
        } else {
            println!("Player 2 wins!");
        }
    }

}