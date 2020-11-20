extern crate distance_resistance;
use distance_resistance::resistance::*;

fn main() {
    let mut games = ResistanceGames::new();
    let gid = games.create(5);

    println!("Game id {:?} {:?}", gid, games);

    let player1 = games.join(gid);
    let player2 = games.join(gid);
    let player3 = games.join(gid);
    let player4 = games.join(gid);
    let player5 = games.join(gid);

    games.get(gid)?.change_name(player1, "Alice".to_string());
    games.get(gid)?.change_name(player2, "Bob".to_string());
    games.get(gid)?.change_name(player3, "Charlie".to_string());
    games.get(gid)?.change_name(player4, "Eve".to_string());
    games.get(gid)?.change_name(player5, "Malory".to_string());

    println!("Game state: {:?}", games.get(gid).state());

}
