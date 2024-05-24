use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use std::vec;

use rand::{seq::SliceRandom, Rng};

const ROWS: usize = 30;
const COLUMNS: usize = 30;
const TILE_TEXTURES: &str = "./tile_texture.png";

#[derive(Clone, Debug, PartialEq)]
enum State {
    GRASS,
    WATER,
    MOUNTAIN,
    FOREST,
    NONE
}

#[derive(Clone, Debug)]
struct Cell {
    state: State,
    x_pos: usize,
    y_pos: usize,
    collapsed: bool,
    options: Vec<State>,
}

struct Rule {
    tile: State,
    top: Vec<State>,
    right: Vec<State>,
    bottom: Vec<State>,
    left: Vec<State>,
    top_left: Vec<State>,
    top_right: Vec<State>,
    bottom_left: Vec<State>,
    bottom_right: Vec<State>,
}

#[derive(PartialEq)]
struct WeightedState{
    state: State,
    weight: f32,
}

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
    .add_plugins(WorldInspectorPlugin::new())
    .add_systems(Startup, setup);

    app.run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture_handle = asset_server.load(TILE_TEXTURES);
    let atlas_layout = TextureAtlasLayout::from_grid(Vec2::new(16.0, 16.0), 5, 1, None, None);
    let texture_atlas_layout_handle = texture_atlas_layouts.add(atlas_layout);

    let tile_rules = generate_tile_rules();
    let mut grid = generate_grid();
    grid = wave_function_collapse(grid, &tile_rules);

    commands.spawn(Camera2dBundle::default());

    for row in 0..ROWS {
        for column in 0..COLUMNS {
            let index = (row * ROWS) + column;
            let state = &grid[index].state;
            let index = match state {
                State::FOREST => 2,
                State::GRASS => 0,
                State::WATER => 1,
                State::MOUNTAIN => 3,
                State::NONE => 4,
            };
            commands.spawn(
                SpriteSheetBundle {
                    texture: texture_handle.clone(),
                    atlas: TextureAtlas {
                        layout: texture_atlas_layout_handle.clone(),
                        index: index,
                    },
                    transform: Transform {
                        translation: Vec3::new(
                        (column as f32 * 16.0) - 200.0,
                        (row as f32 * 16.0) - 200.0, 
                        1.0,
                    ),
                    ..Default::default()
                    },
                    ..Default::default()
                }
            );
        }
    }
}

fn wave_function_collapse(mut grid: Vec<Cell>, tile_rules: &Vec<Rule>) -> Vec<Cell> {
    let mut rng = rand::thread_rng();
    let mut row = rng.gen_range(1..ROWS);
    let mut column = rng.gen_range(1..COLUMNS);
    let mut current_tile = grid[(row * ROWS) + column].clone();

    current_tile.collapsed = true;
    current_tile.state = State::GRASS;

    for _iteration in 0..700 {   //320
        //check top cell
        let top_tile = &mut grid[((row - 1) * ROWS) + column].clone();
        for current_tile_rules in tile_rules {
            if current_tile_rules.tile == current_tile.state {
                let top_tile_rules = &current_tile_rules.top;
                top_tile.options.retain(|state| top_tile_rules.contains(state));
            }
        }
        grid[((row - 1) * ROWS) + column].options = top_tile.options.clone();

        //check bottom cell
        let bottom_tile = &mut grid[(row+1) * ROWS + column].clone();
        for current_tile_rules in tile_rules {
            if current_tile_rules.tile == current_tile.state {
                let bottom_tile_rules = &current_tile_rules.bottom;
                bottom_tile.options.retain(|state| bottom_tile_rules.contains(state));
            }
        }
        grid[(row+1) * ROWS + column].options = bottom_tile.options.clone();

        //check left cell
        let left_tile = &mut grid[row * ROWS + column - 1].clone();
        for current_tile_rules in tile_rules {
            if current_tile_rules.tile == current_tile.state {
                let left_tile_rules = &current_tile_rules.left;
                left_tile.options.retain(|state| left_tile_rules.contains(state));
            }
        }
        grid[row * ROWS + column - 1].options = left_tile.options.clone();

        //check right cell
        let right_tile = &mut grid[row * ROWS + column + 1];
        for current_tile_rules in tile_rules {
            if current_tile_rules.tile == current_tile.state {
                let right_tile_rules = &current_tile_rules.right;
                right_tile.options.retain(|state| right_tile_rules.contains(state));
            }
        }
        grid[row * ROWS + column + 1].options = right_tile.options.clone();

        //check top_left cell
        let top_left = &mut grid[(row-1) * ROWS + column - 1];
        for current_tile_rules in tile_rules {
            if current_tile_rules.tile == current_tile.state {
                let top_left_tile_rules = &current_tile_rules.top_left;
                top_left.options.retain(|state| top_left_tile_rules.contains(state));
            }
        }
        grid[(row-1) * ROWS + column - 1].options = top_left.options.clone();
        
        //check top_right cell
        let top_right = &mut grid[(row-1) * ROWS + column + 1];
        for current_tile_rules in tile_rules {
            if current_tile_rules.tile == current_tile.state {
                let top_right_tile_rules = &current_tile_rules.top_right;
                top_right.options.retain(|state| top_right_tile_rules.contains(state));
            }
        }
        grid[(row-1) * ROWS + column + 1].options = top_right.options.clone();

        //check bottom_left cell
        let bottom_left = &mut grid[(row+1) * ROWS + column - 1];
        for current_tile_rules in tile_rules {
            if current_tile_rules.tile == current_tile.state {
                let bottom_left_tile_rules = &current_tile_rules.bottom_left;
                bottom_left.options.retain(|state| bottom_left_tile_rules.contains(state));
            }
        }
        grid[(row+1) * ROWS + column - 1].options = bottom_left.options.clone();

        //check bottom_right cell
        let bottom_right = &mut grid[(row+1) * ROWS + column + 1];
        for current_tile_rules in tile_rules {
            if current_tile_rules.tile == current_tile.state {
                let bottom_right_tile_rules = &current_tile_rules.bottom_right;
                bottom_right.options.retain(|state| bottom_right_tile_rules.contains(state));
            }
        }
        grid[(row+1) * ROWS + column + 1].options = bottom_right.options.clone();
        
        grid[(current_tile.y_pos * ROWS) + current_tile.x_pos] = current_tile.clone();
        let lowest_entropy_cell = get_lowest_entropy_cell(grid.clone());
        row = lowest_entropy_cell.y_pos;
        column = lowest_entropy_cell.x_pos;
        current_tile = grid[(row * ROWS) + column].clone();
        current_tile.state = current_tile.options.choose(&mut rand::thread_rng()).unwrap().clone();
        current_tile.collapsed = true;
    }    
    return grid;
}

fn get_lowest_entropy_cell(grid: Vec<Cell>) -> Cell {
    let mut lowest_entropy_cells = Vec::<Cell>::new();
    let mut lowest_entropy = std::usize::MAX;
    for cell in grid {
        if !cell.collapsed{
            if cell.options.len() < lowest_entropy {
                lowest_entropy = cell.options.len();
                lowest_entropy_cells.clear();
                lowest_entropy_cells.push(cell.clone());
                continue;
            }
            if cell.options.len() == lowest_entropy {
                lowest_entropy_cells.push(cell.clone());
                continue;
            }
        }
    }
    let lowest_entropy_cell = lowest_entropy_cells.choose(&mut rand::thread_rng()).unwrap();
    // println!("Lowest entropy cell length: {:?}", lowest_entropy_cell.options.len());
    return lowest_entropy_cell.clone();
}

fn generate_grid() -> Vec<Cell> {
    let mut grid: Vec<Cell> = Vec::with_capacity(ROWS * COLUMNS);
    for row in 0..ROWS {
        for column in 0..COLUMNS {
            let mut cell = Cell {
                state: State::NONE,
                x_pos: column,
                y_pos: row,
                collapsed: false,
                options: vec![],
            };
            if row == 0 || row == ROWS-1 {
                let mut options = vec![State::NONE];
                cell.options.append(&mut options);
                cell.collapsed = true;
            }
            else if column == 0 || column == COLUMNS-1 {
                let mut options = vec![State::NONE];
                cell.options.append(&mut options);
                cell.collapsed = true;
            } else {
                let mut options = vec![
                    State::GRASS,
                    State::MOUNTAIN,
                    State::WATER,
                    State::FOREST
                ];
                cell.options.append(&mut options);
            }
            grid.push(cell);
        }
    }
    return grid;
}

fn generate_tile_rules() -> Vec<Rule> {
    let tile_rules = vec![
        Rule { 
            tile:   State::GRASS, 
            top:    vec![State::GRASS, State::FOREST, State::WATER], 
            right:  vec![State::GRASS, State::FOREST, State::WATER], 
            bottom: vec![State::GRASS, State::FOREST, State::WATER], 
            left:   vec![State::GRASS, State::FOREST, State::WATER],
            top_left: vec![State::GRASS, State::FOREST, State::WATER],
            top_right: vec![State::GRASS, State::FOREST, State::WATER],
            bottom_left: vec![State::GRASS, State::FOREST, State::WATER],
            bottom_right: vec![State::GRASS, State::FOREST, State::WATER],
        },
        Rule { 
            tile:   State::FOREST, 
            top:    vec![State::FOREST, State::GRASS, State::MOUNTAIN], 
            right:  vec![State::FOREST, State::GRASS, State::MOUNTAIN], 
            bottom: vec![State::FOREST, State::GRASS, State::MOUNTAIN], 
            left:   vec![State::FOREST, State::GRASS, State::MOUNTAIN],
            top_left: vec![State::FOREST, State::GRASS, State::MOUNTAIN],
            top_right: vec![State::FOREST, State::GRASS, State::MOUNTAIN],
            bottom_left: vec![State::FOREST, State::GRASS, State::MOUNTAIN],
            bottom_right: vec![State::FOREST, State::GRASS, State::MOUNTAIN],
        },
        Rule { 
            tile:   State::MOUNTAIN, 
            top:    vec![State::MOUNTAIN, State::FOREST], 
            right:  vec![State::MOUNTAIN, State::FOREST], 
            bottom: vec![State::MOUNTAIN, State::FOREST], 
            left:   vec![State::MOUNTAIN, State::FOREST],
            top_left: vec![State::MOUNTAIN, State::FOREST],
            top_right: vec![State::MOUNTAIN, State::FOREST],
            bottom_left: vec![State::MOUNTAIN, State::FOREST],
            bottom_right: vec![State::MOUNTAIN, State::FOREST],
        },
        Rule { 
            tile:   State::WATER, 
            top:    vec![State::WATER, State::GRASS], 
            right:  vec![State::WATER, State::GRASS], 
            bottom: vec![State::WATER, State::GRASS], 
            left:   vec![State::WATER, State::GRASS],
            top_left: vec![State::WATER, State::GRASS],
            top_right: vec![State::WATER, State::GRASS],
            bottom_left: vec![State::WATER, State::GRASS],
            bottom_right: vec![State::WATER, State::GRASS],
        },
        Rule { 
            tile:   State::NONE, 
            top:    vec![State::WATER, State::GRASS, State::MOUNTAIN, State::FOREST], 
            right:  vec![State::WATER, State::GRASS, State::MOUNTAIN, State::FOREST], 
            bottom: vec![State::WATER, State::GRASS, State::MOUNTAIN, State::FOREST], 
            left:   vec![State::WATER, State::GRASS, State::MOUNTAIN, State::FOREST],
            top_left: vec![State::WATER, State::GRASS, State::MOUNTAIN, State::FOREST],
            top_right: vec![State::WATER, State::GRASS, State::MOUNTAIN, State::FOREST],
            bottom_left: vec![State::WATER, State::GRASS, State::MOUNTAIN, State::FOREST],
            bottom_right: vec![State::WATER, State::GRASS, State::MOUNTAIN, State::FOREST],
        },
    ];
    return  tile_rules;
}

// fn convert_state_to_ascii(state: &State) -> &str {
//     let ascii = match state {
//         State::GRASS => "G",
//         State::WATER => "W",
//         State::MOUNTAIN => "M",
//         State::FOREST => "F",
//         State::NONE => "N",
//     };
//     return ascii;
// }