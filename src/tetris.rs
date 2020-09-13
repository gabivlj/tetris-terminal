pub mod game {
    use crate::pieces;
    use crate::pieces::Move;
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
                current_piece: pieces::BLOCK_PIECE,
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
                        if let Key::Char('q') = c {
                            return;
                        }
                    }
                    _ => {}
                }
            }
        }

        // !! at the moment it only moves to the right!!!
        fn move_piece(&mut self, piece_move: Move) {
            // Clear the piece
            self.render_piece(pieces::EMPTY_CELL);
            let ((mut x, mut y), _, _, w, h) = &mut self.current_piece;
            match piece_move {
                Move::LEFT => x -= 1,
                Move::RIGHT => x += 1,
                Move::DOWN => y += 1,
            }
            x = x.min((self.buffer[0].len() - *w) as isize).max(0);
            y = y.min((self.buffer.len() - *h) as isize).max(0);
            (self.current_piece.0).0 = x;
            (self.current_piece.0).1 = y;
            self.render_piece(pieces::FILLED_CELL);
        }

        fn reverse_move() -> bool {
            false
        }

        fn render_piece(&mut self, cell_type: char) {
            let ((x, y), rot, piece_buffers, _, _) = &mut self.current_piece;
            let piece_buffer = &mut piece_buffers[*rot];
            for row in 0..piece_buffer.len() {
                for col in 0..piece_buffer[0].len() {
                    if piece_buffer[row][col] == pieces::EMPTY_CELL {
                        continue;
                    }
                    self.buffer[*y as usize + row][*x as usize + col] = cell_type;
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
            std::thread::sleep(std::time::Duration::from_millis(100));
            let mut l = self.inputs.lock().unwrap();
            let s: Vec<Key> = l.drain(..).collect();
            drop(l);
            for c in &s {
                write!(
                    self.stdout,
                    "{}{}",
                    termion::cursor::Goto(1, 1),
                    termion::clear::CurrentLine
                )
                .unwrap();

                match c {
                    Key::Char('q') => {
                        write!(self.stdout, "{}", termion::cursor::Show).unwrap();
                        return Err(());
                    }
                    Key::Left => self.move_piece(Move::LEFT),
                    Key::Right => {
                        self.move_piece(Move::RIGHT);
                    }
                    Key::Down => self.move_piece(Move::DOWN),
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
