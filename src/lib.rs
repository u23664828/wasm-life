use wasm_bindgen::prelude::*;

/// 0 = empty, 1 = X, 2 = O
#[wasm_bindgen]
pub struct Game {
    board: [u8; 9],
    player: u8,  // 1 or 2
    winner: u8,  // 0 none, 1 X, 2 O, 3 draw
    moves: u8,
}

#[wasm_bindgen]
impl Game {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Game {
        Game { board: [0; 9], player: 1, winner: 0, moves: 0 }
    }

    pub fn reset(&mut self) {
        self.board = [0; 9];
        self.player = 1;
        self.winner = 0;
        self.moves = 0;
    }

    pub fn get_board(&self) -> Vec<u8> { self.board.to_vec() }
    pub fn current_player(&self) -> u8 { self.player }
    pub fn winner(&self) -> u8 { self.winner }

    /// idx = 0..8; returns true if move accepted
    pub fn play(&mut self, idx: u8) -> bool {
        if self.winner != 0 { return false; }
        let i = idx as usize;
        if i >= 9 || self.board[i] != 0 { return false; }
        self.board[i] = self.player;
        self.moves += 1;
        if self.check_win(self.player) {
            self.winner = self.player;
        } else if self.moves == 9 {
            self.winner = 3; // draw
        } else {
            self.player = if self.player == 1 { 2 } else { 1 };
        }
        true
    }
}

impl Game {
    fn check_win(&self, p: u8) -> bool {
        const LINES: [[usize; 3]; 8] = [
            [0,1,2],[3,4,5],[6,7,8],
            [0,3,6],[1,4,7],[2,5,8],
            [0,4,8],[2,4,6],
        ];
        LINES.iter().any(|&[a,b,c]| self.board[a]==p && self.board[b]==p && self.board[c]==p)
    }
}
