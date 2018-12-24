use std::{
    collections::VecDeque,
};

fn main() {
    solve(431, 70950);
    solve(431, 7095000);
}

fn solve(players: usize, marbles: usize) {
    let mut board = VecDeque::new();
    let mut scores = vec![0; players];
    board.push_back(0);

    // The current marble is the back end.

    let mut player = 0;
    for marble in 1 ..= marbles {
        if marble % 23 == 0 {
            // First, the current player keeps the marble they would have
            // placed, adding it to their score.
            scores[player] += marble;

            // In addition, the marble 7 marbles counter-clockwise from the
            // current marble is removed from the circle, and also added to
            // the current player's score.  The marble located immediately
            // clockwise of the marble that was removed becomes the new
            // current marble.
            for _ in 0 .. 7 {
                let tmp = board.pop_back().unwrap();
                board.push_front(tmp);
            }
            let tmp = board.pop_back().unwrap();
            scores[player] += tmp;

            let tmp = board.pop_front().unwrap();
            board.push_back(tmp);
        } else {
            let tmp = board.pop_front().unwrap();
            board.push_back(tmp);
            board.push_back(marble);
        }

        player += 1;
        if player >= players {
            player = 0;
        }

        // println!("{}: {:?}", player, board);
    }
    println!("Max score: {:?}", scores.iter().max().unwrap());
}
