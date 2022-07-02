use std::char;
use std::io;
use std::io::Write;

enum FgColor {
    Red,
    Grey,
    None
}
enum BgColor {
    Black,
    White
}

fn main() {
    let mut board = setup_fg_board();
    let mut input = String::new();
    let mut player = 1;

    loop {
        print_board(&board);

        match check_win(&mut board) {
            Ok(n) => {
                println!("P{} wins!", n);
                break;
                },
            Err(x) => x
        }

        let display_string = "Please input P".to_string() + &player.to_string() + "'s move(A1B2): ";
        io::stdout().write(display_string.as_bytes()).expect("msg: &str");
        io::stdout().flush().expect("Unable to flush buffer");
        input.clear();

        io::stdin().read_line(&mut input).expect("Input Failed");
        input = input.trim().to_string().to_uppercase();

        let split_literal: (&str, &str);

        // Check for restart, quit
        match input.as_str() {
            "Q"|"QUIT" => break,
            "R"|"RESTART" => {board = setup_fg_board(); player = 1; continue},
            _ => {}
        }

        if input.len() <= 3 {
            println!("Invalid Input");
            continue;
        } else {
            split_literal = input.split_at(2);
        }

        let x = match parse_board_pos(&mut split_literal.0.to_string()) {
            Ok(n) => n,
            Err(n) => {println!("{}", n); continue}
        };
        let y = match parse_board_pos(&mut split_literal.1.to_string()) {
            Ok(n) => n,
            Err(n) => {println!("{}", n); continue}
        };

        match move_piece(&mut board, &x, &y, &player) {
            Ok(n) => n,
            Err(n) => {
                println!("{}", n);
                continue
            }
        };

        if player == 1 {
            player = 2;
        } else {
            player = 1;
        }
    }
}

fn color(fg: FgColor, bg: BgColor) -> String {
    let fg = match fg {
        FgColor::Red => 32,
        FgColor::Grey => 34,
        FgColor::None => 0
    }.to_string();

    let bg = match bg {
        BgColor::Black => 40,
        BgColor::White => 47
    }.to_string();

    return format!("\x1B[{};{}m", fg, bg)
}

fn setup_fg_board() -> [[i8; 8]; 8] {
    let mut board: [[i8; 8]; 8] = [[0; 8]; 8];

    //Do it the lazy way
    board[0][0] = 1;
    board[0][2] = 1;
    board[0][4] = 1;
    board[0][6] = 1;
    board[7][7] = 4;

    return board
}

fn print_board(board: &[[i8; 8]; 8]) {
    let mut letter = 'A';

    println!("  1 2 3 4 5 6 7 8");
    for i in 0..board.len() {
        print!("{}", letter);
        letter = match char::from_u32(letter as u32 + 1) {
            Some(char) => char,
            None => continue
        };

        for j in 0..board[i].len() {
            let mut bg = BgColor::Black;
            if i % 2 == 0 {
                if j % 2 == 0 {
                    bg = BgColor::White;
                }
            } else {
                if j % 2 != 0 {
                    bg = BgColor::White;
                }
            }

            match board[i][j] {
                1 => print!(" {}\u{273F}", color(FgColor::Red, bg)),
                2 => print!(" {}\u{273F}", color(FgColor::Grey, bg)),
                3 => print!(" {}\u{2742}", color(FgColor::Red, bg)),
                4 => print!(" {}\u{2742}", color(FgColor::Grey, bg)),
                _ => print!(" {} ", color(FgColor::None, bg))
            }
        }
        println!(" \x1B[0m");
    }
}

fn parse_board_pos(s: &mut String) -> Result<(i8, i8), &'static str> {
    let y = s.pop().expect("Error");
    let x = s.pop().expect("Error");

    let true_y = match x {
        'A' => 0,
        'B' => 1,
        'C' => 2,
        'D' => 3,
        'E' => 4,
        'F' => 5,
        'G' => 6,
        'H' => 7,
        _ => return Err("Invalid Input")
    };

    let true_x: i8 = match y.to_string().parse() {
        Ok(n) => n,
        _ => return Err("Invalid Input")
    };
    let true_x = match true_x {
        0 => return Err("Invalid Input"),
        _ => true_x - 1
    };

    return Ok((true_y, true_x))
}

fn move_piece(board: &mut [[i8; 8]; 8], from: &(i8, i8), to: &(i8, i8), player: &i8) -> Result<(), &'static str> {

    if player != &board[from.0 as usize][from.1 as usize] && &(player + 2) != &board[from.0 as usize][from.1 as usize] {
        return Err("Not your piece")
    }

    let mut range = -1..2;
    if player == &1 {
        range = 1..2;
    }

    for y in range {
        if from.0 + y == to.0 {
            for x in -1..2 {
                if from.1 + x == to.1 {
                    if &board[to.0 as usize][to.1 as usize] != &0 {
                        break
                    }
                    if y == 0 || x == 0 {
                        break;
                    }

                    board[to.0 as usize][to.1 as usize] = board[from.0 as usize][from.1 as usize];
                    board[from.0 as usize][from.1 as usize] = 0;

                    return Ok(())
                }
            }
        }
    }

    return Err("Invalid Movement")
}

fn check_win(board: &mut [[i8; 8]; 8]) -> Result<&'static str, ()> {

    for y in 0..board.len() {
        for x in 0..board[y].len() {
            // Check if P1 won
            if &board[y][x] == &4 &&
            get_board_value(&board, y as isize - 1, x as isize - 1) != &0 &&
            get_board_value(&board, y as isize - 1, x as isize + 1) != &0 &&
            get_board_value(&board, y as isize + 1, x as isize + 1) != &0 &&
            get_board_value(&board, y as isize + 1, x as isize - 1) != &0 {
                return Ok("1")
            }
            // Check if P2 won
            if &board[y][x] == &4 && y <= 1 {
                return Ok("2")
            }
        }
    }

    return Err(())
}

fn get_board_value(board: &[[i8; 8]; 8], posx: isize, posy: isize) -> &i8 {
    if posy < 0 || posx < 0 || posx > 7 || posy > 7 {
        return &5
    }
    return &board[posy as usize][posx as usize]

}
