use std::collections::HashMap;
use rand::{Rng, rngs::ThreadRng};
use rand::distributions::Alphanumeric;
use rand::thread_rng;
use rand::seq::SliceRandom;
use crate::names::get_name;

#[derive(Debug)]
pub struct ResistanceGames {
    pub rng: ThreadRng,
    pub games: HashMap<String, ResistanceGame>,
}

#[derive(Debug)]
pub struct ResistanceGame {
    pub leader: usize,
    pub players: HashMap<String, Player>,
    pub spots: Vec<Spot>,
    pub round: usize,
    pub wins: usize,
    pub status: Vec<RoundStatus>,
    pub vote: usize,
    pub started: bool,
}

#[derive(Debug)]
#[derive(Default)]
#[derive(Clone)]
pub struct RoundStatus {
    pub state: MissionState,
    pub players: Vec<usize>,
}

#[derive(Debug)]
#[derive(Clone)]
pub struct Player {
    pub player_type: PlayerType,
    pub key: String,
    pub name: String,
}

#[derive(Debug)]
#[derive(Clone)]
pub struct Spot {
    pub key: String,
    pub claimed: bool,
}

#[derive(Debug)]
#[derive(Clone)]
pub enum PlayerType {
    Agent,
    Spy,
}

#[derive(Debug)]
#[derive(Clone)]
#[derive(Copy)]
pub enum MissionState {
    Joining,
    Pending,
    Approving,
    Engaging,
    Failure,
    Victory,
}

impl Player {
    fn new(rng: &ThreadRng) -> Self {
        Player {
            player_type: PlayerType::default(),
            key: rng.sample_iter(&Alphanumeric).take(18).collect(),
            name: get_name(),
        }
    }

    fn set_type(&mut self, player_type: PlayerType) {
        self.player_type = player_type;
    }

    fn change_name(&mut self, new_name: String) {
        self.name = new_name;
    }
}

impl Default for PlayerType {
    fn default() -> Self { PlayerType::Agent }
}

impl Default for MissionState {
    fn default() -> Self { MissionState::Pending }
}

impl ResistanceGame {
    pub fn new(numberofplayers: usize, mut rng: &ThreadRng) -> Self {
        let (players, spots) = ResistanceGame::generate_players(numberofplayers, &mut rng);
        ResistanceGame {
            players: players,
            leader: 0,
            round: 0,
            wins: 0,
            status: vec![RoundStatus::default(); 5],
            vote: 0,
            spots: spots,
            started: false,
        }

    }

    fn generate_players(numberofplayers: usize, mut rng: &ThreadRng) -> (HashMap<String, Player>, Vec<Spot>) {
        let mut players = HashMap::new();
        let mut spots = Vec::new();
        let mut spies = numberofplayers / 2;
        if numberofplayers - spies == 0 {
            spies -= 1;
        }
        for n in 0..numberofplayers {
            let mut player = Player::new(&mut rng);
            if n <= spies {
                player.set_type(PlayerType::Agent);
            } else {
                player.set_type(PlayerType::Spy);
            }
            spots.push(Spot{key: player.key.clone(), claimed: false});
            players.insert(player.key.clone(), player);
        }
        let mut rng = thread_rng();
        let _:isize = rng.gen();
        spots.shuffle(&mut rng);
        (players, spots)
    }

    pub fn join(&mut self) -> Result<String, &'static str> {
        for n in 0..self.spots.len() {
            if !self.spots[n].claimed {
                self.spots[n].claimed = true;
                if n == self.spots.len() - 1 {
                    self.started = true;
                }
                return Ok(self.spots[n].key.clone());
            }
        }
        Err("No spots available")
    }

    pub fn change_name(&mut self, player_key: String, name: String) {
        self.players.get_mut(&player_key).unwrap().change_name(name);
    }

    pub fn choose_operatives(&mut self, player_key: String, players: Vec<usize>) -> Result<(), &'static str> {
        Err("You're not the leader!")
    }

    pub fn vote_to_approve(&mut self, player_key: String, vote: bool) -> Result<(), &'static str> {
        Err("It's not time to vote!")
    }

    pub fn succeed_mission(&mut self, player_key: String, status: bool) -> Result<(), &'static str> {
        Err("It's not even time for the mission")
    }

    fn current_status(&self) -> MissionState {
        self.status[self.round].state
    }
}

impl ResistanceGames {
    pub fn new() -> Self {
        ResistanceGames {
            rng: rand::thread_rng(),
            games: HashMap::new(),
        }
    }

    pub fn create(&mut self, numberofplayers: usize)  -> String {
        let id : String = self.rng.sample_iter(&Alphanumeric).take(6).collect();
        self.games.insert(id.clone(), ResistanceGame::new(numberofplayers, &mut self.rng));
        id.clone()
    }

    pub fn get(&self, key: String) -> Option<&ResistanceGame> {
        self.games.get(&key)
    }

    pub fn get_mut(&mut self, key: String) -> Option<&mut ResistanceGame> {
        self.games.get_mut(&key)
    }
}
