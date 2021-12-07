const BOARD_SIZE: usize = 5;

pub fn main() {
  let lines = include_str!("./input.txt").lines();

  let mut numbers: Vec<u32> = Vec::new();
  let mut boards: Vec<Board> = Vec::new();
  let mut board = create_board();
  let mut row = 0;
  for (index, line) in lines.enumerate() {
    if index == 0 {
      for num in line.split(",") {
        let num: u32 = num.parse().expect(&format!("Invalid number: {}", num));
        numbers.push(num);
      }
      continue;
    }

    if line.trim().is_empty() {
      if index > 1 {
        row = 0;
        boards.push(board);
        board = create_board();
      }
      continue;
    }

    for (i, square) in line.split_whitespace().enumerate() {
      let square: u32 = square
        .parse()
        .expect(&format!("Invalid square: {}", square));
      board.squares[row][i] = square;
    }
    row += 1;
  }
  boards.push(board);

  // println!("numbers: {:?}", numbers);

  // for (index, board) in boards.iter().enumerate() {
  //   println!("Board {}:", index+1);
  //   board.print();
  // }

  println!("\nLet the games begin!!");

  let mut winner = false;
  for number in &numbers {
    // println!("\nNumber called: {}\n", *number);
    // for (i, board) in &mut boards.iter().enumerate() {
    for board in &mut boards {
      board.mark(*number);
      // board.print();
      if board.is_winner() {
        println!("####### WINNER: {}", board.score(*number));
        board.print();
        winner = true;
        break;
      }
    }
    if winner {
      break;
    }
  }
}

#[derive(Debug)]
struct Board {
  squares: [[u32; BOARD_SIZE]; BOARD_SIZE],
  marks: [[bool; BOARD_SIZE]; BOARD_SIZE],
}

fn create_board() -> Board {
  Board {
    squares: [[0; BOARD_SIZE]; BOARD_SIZE],
    marks: [[false; BOARD_SIZE]; BOARD_SIZE],
  }
}

impl Board {
  pub fn mark(&mut self, number: u32) {
    for row in 0..BOARD_SIZE {
      for col in 0..BOARD_SIZE {
        if self.squares[row][col] == number {
          self.marks[row][col] = true;
        }
      }
    }
  }

  pub fn is_winner(&self) -> bool {
    // check rows
    for row in 0..BOARD_SIZE {
      let mut all_marked = true;
      for col in 0..BOARD_SIZE {
        all_marked &= self.marks[row][col];
      }
      if all_marked {
        return true;
      }
    }

    // check columns
    for col in 0..BOARD_SIZE {
      let mut all_marked = true;
      for row in 0..BOARD_SIZE {
        all_marked &= self.marks[row][col];
      }
      if all_marked {
        return true;
      }
    }

    return false;
  }

  pub fn score(&self, called_number: u32) -> u32 {
    let mut sum = 0;
    for row in 0..BOARD_SIZE {
      for col in 0..BOARD_SIZE {
        if !self.marks[row][col] {
          sum += self.squares[row][col];
        }
      }
    }
    return sum * called_number;
  }

  pub fn print(&self) {
    for row in 0..BOARD_SIZE {
      for col in 0..BOARD_SIZE {
        print!("{:>2}", self.squares[row][col]);
        if self.marks[row][col] {
          print!("*");
        } else {
          print!(" ");
        }
        print!(" ");
      }
      println!();
    }
    println!();
  }
}
