use amethyst::prelude::*;
use amethyst::assets::{Completion, ProgressCounter};
use log::{error, info, trace};

use crate::game::Game;

#[derive(Debug)]
pub struct Load;

impl Load {
    pub fn new() -> Self {
        Load
    }
}

impl SimpleState for Load {
    fn update(&mut self, data: &mut StateData<GameData>) -> SimpleTrans {
        let StateData { world, .. } = data;
        let progress = world.fetch::<ProgressCounter>();
        match progress.complete() {
            Completion::Complete => {
                info!("Loading of assets complete");
                Trans::Switch(Box::new(Game::new()))
            }
            Completion::Failed => {
                error!("Failed to load some assets: ");
                for e in progress.errors() {
                    error!("{:?}", e);
                }
                Trans::Quit
            }
            Completion::Loading => {
                trace!("Haven't loaded everything yet");
                Trans::None
            }
        }
    }
}
