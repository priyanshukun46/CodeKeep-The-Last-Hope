use bevy::prelude::*;
use bevy::window::PrimaryWindow;

/// Central dimensions for standard 2:1 isometric diamond tiles.
pub const ISO_TILE_WIDTH: f32 = 64.0;
pub const ISO_TILE_HEIGHT: f32 = 32.0;

/// Discrete 2D integer coordinates on the isometric grid.
pub type IsoCoord = (i32, i32);

/// Converts discrete isometric grid coordinates (gx, gy) into continuous 2D world space coordinates.
///
/// Under this 2.5D standard projection:
/// - Moving +X (gx + 1) steps right (+W/2) and down (-H/2).
/// - Moving +Y (gy + 1) steps left (-W/2) and down (-H/2).
/// - Tile (0, 0) is centered at world (0.0, 0.0).
#[inline]
pub fn iso_grid_to_world(gx: i32, gy: i32) -> Vec2 {
    let half_w = ISO_TILE_WIDTH * 0.5;
    let half_h = ISO_TILE_HEIGHT * 0.5;
    Vec2::new((gx - gy) as f32 * half_w, -((gx + gy) as f32) * half_h)
}

/// Converts continuous 2D world coordinates (e.g. from mouse raycast) into discrete isometric grid coordinates.
///
/// Uses the exact analytical inverse of `iso_grid_to_world`:
///   gx = wx / W - wy / H
///   gy = -wx / W - wy / H
#[inline]
pub fn iso_world_to_grid(world_pos: Vec2) -> IsoCoord {
    let gx = (world_pos.x / ISO_TILE_WIDTH) - (world_pos.y / ISO_TILE_HEIGHT);
    let gy = (-world_pos.x / ISO_TILE_WIDTH) - (world_pos.y / ISO_TILE_HEIGHT);
    (gx.round() as i32, gy.round() as i32)
}

/// Computes the 4 outer diamond vertices of an isometric tile centered at `center`.
/// Order: Top, Right, Bottom, Left.
#[inline]
pub fn iso_tile_vertices(center: Vec2) -> [Vec2; 4] {
    let half_w = ISO_TILE_WIDTH * 0.5;
    let half_h = ISO_TILE_HEIGHT * 0.5;
    [
        Vec2::new(center.x, center.y + half_h), // Top
        Vec2::new(center.x + half_w, center.y), // Right
        Vec2::new(center.x, center.y - half_h), // Bottom
        Vec2::new(center.x - half_w, center.y), // Left
    ]
}

/// Terrain surface classification for ruined-world isometric tiles.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IsoTerrain {
    RuinConcrete,
    CrackedAsphalt,
    OvergrownSoil,
    HazardScrap,
}

impl IsoTerrain {
    pub fn edge_color(&self) -> Color {
        match self {
            IsoTerrain::RuinConcrete => Color::srgba(0.28, 0.32, 0.38, 0.65),
            IsoTerrain::CrackedAsphalt => Color::srgba(0.20, 0.23, 0.28, 0.65),
            IsoTerrain::OvergrownSoil => Color::srgba(0.22, 0.30, 0.24, 0.65),
            IsoTerrain::HazardScrap => Color::srgba(0.35, 0.28, 0.20, 0.65),
        }
    }
}

/// Small visual test map containing predefined sector tiles.
#[derive(Resource)]
pub struct IsoMap {
    pub min_gx: i32,
    pub max_gx: i32,
    pub min_gy: i32,
    pub max_gy: i32,
}

impl Default for IsoMap {
    fn default() -> Self {
        Self {
            min_gx: -4,
            max_gx: 4,
            min_gy: -4,
            max_gy: 4,
        }
    }
}

impl IsoMap {
    pub fn contains(&self, coord: IsoCoord) -> bool {
        coord.0 >= self.min_gx
            && coord.0 <= self.max_gx
            && coord.1 >= self.min_gy
            && coord.1 <= self.max_gy
    }

    pub fn terrain_at(&self, coord: IsoCoord) -> IsoTerrain {
        let (x, y) = coord;
        if (x + y) % 3 == 0 {
            IsoTerrain::OvergrownSoil
        } else if (x - y).abs() <= 1 {
            IsoTerrain::CrackedAsphalt
        } else if (x * y) % 2 == 0 {
            IsoTerrain::RuinConcrete
        } else {
            IsoTerrain::HazardScrap
        }
    }
}

/// Tracks the isometric grid tile currently under the player's cursor.
#[derive(Resource, Default, Debug)]
pub struct HoveredIsoTile(pub Option<IsoCoord>);

/// Tracks the last tile clicked by the player.
#[derive(Resource, Default, Debug)]
pub struct SelectedIsoTile(pub Option<IsoCoord>);

/// Marker component for the isometric debug HUD overlay.
#[derive(Component)]
pub struct IsoDebugText;

pub struct IsoPlugin;

impl Plugin for IsoPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<IsoMap>()
            .init_resource::<HoveredIsoTile>()
            .init_resource::<SelectedIsoTile>()
            .add_systems(Startup, setup_iso_debug_ui)
            .add_systems(
                Update,
                (
                    update_iso_hover_and_click_system,
                    render_iso_grid_system,
                    update_iso_debug_ui_system,
                ),
            );
    }
}

/// Spawns a small, readable isometric coordinate readout in the corner of the screen.
fn setup_iso_debug_ui(mut commands: Commands) {
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                bottom: Val::Px(48.0),
                left: Val::Px(16.0),
                padding: UiRect::axes(Val::Px(12.0), Val::Px(6.0)),
                border: UiRect::all(Val::Px(1.0)),
                ..default()
            },
            BackgroundColor(Color::srgba(0.06, 0.08, 0.12, 0.85)),
            BorderColor(Color::srgba(0.3, 0.45, 0.6, 0.5)),
        ))
        .with_children(|parent| {
            parent.spawn((
                IsoDebugText,
                Text::new("ISO GRID: Hovered: [None] | Selected: [None]"),
                TextFont {
                    font_size: 13.0,
                    ..default()
                },
                TextColor(Color::srgb(0.35, 0.85, 1.0)),
            ));
        });
}

/// Updates mouse hover detection and handles left-clicks on isometric tiles.
fn update_iso_hover_and_click_system(
    mut hovered: ResMut<HoveredIsoTile>,
    mut selected: ResMut<SelectedIsoTile>,
    iso_map: Res<IsoMap>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform), With<Camera2d>>,
) {
    let Ok(window) = window_query.get_single() else {
        hovered.0 = None;
        return;
    };
    let Ok((camera, camera_transform)) = camera_query.get_single() else {
        hovered.0 = None;
        return;
    };

    if let Some(cursor_pos) = window.cursor_position() {
        if let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_pos) {
            let coord = iso_world_to_grid(world_pos);
            if iso_map.contains(coord) {
                hovered.0 = Some(coord);

                if mouse_button.just_pressed(MouseButton::Left) {
                    selected.0 = Some(coord);
                }
                return;
            }
        }
    }

    hovered.0 = None;
}

/// Renders the isometric grid diamond tiles and active selection highlights using Bevy Gizmos.
fn render_iso_grid_system(
    mut gizmos: Gizmos,
    iso_map: Res<IsoMap>,
    hovered: Res<HoveredIsoTile>,
    selected: Res<SelectedIsoTile>,
) {
    // 1. Render isometric grid diamond outlines
    for gx in iso_map.min_gx..=iso_map.max_gx {
        for gy in iso_map.min_gy..=iso_map.max_gy {
            let center = iso_grid_to_world(gx, gy);
            let vertices = iso_tile_vertices(center);
            let terrain = iso_map.terrain_at((gx, gy));

            // Draw diamond outline
            let strip = [
                vertices[0],
                vertices[1],
                vertices[2],
                vertices[3],
                vertices[0],
            ];
            gizmos.linestrip_2d(strip, terrain.edge_color());

            // Center anchor dot
            gizmos.circle_2d(center, 1.5, Color::srgba(0.35, 0.45, 0.55, 0.4));
        }
    }

    // 2. Render persistent selected tile highlight (Gold)
    if let Some(sel_coord) = selected.0 {
        if iso_map.contains(sel_coord) {
            let center = iso_grid_to_world(sel_coord.0, sel_coord.1);
            let vertices = iso_tile_vertices(center);
            let gold = Color::srgb(1.0, 0.8, 0.2);
            let strip = [
                vertices[0],
                vertices[1],
                vertices[2],
                vertices[3],
                vertices[0],
            ];
            gizmos.linestrip_2d(strip, gold);
            gizmos.circle_2d(center, 3.5, gold);
        }
    }

    // 3. Render active hover highlight (Bright Cyan)
    if let Some(hov_coord) = hovered.0 {
        let center = iso_grid_to_world(hov_coord.0, hov_coord.1);
        let vertices = iso_tile_vertices(center);
        let cyan = Color::srgb(0.2, 0.95, 1.0);
        let strip = [
            vertices[0],
            vertices[1],
            vertices[2],
            vertices[3],
            vertices[0],
        ];
        gizmos.linestrip_2d(strip, cyan);
        gizmos.circle_2d(center, 2.5, cyan);
    }
}

/// Updates the debug HUD readout text with the active hovered and selected tile coordinates.
fn update_iso_debug_ui_system(
    hovered: Res<HoveredIsoTile>,
    selected: Res<SelectedIsoTile>,
    mut query: Query<&mut Text, With<IsoDebugText>>,
) {
    if let Ok(mut text) = query.get_single_mut() {
        let hov_str = match hovered.0 {
            Some((x, y)) => format!("({}, {})", x, y),
            None => "None".to_string(),
        };
        let sel_str = match selected.0 {
            Some((x, y)) => format!("({}, {})", x, y),
            None => "None".to_string(),
        };
        **text = format!("ISO GRID: Hovered: {} | Selected: {}", hov_str, sel_str);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iso_origin_center() {
        let world = iso_grid_to_world(0, 0);
        assert_eq!(world, Vec2::ZERO);
        assert_eq!(iso_world_to_grid(Vec2::ZERO), (0, 0));
    }

    #[test]
    fn test_iso_axes_cardinal_directions() {
        let half_w = ISO_TILE_WIDTH * 0.5;
        let half_h = ISO_TILE_HEIGHT * 0.5;

        // +gx moves down-right (+X, -Y)
        assert_eq!(iso_grid_to_world(1, 0), Vec2::new(half_w, -half_h));
        // +gy moves down-left (-X, -Y)
        assert_eq!(iso_grid_to_world(0, 1), Vec2::new(-half_w, -half_h));
        // -gx moves up-left (-X, +Y)
        assert_eq!(iso_grid_to_world(-1, 0), Vec2::new(-half_w, half_h));
        // -gy moves up-right (+X, +Y)
        assert_eq!(iso_grid_to_world(0, -1), Vec2::new(half_w, half_h));
    }

    #[test]
    fn test_iso_round_trip_grid_to_world_to_grid() {
        for gx in -25..=25 {
            for gy in -25..=25 {
                let world = iso_grid_to_world(gx, gy);
                let round_trip = iso_world_to_grid(world);
                assert_eq!(
                    round_trip,
                    (gx, gy),
                    "Failed round trip for grid coord ({}, {}) -> world {:?}",
                    gx,
                    gy,
                    world
                );
            }
        }
    }

    #[test]
    fn test_iso_interior_points_map_to_tile() {
        let center = iso_grid_to_world(3, -2);
        // Small offsets in each quadrant within diamond radius
        let offsets = [
            Vec2::new(5.0, 0.0),
            Vec2::new(-5.0, 0.0),
            Vec2::new(0.0, 3.0),
            Vec2::new(0.0, -3.0),
            Vec2::new(4.0, 2.0),
            Vec2::new(-4.0, -2.0),
        ];

        for offset in offsets {
            let test_point = center + offset;
            assert_eq!(
                iso_world_to_grid(test_point),
                (3, -2),
                "Offset {:?} from center {:?} mapped to wrong tile",
                offset,
                center
            );
        }
    }

    #[test]
    fn test_iso_tile_vertices_geometry() {
        let center = Vec2::new(100.0, 50.0);
        let verts = iso_tile_vertices(center);
        let half_w = ISO_TILE_WIDTH * 0.5;
        let half_h = ISO_TILE_HEIGHT * 0.5;

        assert_eq!(verts[0], Vec2::new(100.0, 50.0 + half_h)); // Top
        assert_eq!(verts[1], Vec2::new(100.0 + half_w, 50.0)); // Right
        assert_eq!(verts[2], Vec2::new(100.0, 50.0 - half_h)); // Bottom
        assert_eq!(verts[3], Vec2::new(100.0 - half_w, 50.0)); // Left
    }
}
