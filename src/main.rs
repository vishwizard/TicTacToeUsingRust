use std::io;

const PLAYER_X: char = 'X';
const PLAYER_O: char = 'O';

const BOARD_SIZE: usize = 3;
type Board = [[char; BOARD_SIZE]; BOARD_SIZE];

fn initialize_board() -> Board {
    return [[' '; BOARD_SIZE]; BOARD_SIZE];
}

fn print_board(board: &Board) {
    for row in board {
        for element in row {
            print!("{}", element);
        }
        println!();
    }
}

fn get_player_move(player:char,board:&Board)->(usize,usize){
    loop{
        let mut input = String::new();
        println!("Player {} make your move (row col)",player);
        let _ = io::stdin().read_line(&mut input);
        let coordinates:Vec<usize> = input.split_whitespace().flat_map(str::parse::<usize>).collect();
        if coordinates.len()==2{
            let (row,col) = (coordinates[0],coordinates[1]);
            if row<BOARD_SIZE && col<BOARD_SIZE && board[row][col]==' ' {
                return (row,col);
            }
        }
        println!("Invalid Input");
    }
    
}

fn check_draw(board : &Board)->bool {
    for row in board {
        for element in row {
            if *element==' ' {
                return false;
            }
        }
    }
    return true;
}


fn check_winner(player:char,board:&Board)->bool{
    for row in 0..BOARD_SIZE {
        if board[row][0]==player && board[row][1]==player && board[row][2]==player {
            return true;
        }
    }
    for col in 0..BOARD_SIZE {
        if board[0][col]==player && board[1][col]==player && board[2][col]==player {
            return true;
        }
    }

    if (board[0][0]==player && board[1][1]==player && board[2][2]==player) || 
    (board[0][2]==player && board[1][1]==player && board[2][0]==player) {
        return true;
    }

    return false;
}

fn play_game() {
    let mut board: Board = initialize_board();
    let mut current_player = PLAYER_X;

    loop{
        println!("Current Board : ");
        print_board(&board);
        let (row,col) = get_player_move(current_player,&board);
        board[row][col]=current_player;
        if check_winner(current_player,&board){
            print_board(&board);
            println!("Player {} is the winner",current_player);
            break;
        }
        if check_draw(&board){
            print_board(&board);
            println!("Match is drawn!");
            break;
        }
        current_player = if current_player == PLAYER_X {
            PLAYER_O
        } else {
            PLAYER_X
        }
    }
    
}

fn main() {
    println!("Welcome to Tic Tac Toe Game : ");
    play_game();
}
