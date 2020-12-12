use std::collections::HashMap;
use rand::{Rng, rngs::ThreadRng};
use rand::distributions::Alphanumeric;
use rand::thread_rng;
use rand::seq::SliceRandom;
use crate::names::get_name;

#[derive(Clone, Debug, Default)]
pub struct RoundStatus {
    pub state: MissionState,
    pub operatives: Vec<usize>,
    pub approvals: Vec<Approval>,
    pub mission: Vec<bool>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Approval {
    None,
    Reject,
    Approve,
}

#[derive(Clone, Debug)]
pub struct Player {
    pub player_type: PlayerType,
    pub key: String,
    pub name: String,
    pub id: usize,
}

impl Player {
    fn new(rng: &ThreadRng) -> Self {
        Player {
            player_type: PlayerType::default(),
            key: rng.sample_iter(&Alphanumeric).take(18).collect(),
            name: get_name(),
            id: 0,
        }
    }

    fn set_type(&mut self, player_type: PlayerType) {
        self.player_type = player_type;
    }

    fn change_name(&mut self, new_name: String) {
        self.name = new_name;
    }
}

#[derive(Clone, Debug)]
pub struct Spot {
    pub key: String,
    pub claimed: bool,
}

#[derive(Clone, Debug)]
pub struct ResistanceError;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PlayerType {
    Agent,
    Spy,
}

impl Default for PlayerType {
    fn default() -> Self { PlayerType::Agent }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum MissionState {
    Pending,
    SelectingOperatives,
    ApprovingMission,
    RunningMission,
    Failure,
    Victory,
}

impl Default for MissionState {
    fn default() -> Self { MissionState::Pending }
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
    pub numberofplayers: usize,
}

impl ResistanceGame {
    pub fn new(numberofplayers: usize, mut rng: &ThreadRng) -> Result<Self,&'static str> {
        if numberofplayers >= 5 && numberofplayers <=10 {
            let (players, spots) = ResistanceGame::generate_players(numberofplayers, &mut rng);
            Ok(ResistanceGame {
                players: players,
                leader: 0,
                round: 0,
                wins: 0,
                status: vec![RoundStatus::default(); 5],
                vote: 0,
                spots: spots,
                started: false,
                numberofplayers: numberofplayers,
            })
        } else {
            Err("Only 5 - 10 players are allowed")
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
                let player_key = self.spots[n].key.clone();
                self.get_mut_player(&player_key)?.id = n;
                return Ok(self.spots[n].key.clone());
            }
        }
        Err("No spots available")
    }

    pub fn start(&mut self) -> Result<(), &'static str> {
        if self.started {
            Err("Game is already started")
        } else {
            for n in 0..self.spots.len() {
                if !self.spots[n].claimed {
                    return Err("Not everyone has joined yet");
                }
            }
            self.started = true;
            self.status[0].state = MissionState::SelectingOperatives;
            Ok(())
        }
    }

    pub fn change_name(&mut self, player_key: &String, name: String) -> Result<(), &'static str> {
        if self.started {
            Err("You cannot change your name once the game has started.")
        } else {
            self.get_mut_player(&player_key)?.change_name(name);
            Ok(())
        }
    }

    pub fn choose_operatives(&mut self, player_key: &String, selected_players: Vec<usize>) -> Result<(), &'static str> {
        if self.status[self.round].state != MissionState::SelectingOperatives {
            Err("It's not time to select operatives")
        } else if self.leader != self.get_player(&player_key)?.id {
          Err("You're not the leader!")
        } else if mission_size(self.spots.len(), self.round) != selected_players.len() {
            Err("Wrong number of operatives selected")
        } else {
            self.status[self.round].operatives = selected_players;
            self.status[self.round].state = MissionState::ApprovingMission;
            Ok(())
        }
    }

    pub fn vote_to_approve(&mut self, player_key: &String, vote: bool) -> Result<(), &'static str> {
        let playerid = self.playerid(player_key)?;
        if self.status[self.round].state != MissionState::ApprovingMission {
            Err("It's not time to vote!")
        } else if self.has_voted(playerid) {
            Err("Player has already voted!")
        } else {
            if self.status[self.round].approvals.len() == 0 {
                self.status[self.round].approvals = vec![Approval::None; self.numberofplayers];
            }
            if vote {
                self.status[self.round].approvals[playerid] = Approval::Approve;
            } else {
                self.status[self.round].approvals[playerid] = Approval::Reject;
            }
            if self.has_everyone_voted() {
                self.approve_or_reject();
            }

            Ok(())
        }
    }

    pub fn succeed_mission(&mut self, player_key: &String, status: bool) -> Result<(), &'static str> {
        Err("It's not even time for the mission")
    }

    fn current_status(&self) -> MissionState {
        self.status[self.round].state
    }

    fn playerid(&self, player_key: &String) -> Result<usize, &'static str> {
        Ok(self.get_player(player_key)?.id)
    }

    fn has_voted(&self, playerid: usize) -> bool {
        !(self.status[self.round].approvals.len() == 0 || self.status[self.round].approvals[playerid] == Approval::None)
    }

    fn has_everyone_voted(&self) -> bool {
        self.status[self.round].approvals.iter().filter(|a|**a == Approval::None).collect::<Vec<&Approval>>().len() == 0
    }

    fn approve_or_reject(&mut self) {
        let approve = self.status[self.round].approvals.iter().filter(|a|**a == Approval::Approve).collect::<Vec<&Approval>>().len();
        let reject = self.status[self.round].approvals.iter().filter(|a|**a == Approval::Reject).collect::<Vec<&Approval>>().len();
        if approve >= reject {
            self.status[self.round].state = MissionState::RunningMission;
        } else {
            self.vote += 1;
            // Handle if vote fails
            self.status[self.round].state = MissionState::RunningMission;
        }
    }

    fn get_player(&self, player_key: &String) -> Result<&Player, &'static str> {
        let player = self.players.get(player_key);
        match player {
            Some(p) => Ok(p),
            None => Err("Invalid player"),
        }
    }

    fn get_mut_player(&mut self, player_key: &String) -> Result<&mut Player, &'static str> {
        let player = self.players.get_mut(player_key);
        match player {
            Some(p) => Ok(p),
            None => Err("Invalid player"),
        }
    }
}

#[derive(Debug)]
pub struct ResistanceGames {
    pub rng: ThreadRng,
    pub games: HashMap<String, ResistanceGame>,
}

impl ResistanceGames {
    pub fn new() -> Self {
        ResistanceGames {
            rng: rand::thread_rng(),
            games: HashMap::new(),
        }
    }

    pub fn create(&mut self, numberofplayers: usize) -> Result<String, &'static str> {
        let id : String = self.rng.sample_iter(&Alphanumeric).take(6).collect();
        self.games.insert(id.clone(), ResistanceGame::new(numberofplayers, &mut self.rng)?);
        Ok(id.clone())
    }

    pub fn get(&self, key: &String) -> Option<&ResistanceGame> {
        self.games.get(key)
    }

    pub fn get_mut(&mut self, key: &String) -> Option<&mut ResistanceGame> {
        self.games.get_mut(key)
    }
}

fn mission_size( numberofplayers: usize, missionno: usize) -> usize {
    let sizes = [
        [2,2,2,3,3,3],
        [3,3,3,4,4,4],
        [2,4,3,4,4,4],
        [3,3,4,5,5,5],
        [3,4,4,5,5,5],
    ];
    return sizes[missionno][numberofplayers - 5];
}