pub struct Board {
    size: i8,
    board: Vec<Vec<i8>>,
    piece_taken: bool
}

impl Board {

    pub fn new() -> Board {
        let n = 8;
        Board {
            size: n,
            board: {
                // Initialize the vector to contain rows
                let mut board = Vec::new();
                // Loop through adding n rows with players on top two and bottom two
                for i in 0..n as i8 {
                    let mut row = Vec::new();
                    if i == 0 || i == 1 {
                        for j in 0..n as i8 {
                            let player = (j + i + 1) % 2;
                            row.push(player);
                        }
                    } else if i == n - 1 || i == n - 2 {
                        for j in 0..n as i8 {
                            let player = ((i + j + 1) % 2) * 2;
                            row.push(player);
                        }
                    } else {
                        for _j in 0..n {
                            row.push(0);
                        }
                    }
                    board.push(row);
                }
                // Set as board
                board
            },
            piece_taken: false
        }
    }

    pub fn king_pieces(&mut self) {
        // Loop through top and bottom rows
        for i in 0..(self.size as usize) {
            // If either side has reached the other, set to its negative
            if self.board[0][i] == 2{
                self.board[0][i] *= -1;
            }
            if self.board[self.size as usize - 1][i] == 1{
                self.board[self.size as usize - 1][i] *= -1;
            }
        }
    }

    pub fn move_piece(&mut self, row1: usize, col1: usize, row2: usize, col2: usize, player: i8) -> bool {
        // Unset piece taken flag
        self.piece_taken = false;
        // If bounds are outside of board
        if row1 >= self.size as usize || row2 >= self.size as usize || col1 >= self.size as usize || col2 >= self.size as usize || self.board[row2][col2] != 0 {
            return false;
        }
        // If the player making move isn't present here it is invalid
        if self.board[row1][col1] != player && self.board[row1][col1] != -player {
            return false;
        }
        // If the piece isn't moving 2 or 4 units (horizontal + vertical) it is invalid
        if (row1 as i8 - row2 as i8).abs() + (col1 as i8 - col2 as i8).abs() != 4 && (row1 as i8 - row2 as i8).abs() + (col1 as i8 - col2 as i8).abs() != 2 {
            return false;
        }
        // If an unkinged x moves upwards it is invalid
        if player == 1 && self.board[row1][col1] > 0 && row2 < row1 {
            return false;
        }
        // If an unkinged o moves downward it is invalid
        if player == 2 && self.board[row1][col1] > 0 && row1 < row2 {
            return false;
        }
        // See if we are checking a move to take a piece or a regular move
        if (row1 as i8 - row2 as i8).abs() + (col1 as i8 - col2 as i8).abs() == 2  {
            // If regular move, make the move and return true
            self.board[row2][col2] = self.board[row1][col1];
            self.board[row1][col1] = 0;
            return true;
        } else {
            // Otherwise make sure a piece will be taken
            // Calculate the pieces x and y
            let x = col1 + (col2 - col1) / 2;
            let y = row1 + (row2 - row1) / 2;
            // Make sure opponet is there
            if player == 1 && self.board[x][y] != 2 {
                return false;
            } else if player == 2 && self.board[x][y] != 1 {
                return false;
            }
            // Jump and return true
            self.board[x][y] = 0;
            self.board[row2][col2] = self.board[row1][col1];
            self.board[row1][col1] = 0;
            // Update piece taken flag
            self.piece_taken = true;
            return true;
        }
    }

    pub fn get_piece_taken(&self) -> bool {
        // Return if a piece was just taken
        return self.piece_taken
    }

    pub fn display(&self) {
        // Initialize result string
        let mut result: String = "┌-┬".to_string();
        // Add the upper row
        for _i in 0..(self.size * 4) as usize {
            result += "-";
        }
        // Add the corner
        result += "┐\n";
        // Loop through adding board content
        for i in 0..self.size as usize {
            let mut row: String = "".to_string();
            for j in 0..self.size as usize {
                for k in 0..4 {
                    if self.board[i][j] == 1 && (k == 1 || k == 2) {
                        row += &format!("{v}", v = 'x');
                    } else if self.board[i][j] == -1 && (k == 0 || k == 3) {
                        row += &format!("{v}", v = 'x');
                    } else if self.board[i][j] == 2 && (k == 1 || k == 2)  {
                        row += &format!("{v}", v = 'o');
                    } else if self.board[i][j] == -2 && (k == 0 || k == 3){
                        row += &format!("{v}", v = 'o');
                    } else {
                        let ch = if (i + j) % 2 != 0 { '▓' } else { '░' };
                        row.push(ch);
                    }
                }
            }
            // Add the final border
            row += "|";
            // Append the proper front to row
            for k in 0..2 {
                let ch = &format!("|{v}|", v = i + 1);
                result += if k % 2 == 0 { ch } else { "| |" };
                result += &row[..];
                result += "\n";
            }

        }
        // Add the indexes 
        result += "├";
        for k in 0..(self.size * 4 + 2) as usize {
            result += if (k % 4 != 2) || (k < 4) {"-"} else {"┬"};
        }
        result += "┤\n|   ";
        for i in 1..(self.size + 1) as usize {
            result += &format!(" {v} |", v = i);
        }
        result += "\n";
        result += "└";
        for k in 0..(self.size * 4 + 2) as usize {
            result += if (k % 4 != 2) || (k < 4) {"-"} else {"┴"};
        }
        result += "┘\n";
        // Print the result
        println!("{}", result);
    }
}