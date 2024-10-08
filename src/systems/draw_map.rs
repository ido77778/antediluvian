use crate::camera::Camera;
use crate::json::JsonData;
use crate::prelude::*;
use crate::map::Map;
use crate::components::{Player, Viewshed};

#[system]
#[read_component(Viewshed)]
#[read_component(Player)]
pub fn draw_map
(
    ecs: &SubWorld,
    #[resource] map: &Map,
    #[resource] camera: &Camera,
    #[resource] json: &JsonData
)
{
    let mut viewshed = <&Viewshed>::query().filter(component::<Player>());
    let player_viewshed = viewshed.iter(ecs).nth(0).unwrap();
    let mut draw_batch = DrawBatch::new();

    // Renders the viewable tiles from top left to bottom right.
    for y in camera.top_y .. camera.bottom_y
    {
        for x in camera.left_x .. camera.right_x
        {
            let pt = Point::new(x, y); // The current tile.
            let offset = Point::new(camera.left_x, camera.top_y);

            if map.in_bounds(pt) && map.revealed_tiles[pt.to_index(map.width)]
            {
                let tile = map.get_tile(pt);
                    
                let color = json.tiles[&tile].rgb;
                let glyph = json.tiles[&tile].glyph;
                if player_viewshed.visible_tiles.contains(&pt)
                {
                    // If it's in the player's FOV, we want to draw the tile in full color.
                    draw_batch.set
                    (
                        pt - offset,
                        ColorPair::new(RGB::from_u8(color.0, color.1, color.2), rltk::BLACK),
                        to_cp437(glyph)
                    );
                }
                else
                {
                    // Otherwise, we'll draw it in grayscale.
                    draw_batch.set
                    (
                        pt - offset,
                        ColorPair::new(RGB::from_u8(color.0, color.1, color.2).to_greyscale(), rltk::BLACK),
                        to_cp437(glyph)
                    );
                }
            }
        }
    }
    draw_batch.submit(0).expect("Batch error")
}