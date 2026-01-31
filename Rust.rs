// Rust Battleships 

// Import Standard Input/Output functions for input handling
use std::io;
// Imports for Seedable Random Number Generator
use rand::SeedableRng;
use rand::rngs::StdRng;
use rand::Rng;

// Character Array containing Tuples of Ships Symbols and their respective sizes
const SHIPS: [(char, usize); 5] = [
    ('A', 5), // Aircraft Carrier
    ('B', 4), // Battleship
    ('C', 3), // Cruiser
    ('S', 3), // Submarine
    ('D', 2), // Destroyer
];

const STUDENT_ID: u64 = 21362241; // Student ID for random seed

// Get the custom board size from the user.
fn get_board_size() -> usize {
    loop { // Loop until valid input is given
        println!("Enter the size of the board (minimum 9, maximum 20):");
        let mut input = String::new(); // Get User Input
        io::stdin().read_line(&mut input).unwrap();
        if let Ok(size) = input.trim().parse::<usize>() { // Remove whitespace and convert to integer
            if size >= 9 && size <= 20 { 
                return size; // Return if board size within range
            } 
        }
        println!("Invalid size. Please enter a number between 9 and 20."); // Else Invalid Number
    }
}

// Creates an empty board with the specified size.
fn create_board(size: usize) -> [[char; 20]; 20] {
    let mut board = [['~'; 20]; 20];
    for i in 0..size { // Iterate through column, row to set each cell to '~'
        for j in 0..size {
            board[i][j] = '~';
        }
    }
    board
}

// Place a single missile token randomly on the board.
fn place_missile_token(mut board: [[char; 20]; 20], board_size: usize) -> [[char; 20]; 20] {
    let mut rng = StdRng::seed_from_u64(STUDENT_ID); // Seed random number generator with STUID
    loop {
        let col = rng.gen_range(0..board_size);
        let row = rng.gen_range(0..board_size);
        if board[row][col] == '~' { // Check if cell is empty 
            board[row][col] = 'M'; // Place the missile token
            break;
        }
    }
    board
}

// Get and check coordinates from user 
fn get_coordinates(prompt: &str) -> Option<(usize, usize)> {
    println!("{}", prompt);
    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap(); // Get User Input
    let parts: Vec<&str> = input.trim().split_whitespace().collect(); // Remove any whitespace and split 

    if parts.len() == 2 { // Check if input contains 2 parts, 2 Coordinates
        if let (Ok(col), Ok(row)) = (parts[0].parse::<usize>(), parts[1].parse::<usize>()) { // Convert parts into integers 
            return Some((col, row)); 
        }
    }
    None
}

// Places a ship on the board manually.
fn place_ship(
    mut board: [[char; 20]; 20],
    ship_char: char,
    size: usize,
    board_size: usize
) -> [[char; 20]; 20] {
    println!("Placing ship: {} (size {})", ship_char, size);
    loop {
        println!("Enter the starting position (x y) and orientation (h for horizontal, v for vertical):"); // Prompt for coordinates and rotation

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap(); // Get coordinates and save as input
        let parts: Vec<&str> = input.trim().split_whitespace().collect(); // Split on whitespaice into seperate coordinates

        if parts.len() == 3 { // Checks 3 digits were entered
            if let (Ok(col), Ok(row)) = (parts[0].parse::<usize>(), parts[1].parse::<usize>()) { // Converts inputs from strings to integers
                let orientation = parts[2];
                let valid = match orientation {
                    "h" => col + size <= board_size && (0..size).all(|i| board[row][col + i] == '~'), // Checks if coordinates fit in boundaries and boundaries are empty
                    "v" => row + size <= board_size && (0..size).all(|i| board[row + i][col] == '~'), // Checks if coordinates fit in boundaries and boundaries are empty
                    _ => false,
                };

                if valid { // If placement is valid
                    match orientation {
                        "h" => (0..size).for_each(|i| board[row][col + i] = ship_char), // Change '~' to ship symblol
                        "v" => (0..size).for_each(|i| board[row + i][col] = ship_char), // Change '~' to ship symblol
                        _ => (),
                    }
                    break;
                } else {
                    println!("Invalid position or overlapping ships. Try again.");
                }
            }
        }
        println!("Invalid input. Try again.");
    }
    board
}

// Places all ships for a player
fn place_all_ships(mut board: [[char; 20]; 20], player: usize, board_size: usize) -> [[char; 20]; 20] {
    println!("Player {}: Place your ships.", player); // Prompt player to place ships
    for &(ship_char, size) in &SHIPS { // Loop through all ship types
        board = place_ship(board, ship_char, size, board_size); // Call place_ship for each ship
        println!("Current board after placing {}:", ship_char);
        display_board(&board, board_size); // Display board after each ship is placed
    }
    board
}

// Handles a player's move, returning the updated board, result, and whether the missile token was collected.
fn play_turn(
    mut board: [[char; 20]; 20],
    col: usize,
    row: usize,
    board_size: usize,
    use_special_missile: bool
) -> ([[char; 20]; 20], String, bool) {
    let mut missile_collected = false; // Boolean to check if missile has been collected

    if col >= board_size || row >= board_size {
        return (board, "Out of bounds!".to_string(), missile_collected);
    } // Return error if coordinates are out of bounds

    if use_special_missile { // If using special missile
        let directions = [(0, 0), (0, 1), (1, 0), (1, 1)];
        for (dx, dy) in directions { // Check each cell in 2x2
            let target_col = col.wrapping_add(dx);
            let target_row = row.wrapping_add(dy);
            if target_row < board_size && target_col < board_size {
                board[target_row][target_col] = match board[target_row][target_col] {
                    'A' | 'B' | 'C' | 'S' | 'D' => 'X', // If ships is in 2x2 mark as hit
                    '~' => 'O',                        // If empty mark miss 
                    other => other,                    // Else already attacked
                };
            }
        }
        return (board, "Special missile used!".to_string(), missile_collected); 
    }

    match board[row][col] { // Standard Attack
        'A' | 'B' | 'C' | 'S' | 'D' => {
            board[row][col] = 'X'; // If ship mark as hit
            (board, "Hit!".to_string(), missile_collected)
        }
        'M' => {
            board[row][col] = 'O';
            missile_collected = true; // If missile, missile collected updated 
            (board, "Missile collected!".to_string(), missile_collected)
        }
        '~' => {
            board[row][col] = 'O'; // If miss mark as missed
            (board, "Miss!".to_string(), missile_collected)
        }
        'X' | 'O' => (board, "Already attacked here!".to_string(), missile_collected), // If 'X' or 'O', already attsacked here
        _ => (board, "Invalid cell!".to_string(), missile_collected),
    }
}

// Displays the board.
fn display_board(board: &[[char; 20]; 20], board_size: usize) {
    for row in 0..board_size { 
        for col in 0..board_size { // Loop through row and columns of board
            print!("{} ", board[row][col]); // Print cell
        }
        println!();
    }
}

fn main() {
    println!("Welcome to Battleships!

Rules of the Game:
1. Each player takes turns to attack a coordinate on their opponent's board.
2. The goal is to sink all of your opponent's ships before they sink yours.
3. Ships are placed manually on the board at the start of the game.
4. You can collect a special missile by attacking the cell where it is randomly placed.
   - The special missile allows you to attack a 2x2 area on your opponent's board in one turn.
   - The special missile can only be used once.

Ships:
- Aircraft Carrier(A): 5 units
- Battleship(B): 4 units
- Cruiser(C): 3 units
- Submarine(S): 3 units
- Destroyer(D): 2 units
"); // Welcome message and rule explaination

    let board_size = get_board_size(); // Get custom board size

    // Create boards for both players
    let mut player1_board = create_board(board_size);
    let mut player2_board = create_board(board_size);

    // Place missile token on both boards
    player1_board = place_missile_token(player1_board, board_size);
    player2_board = place_missile_token(player2_board, board_size);

    // Place all ships manually for both players
    player1_board = place_all_ships(player1_board, 1, board_size);
    player2_board = place_all_ships(player2_board, 2, board_size);

    // Game loop
    let mut turn = 1; // Track turn
    let mut player1_attacks = create_board(board_size); // To track attacks made by Player 1
    let mut player2_attacks = create_board(board_size); // To track attacks made by Player 2
    let mut special_missile_available = [false, false]; // Tracks if each player has collected the missile

    loop {
        let current_player = if turn % 2 == 1 { 1 } else { 2 }; // Alternate between each player
        println!("Player {}'s turn.", current_player);

        let (attacks, opponent_board, missile_available) = if current_player == 1 {
            (&mut player1_attacks, &mut player2_board, &mut special_missile_available[0])
        } else {
            (&mut player2_attacks, &mut player1_board, &mut special_missile_available[1])
        };

        display_board(attacks, board_size);

        if *missile_available {
            println!("You have a special missile available!");
        }

        println!("Enter coordinates (x y):");
        if let Some((col, row)) = get_coordinates("Your move:") {
            let use_special_missile = *missile_available;
            let (updated_board, result, missile_collected) = play_turn(
                *opponent_board, col, row, board_size, use_special_missile,
            ); // Attacking player enters coordinates to attack 

            *opponent_board = updated_board;

            if use_special_missile {
                *missile_available = false; // Special missile used
            } else if missile_collected {
                *missile_available = true; // Special missile collected
            }

            attacks[row][col] = if result.contains("Hit!") || result.contains("Missile collected!") {
                'X'
            } else {
                'O'
            };

            println!("{}", result);
        } else {
            println!("Invalid input. Try again.");
            continue;
        }

        // Check win conditions for each player 
        if player2_board.iter().flatten().all(|&cell| cell != 'A' && cell != 'B' && cell != 'C' && cell != 'S' && cell != 'D') {
            println!("Player 1 wins!");
            break;
        }
        if player1_board.iter().flatten().all(|&cell| cell != 'A' && cell != 'B' && cell != 'C' && cell != 'S' && cell != 'D') {
            println!("Player 2 wins!");
            break;
        }

        turn += 1;
    }
}

