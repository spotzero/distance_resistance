extern crate distance_resistance;
use distance_resistance::resistance::*;
use std::fmt;

fn main() {
    let mut games = ResistanceGames::new();
    let gid = games.create(5).unwrap();

    println!("{:#?}", games);

    let player0 = games.get_mut(&gid).unwrap().join().unwrap();
    let player1 = games.get_mut(&gid).unwrap().join().unwrap();
    let player2 = games.get_mut(&gid).unwrap().join().unwrap();
    let player3 = games.get_mut(&gid).unwrap().join().unwrap();
    let player4 = games.get_mut(&gid).unwrap().join().unwrap();

    games.get_mut(&gid).unwrap().change_name(&player0, "Malory".to_string()).unwrap();
    games.get_mut(&gid).unwrap().change_name(&player1, "Alice".to_string()).unwrap();
    games.get_mut(&gid).unwrap().change_name(&player2, "Bob".to_string()).unwrap();
    games.get_mut(&gid).unwrap().change_name(&player3, "Charlie".to_string()).unwrap();
    games.get_mut(&gid).unwrap().change_name(&player4, "Eve".to_string()).unwrap();


    games.get_mut(&gid).unwrap().start().unwrap();

    println!("{:#?}", games);

    let _ = games.get_mut(&gid).unwrap().choose_operatives(&player0, vec![0, 1]);

    games.get_mut(&gid).unwrap().vote_to_approve(&player0, false).unwrap();
    games.get_mut(&gid).unwrap().vote_to_approve(&player1, true).unwrap();
    games.get_mut(&gid).unwrap().vote_to_approve(&player2, true).unwrap();
    games.get_mut(&gid).unwrap().vote_to_approve(&player3, false).unwrap();
    games.get_mut(&gid).unwrap().vote_to_approve(&player4, false).unwrap();

    println!("{:#?}", games);

    let _ = games.get_mut(&gid).unwrap().choose_operatives(&player1, vec![2, 4]);

    games.get_mut(&gid).unwrap().vote_to_approve(&player0, false).unwrap();
    games.get_mut(&gid).unwrap().vote_to_approve(&player1, true).unwrap();
    games.get_mut(&gid).unwrap().vote_to_approve(&player2, true).unwrap();
    games.get_mut(&gid).unwrap().vote_to_approve(&player3, false).unwrap();
    games.get_mut(&gid).unwrap().vote_to_approve(&player4, true).unwrap();

    println!("{:#?}", games);

    games.get_mut(&gid).unwrap().succeed_mission(&player2, true).unwrap();
    games.get_mut(&gid).unwrap().succeed_mission(&player4, true).unwrap();

    println!("{:#?}", games);
}
