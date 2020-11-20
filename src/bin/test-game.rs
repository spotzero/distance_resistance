extern crate distance_resistance;
use distance_resistance::resistance::*;
use std::fmt;

fn main() {
    let mut games = ResistanceGames::new();
    let gid = games.create(5);

    println!("{:#?}", games);

    let player1 = games.get_mut(gid.clone()).unwrap().join().unwrap();
    let player2 = games.get_mut(gid.clone()).unwrap().join().unwrap();
    let player3 = games.get_mut(gid.clone()).unwrap().join().unwrap();
    let player4 = games.get_mut(gid.clone()).unwrap().join().unwrap();
    let player5 = games.get_mut(gid.clone()).unwrap().join().unwrap();

    games.get_mut(gid.clone()).unwrap().change_name(player1.clone(), "Alice".to_string());
    games.get_mut(gid.clone()).unwrap().change_name(player2.clone(), "Bob".to_string());
    games.get_mut(gid.clone()).unwrap().change_name(player3.clone(), "Charlie".to_string());
    games.get_mut(gid.clone()).unwrap().change_name(player4.clone(), "Eve".to_string());
    games.get_mut(gid.clone()).unwrap().change_name(player5.clone(), "Malory".to_string());

    let _ = games.get_mut(gid.clone()).unwrap().choose_operatives(player1.clone(), vec![0, 1]);

    println!("{:#?}", games);

}
