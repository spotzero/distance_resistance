use std::collections::HashMap;
use rand::{Rng, rngs::ThreadRng};
use rand::prelude::SliceRandom;
use rand::distributions::Alphanumeric;
use crate::names::get_name;

#[derive(Debug)]
pub struct ResistanceGames {
    pub rng: ThreadRng,
    pub games: HashMap<String, ResistanceGame>,
}

#[derive(Debug)]
pub struct ResistanceGame {
    pub leader: usize,
    pub players: Vec<Player>,
    pub round: usize,
    pub wins: usize,
    pub status: Vec<RoundStatus>,
    pub vote: usize,
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
pub enum PlayerType {
    Agent,
    Spy,
}

#[derive(Debug)]
#[derive(Clone)]
#[derive(Copy)]
pub enum MissionState {
    Pending,
    Approving,
    Engaging,
    Failure,
    Victory,
}

impl Player {
    fn new(rng: &ThreadRng, pt: PlayerType) -> Self {
        Player {
            player_type: pt,
            key: rng.sample_iter(&Alphanumeric).take(18).collect(),
            name: get_name(),
        }
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
        ResistanceGame {
            players: ResistanceGame::generate_players(numberofplayers, &mut rng),
            leader: 0,
            round: 0,
            wins: 0,
            status: vec![RoundStatus::default(); 5],
            vote: 0,
        }
    }

    fn generate_players(numberofplayers: usize, mut rng: &ThreadRng) -> Vec<Player> {
        let mut players = Vec::with_capacity(numberofplayers);
        let mut spies = numberofplayers / 2;
        if numberofplayers - spies == 0 {
            spies -= 1;
        }
        for n in 1..5 {
            if n <= spies {
                players.push(Player::new(&mut rng, PlayerType::Agent));
            } else {
                players.push(Player::new(&mut rng, PlayerType::Spy));
            }
        }
        players
    }

    pub fn change_name(&mut self, player_key: String, name: String) {
        
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

    fn currentStatus(&self) -> MissionState {
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
        id
    }

    pub fn get(&self, key: String) -> Option<&ResistanceGame> {
        self.games.get(&key)
    }
}