use crate::visibility_system::VisibilitySystem;
use crate::components::*;
use crate::player::player_input;
use crate::renderer::draw_level;

use rltk::{GameState, Rltk};
use specs::{Join, RunNow, World, WorldExt};

pub struct State
{
    pub ecs: World,
}

impl GameState for State
{
    fn tick(&mut self, ctx: &mut Rltk)
    {
        ctx.cls();

        player_input(self, ctx);
        draw_level(&self.ecs, ctx);
        self.run_systems();

        // Reads from storage.
        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();
        
        // Renders all objects with both Position and Renderable.
        for (pos, render) in (&positions, &renderables).join()
        {
            ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph)
        }
    }
}

impl State
{
    fn run_systems(&mut self)
    {
        let mut vis = VisibilitySystem{};
        vis.run_now(&self.ecs);
        self.ecs.maintain()
    }
}
