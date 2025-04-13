use pancurses::{newwin, Window, COLOR_PAIR};

pub struct Score {
    score_board:Window,
}

impl Score {
    pub fn new(board_game: &Window) -> Score {
        let window = newwin(
            3,
            5,
            board_game.get_beg_y(),
            board_game.get_beg_x() + board_game.get_max_x() + 1, // rightmost point + wight of window + 1
        );

        window.bkgd(COLOR_PAIR(1));
        window.refresh();
        window.nodelay(true);

        Score {
            score_board: window,
        }
    }

    pub fn update_score(&mut self, len:i32, capacity: i32) {
        let score = format!("{}/{}", len.to_string(), capacity.to_string());
        self.score_board.clear();
        self.score_board.mvprintw(self.score_board.get_max_y() / 2, (self.score_board.get_max_x() / 2) - (score.len() as i32 / 2), score);
        self.score_board.refresh();
    }
}