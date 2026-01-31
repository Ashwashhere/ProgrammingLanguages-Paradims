# Python Battleships

# Random Number Generator for Special Missile
import random


# Board Class
class Board:
    # Initialise board
    def __init__(self, size):
        self.size = size
        self.grid = [["~" for _ in range(size)] for _ in range(size)] # Poplate a grid of '~' for water 
        self.special_token_placed = False # Boolean to track special missile token placed
        self.seed = 21362241
        self.place_special_token()

    # Display board method 
    def display(self, hide_ships=False):
        print("  " + " ".join(map(str, range(self.size))))
        for i, row in enumerate(self.grid): # Loop through board
            if hide_ships:
                row_display = ["~" if cell == "S" else cell for cell in row]
                print(f"{i} " + " ".join(row_display)) # Print board with ships
            else:
                print(f"{i} " + " ".join(row)) # Print board without ships
          
    # Place ship method
    def place_ship(self, x, y, length, orientation):
        if orientation == 'H':
            if y + length > self.size: # Check if ship fits on board
                return False
            if any(self.grid[x][y + i] != "~" for i in range(length)): # Check if cells are empty
                return False
            for i in range(length):
                self.grid[x][y + i] = "S" # Change '~' to 'S' for ship 
        elif orientation == 'V':
            if x + length > self.size: # Check if ship fits on board
                return False
            if any(self.grid[x + i][y] != "~" for i in range(length)): # Check if cells are empty
                return False
            for i in range(length):
                self.grid[x + i][y] = "S" # Place ship
        return True
      
    # Place missile token
    def place_special_token(self):
        random.seed(self.seed)
        while not self.special_token_placed: # While not already placed
            x, y = random.randint(0, self.size - 1), random.randint(0, self.size - 1) # Generate random coordinates 
            if self.grid[x][y] == "~": 
                self.grid[x][y] = "T" # Place token if cell is empty
                self.special_token_placed = True # Update token tracker boolean

    # Take shot method
    def take_shot(self, x, y):
        if self.grid[x][y] == "S":
            self.grid[x][y] = "X" # If ship mark cell hit 
            print("Hit!")
            return "hit"
        elif self.grid[x][y] == "T":
            self.grid[x][y] = "O" # If T mark as O and update missile token boolean
            print("You collected the special missile!")
            return "token"
        elif self.grid[x][y] == "~":
            self.grid[x][y] = "O" # If empty mark as miss
            print("Miss!")
            return "miss"
        else:
            print("Already targeted this spot.")
            return "repeat"


    # Special missile method
    def special_missile(self, x, y):
        hits = 0 # Track hits
        for dx in range(2):
            for dy in range(2):
                nx, ny = x + dx, y + dy # Get 2x2 cell coordinates
                if 0 <= nx < self.size and 0 <= ny < self.size: # Check if coordinates are in bounds
                    if self.take_shot(nx, ny) == "hit": # Check for hit
                        hits += 1
        print(f"Special missile hit {hits} target(s)!")

    # All ships sunk method 
    def all_ships_sunk(self):
        return all(cell != "S" for row in self.grid for cell in row) # Check each cell, if not ship cells then all ships sunk

# Game Class
class Game:
    # Initialise game
    def __init__(self):
        while True:
            self.size = int(input("Enter board size (between 9 and 20): ")) # Get board size from player
            if 9 <= self.size <= 20:
                break
            print("Invalid size. Please enter a number between 9 and 20.")
        self.players = ["Player 1", "Player 2"] 
        self.boards = {player: Board(self.size) for player in self.players} # Set up board for each player
        self.ships = {"Aircraft Carrier": 5, "Battleship": 4, "Cruiser": 3, "Submarine": 3, "Destroyer": 2}
        self.special_missile_available = {player: False for player in self.players} 

    # Print rules
    def print_rules(self):
        print("""
        Welcome to Battleships!
        Rules:
        1. Place your ships, each player has 5 Ships: Aircraft Carrier (5), Battleship (4), Cruiser (3), Submarine (3), Destroyer (2).
        2. Each Player takes in it in turns to take shots at the others ships. 
        4. The special missile can target a 2x2 area but is only available after collecting the special token.
        5. First player to sink all the others ships wins!
        """)

    # Place ships for each player
    def place_ships(self, player):
        print(f"{player}, place your ships!")
        for ship, length in self.ships.items(): # Loop through ships
            while True:
                print(f"Place your {ship} (length: {length})")
                x, y = map(int, input("Enter starting coordinates (x y): ").split()) # Get coordinates from player
                orientation = input("Enter orientation (H for horizontal, V for vertical): ").strip().upper() # Get orientation from player
                if self.boards[player].place_ship(x, y, length, orientation): # Check for valid placement
                    break
                print("Invalid placement. Try again.")

    # Play turn method 
    def play_turn(self, current_player, opponent):
        print(f"{current_player}'s turn!")
        self.boards[opponent].display(hide_ships=True) # Display board while hiding ships
        missile = input("Choose missile (normal/special): ").lower() # Choose missile 

        if missile == "normal":
            x, y = map(int, input("Enter coordinates (x y): ").split()) # Get coordinates 
            result = self.boards[opponent].take_shot(x, y) # Call take shot
            if result == "token":
                self.special_missile_available[current_player] = True  

        elif missile == "special" and self.special_missile_available[current_player]: 
            x, y = map(int, input("Enter coordinates for special missile (x y): ").split()) # Get coordinates for the special missile
            self.boards[opponent].special_missile(x, y) # Fire special missile
            self.special_missile_available[current_player] = False

        elif missile == "special": # If token not collected 
            print("Special missile not available yet! Collect the token first.")

        else:
            print("Invalid choice. Try again.")

    # Game loop
    def play(self):
        self.print_rules()
        for player in self.players:
            self.place_ships(player) # Place ships for each player

        current_player_index = 0 # Track current player
        while not any(self.boards[player].all_ships_sunk() for player in self.players): # While game all ships are'nt sunk for each player
            current_player = self.players[current_player_index]
            opponent = self.players[1 - current_player_index]
            self.play_turn(current_player, opponent)

            if self.boards[opponent].all_ships_sunk(): # If all ships sunk, game over
                print(f"{current_player} wins! Congratulations!") 
                break

            current_player_index = 1 - current_player_index  # Switch turn


game = Game()
game.play()