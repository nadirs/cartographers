mod types;

use types::Adventurer;


#[derive(Debug, PartialEq, Eq)]
pub struct Game {
    player: Option<Player>,
}

#[derive(Debug, PartialEq, Eq)]
struct Player {
    guild: Guild,
    gold: usize,
}

#[derive(Debug, PartialEq, Eq)]
struct Guild {
    name: String,
    members: Vec<Adventurer>,
}

impl Game {
    pub fn new() -> Self {
        Game {
            player: None,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        Game::new();
    }
}
