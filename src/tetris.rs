pub mod game {
    use crate::pieces;
    use crate::pieces::Move;
    use crate::utils::{clamp_over, out_of_bounds};
    use std::io::{self, Read};
    use std::io::{stdin, stdout, Stdin, Stdout, Write};
    use std::sync::{Arc, Mutex};
    use termion::event::{Event, Key};
    use termion::input::TermRead;
    use termion::raw::{IntoRawMode, RawTerminal};
    use termion::{color, style};

    // pub const EMPTY_BLOCK: char = '▢';
    // pub const FILLED_BLOCK: char = '▨';
    // pub const E: char = '▢';
    // pub const F: char = '▨';

    pub struct Tetris {
        stdout: RawTerminal<Stdout>,
        buffer: [[char; 6]; 12],
        current_piece: pieces::Piece,
        inputs: Arc<Mutex<Vec<Key>>>,
        changed_buffer: bool,
    }

    impl Tetris {
        pub fn new() -> Self {
            Tetris {
                stdout: stdout().into_raw_mode().unwrap(),
                inputs: Arc::default(),
                buffer: [[pieces::EMPTY_CELL; 6]; 12],
                changed_buffer: true,
                current_piece: pieces::STICK_PIECE,
            }
        }

        fn read_inputs(ref_inputs: &mut Arc<Mutex<Vec<Key>>>) {
            let stdin = stdin();
            for c in stdin.events() {
                match c.unwrap() {
                    Event::Key(c) => {
                        let mut l = ref_inputs.lock().unwrap();
                        l.push(c);
                        drop(l);
                        if let Key::Esc = c {
                            return;
                        }
                    }
                    _ => {}
                }
            }
        }

        fn interchange(&mut self, since_this_row: usize) {
            for col in 0..self.buffer[0].len() {
                let mut write_index = since_this_row;
                for row in (0..since_this_row).rev() {
                    self.buffer[write_index][col] = self.buffer[row][col];
                    write_index -= 1;
                }
            }
        }

        /// Returns how many rows have been deleted
        fn delete_rows(&mut self) -> usize {
            let mut count = 0;
            for row in 0..self.buffer.len() {
                let mut delete = true;
                for col in 0..self.buffer[0].len() {
                    if self.buffer[row][col] == pieces::EMPTY_CELL {
                        delete = false;
                        break;
                    }
                }
                if delete {
                    count += 1;
                    self.interchange(row);
                }
            }
            count
        }

        /// Returns if the move has been succesful
        fn move_piece(&mut self, piece_move: Move) -> bool {
            // Clear the piece
            self.render_piece(pieces::EMPTY_CELL);
            let ((mut x, mut y), mut rot, _) = &mut self.current_piece;
            match piece_move {
                Move::LEFT => x -= 1,
                Move::RIGHT => x += 1,
                Move::DOWN => y += 1,
                Move::RIGHT_ROTATION => rot += 1,
                Move::LEFT_ROTATION => rot -= 1,
            }
            // y = y.min((self.buffer.len() - height) as isize).max(0);
            rot = clamp_over(rot, 3, 0);
            (self.current_piece.0).0 = x;
            (self.current_piece.0).1 = y;
            self.current_piece.1 = rot;
            let do_reverse = self.reverse_move();
            if do_reverse {
                match piece_move {
                    Move::LEFT => x += 1,
                    Move::RIGHT => x -= 1,
                    Move::DOWN => y -= 1,
                    Move::RIGHT_ROTATION => rot -= 1,
                    Move::LEFT_ROTATION => rot += 1,
                }
                (self.current_piece.0).0 = x;
                (self.current_piece.0).1 = y;
                rot = clamp_over(rot, 3, 0);
                self.current_piece.1 = rot;
            }
            self.render_piece(pieces::FILLED_CELL);
            !do_reverse
        }

        fn reverse_move(&self) -> bool {
            let ((x, y), rot, piece_buffers) = &self.current_piece;
            let piece_buffer = &piece_buffers[*rot as usize];
            for row in 0..piece_buffer.len() {
                for col in 0..piece_buffer[0].len() {
                    if piece_buffer[row][col] == pieces::EMPTY_CELL {
                        continue;
                    }
                    let (y, x) = ((*y + row as isize), (*x + col as isize));
                    if out_of_bounds(x, &self.buffer[0])
                        || out_of_bounds(y, &self.buffer)
                        || self.buffer[y as usize][x as usize] == pieces::FILLED_CELL
                    {
                        return true;
                    }
                }
            }
            false
        }

        fn go_to_low(&mut self) {
            while self.move_piece(Move::DOWN) {}
        }

        fn render_piece(&mut self, cell_type: char) {
            let ((x, y), rot, piece_buffers) = &self.current_piece;
            let piece_buffer = &piece_buffers[*rot as usize];
            for row in 0..piece_buffer.len() {
                for col in 0..piece_buffer[0].len() {
                    if piece_buffer[row][col] == pieces::EMPTY_CELL {
                        continue;
                    }
                    self.buffer[(*y + row as isize) as usize][(*x + col as isize) as usize] =
                        cell_type;
                }
            }
            self.changed_buffer = true;
        }

        fn render_buffer(&mut self) -> Result<(), ()> {
            write!(self.stdout, "{}", termion::clear::All);
            for r in 0..self.buffer.len() {
                write!(self.stdout, "{}", termion::cursor::Goto(1, r as u16 + 1),);
                for c in 0..self.buffer[0].len() {
                    let character = self.buffer[r][c];
                    write!(self.stdout, "{}", character);
                }
            }
            write!(self.stdout, "\n{:?}", self.current_piece);
            self.changed_buffer = false;
            self.stdout.flush().unwrap();
            Ok(())
        }

        fn update(&mut self) -> Result<(), ()> {
            if self.changed_buffer {
                self.render_buffer().unwrap();
            }
            std::thread::sleep(std::time::Duration::from_millis(1));
            let mut l = self.inputs.lock().unwrap();
            let s: Vec<Key> = l.drain(..).collect();
            drop(l);
            for c in s {
                write!(
                    self.stdout,
                    "{}{}",
                    termion::cursor::Goto(1, 1),
                    termion::clear::CurrentLine
                )
                .unwrap();

                match c {
                    Key::Esc => {
                        write!(self.stdout, "{}", termion::cursor::Show).unwrap();
                        return Err(());
                    }
                    Key::Char(' ') => {
                        self.go_to_low();
                        self.delete_rows();
                        self.current_piece = pieces::get_piece();
                        self.render_piece(pieces::FILLED_CELL);
                    }
                    Key::Char('q') | Key::Char('Q') => {
                        self.move_piece(Move::LEFT_ROTATION);
                    }
                    Key::Char('e') | Key::Char('E') => {
                        self.move_piece(Move::RIGHT_ROTATION);
                    }
                    Key::Left => {
                        self.move_piece(Move::LEFT);
                    }
                    Key::Right => {
                        self.move_piece(Move::RIGHT);
                    }
                    Key::Down => {
                        self.move_piece(Move::DOWN);
                    }
                    _ => {}
                }
            }
            Ok(())
        }

        pub fn start(mut self) {
            write!(
                self.stdout,
                "{}{}",
                termion::cursor::Goto(1, 1),
                termion::clear::CurrentLine
            );
            let mut r = self.inputs.clone();
            let c = std::thread::spawn(move || {
                Tetris::read_inputs(&mut r);
            });
            self.render_piece(pieces::FILLED_CELL);
            loop {
                if let Err(()) = self.update() {
                    break;
                }
            }
            c.join().unwrap();
            write!(
                self.stdout,
                "{}{}",
                termion::cursor::Show,
                termion::clear::All,
            )
            .unwrap();
            self.stdout.flush().unwrap();
        }
    }
}
