% Board setup, Define variables as Dynamic Predicates(variables that can be changed at runtime)
:- dynamic(board_size/1). % Board Size
:- dynamic(ship_positions/1). % Ships Coordinates
:- dynamic(missile_collected/1). % Missile collected boolean
:- dynamic(hit_positions/1). % Records hits/misses

% Welcome message and instructions
welcome_message :-
    write('Welcome to Battleships!'), nl,
    write('Instructions:'), nl,
    write('1. Choose a board size to start the game by using: init_game(board_size).'), nl,
    write('2. Take shots, targeting cells by entering X, Y coordinates by using: take_shot(X, Y).'), nl,
    write('3. Hit ships to collect the special missile token (after 3 hits).'), nl,
    write('4. Use the special missile to target a cross pattern once collected by using: fire_missile(X, Y).'), nl,
    write('5. The game ends when all ships are sunk.'), nl, nl.

% Displays empty board with row, column numbers, to visualise the board
display_board(BoardSize) :-
    write('  '),
    forall(between(1, BoardSize, Col), (write(Col), write(' '))), nl,
    forall(between(1, BoardSize, Row), (
        write(Row), write(' '),
        forall(between(1, BoardSize, Col), write('. ')), nl % Loop through board and print '.' for water
    )).

% Initialise the game with custom board size and random ship placements
init_game(BoardSize) :-
    welcome_message, % Call welcome_message 
    retractall(board_size(_)), % Reset dynamic predicate/variable so values not saved from previous game 
    retractall(ship_positions(_)), % Reset dynamic predicate
    retractall(missile_collected(_)), % Reset dynamic predicate
    retractall(hit_positions(_)), % Reset dynamic predicate
    asserta(board_size(BoardSize)), % Set board size
    generate_ships(BoardSize, Ships), % Calls generate ships
    asserta(ship_positions(Ships)), % Store ships coordinates
    asserta(missile_collected(false)), % Reset missile token
    asserta(hit_positions([])), % Clear any recorded hits
    write('Game initialised with board size: '), write(BoardSize), nl, nl,
    write('Here is your game board:'), nl,
    display_board(BoardSize), nl. % Calls display board

% Generate random ship positions with a seed
generate_ships(BoardSize, Ships) :-
    Seed = 21362241,  % Use student ID as the seed
    set_random(seed(Seed)),
    findall((X, Y), (between(1, BoardSize, X), between(1, BoardSize, Y)), AllCells), % Create list of all possible cooridnates
    random_permutation(AllCells, RandomCells), % Shuffle all cells into new list
    length(Ships, 5),  % Take 5 cells
    append(Ships, _, RandomCells). % Append ships

% Take shot
take_shot(X, Y) :-
    ship_positions(Ships), % Get ship positions
    (
        member((X, Y), Ships) -> % Checks if coordinate is in the ships list
        ( % If hit 
            write('Hit!'), nl, % Print hit
            retract(ship_positions(Ships)), % Remove old list of postions 
            delete(Ships, (X, Y), NewShips), % Delete (X,Y) from list of ships
            asserta(ship_positions(NewShips)), % Update ship positions with new list 
            update_hits((X, Y)), % Add (X,Y) to list of hits 
            check_missile_token % Call check missile tokem
        );
        write('Miss!'), nl % Else print miss
    ),
    check_game_over. % Calls check_game_over after every shot

% Track coordinates of succesful hits
update_hits(Hit) :- 
    hit_positions(Hits), % Gets hit_positions list
    retract(hit_positions(Hits)), 
    asserta(hit_positions([Hit | Hits])). % Add ship coordinate at begging and replace old list

% Check if the missile token is collected after 3 hits
check_missile_token :-
    hit_positions(Hits), % Get hit list 
    length(Hits, Count), % Counts number of hits 
    Count >= 3,
    missile_collected(false), % Missile collected 
    retract(missile_collected(false)),
    asserta(missile_collected(true)), % Update missile collected
    write('Special missile token collected!'), nl.

% Fire the special missile targeting a cross pattern
fire_missile(X, Y) :-
    missile_collected(true), % Ensure missile token collected
    write('Firing special missile at: '), write(X), write(','), write(Y), nl,
    retract(missile_collected(true)),  % Reset missile token to false
    asserta(missile_collected(false)),
    board_size(Size),
    TargetCells = [(X, Y), (X, Y1), (X1, Y), (X2, Y)], % Target cells
    Y1 is max(1, Y - 1),
    X1 is max(1, X - 1),
    X2 is min(Size, X + 1),
    maplist(take_shot_cell, TargetCells).

% Helper for missile targeting
take_shot_cell((X, Y)) :-
    ship_positions(Ships),
    (
        member((X, Y), Ships) -> % Checks if coordinates are in the ships list
        (
            write('Hit at '), write(X), write(','), write(Y), nl,
            retract(ship_positions(Ships)),
            delete(Ships, (X, Y), NewShips), 
            asserta(ship_positions(NewShips)), % Remove hit ship from ship positions
            update_hits((X, Y))
        );
        true
    ).

% Check if the game is over
check_game_over :-
    ship_positions(Ships),
    (
        Ships = [] -> write('All ships sunk! Game over.'), nl; % If ships is empty = game over
        true
    ).