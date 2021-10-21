mod components;
mod state;
mod json;
mod player;
mod map;
mod renderer;

use player::create_player;
use components::*;
use json::JsonData;
use map::Map;
use state::State;

use specs::{ World, WorldExt };

#[macro_use]
extern crate log;

fn main() -> rltk::BError
{
    log4rs::init_file("log4rs.yml", Default::default()).unwrap();

    let context = rltk::RltkBuilder::simple80x50()
        .with_title("Agricultor")
        .build()?;
    
    let mut gs = State { ecs: World::new(), json: JsonData::new()}; // Gamestate

    // Register the components.
    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Player>();

    gs.ecs.insert(Map::new());

    create_player(&mut gs, 40, 25);

    rltk::main_loop(context, gs)
}