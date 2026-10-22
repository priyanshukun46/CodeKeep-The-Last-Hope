use super::path::MapPath;
use bevy::prelude::*;
use std::collections::HashSet;

pub const CELL_SIZE: f32 = 40.0;
pub const PATH_CLEARANCE: f32 = 36.0;
pub const REFUGE_CLEARANCE: f32 = 50.0;

/// Grid coordinates representing discretized battlefield construction cells.
pub type GridCoord = (i32, i32);

/// Converts a continuous 2D world position into discrete grid coordinates.
pub fn world_to_grid(world_pos: Vec2) -> GridCoord {
    (
        (world_pos.x / CELL_SIZE).round() as i32,
        (world_pos.y / CELL_SIZE).round() as i32,
    )
}

/// Converts discrete grid coordinates back to the center of the world cell.
pub fn grid_to_world(coord: GridCoord) -> Vec2 {
    Vec2::new(coord.0 as f32 * CELL_SIZE, coord.1 as f32 * CELL_SIZE)
}

/// Set of all grid cells currently occupied by towers or structures.
#[derive(Resource, Default, Debug, Clone)]
pub struct OccupiedCells {
    pub cells: HashSet<GridCoord>,
}

impl OccupiedCells {
    pub fn is_occupied(&self, coord: GridCoord) -> bool {
        self.cells.contains(&coord)
    }

    pub fn occupy(&mut self, coord: GridCoord) -> bool {
        self.cells.insert(coord)
    }

    #[allow(dead_code)]
    pub fn free(&mut self, coord: GridCoord) -> bool {
        self.cells.remove(&coord)
    }
}

/// Validates whether a defensive tower can be constructed at the specified grid coordinate.
pub fn can_build(
    coord: GridCoord,
    world_pos: Vec2,
    occupied: &OccupiedCells,
    path: &MapPath,
    refuge_pos: Vec2,
) -> bool {
    // 1. Cannot build on an already occupied cell
    if occupied.is_occupied(coord) {
        return false;
    }

    // 2. Cannot build within the playable boundaries
    if world_pos.x.abs() > 560.0 || world_pos.y.abs() > 280.0 {
        return false;
    }

    // 3. Cannot obstruct the enemy path corridor
    if path.is_near_path(world_pos, PATH_CLEARANCE) {
        return false;
    }

    // 4. Cannot build directly on top of the human Refuge core
    if world_pos.distance(refuge_pos) < REFUGE_CLEARANCE {
        return false;
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grid_conversion_bidirectional() {
        let coord = (5, -3);
        let world = grid_to_world(coord);
        assert_eq!(world, Vec2::new(200.0, -120.0));
        assert_eq!(world_to_grid(world), coord);
    }

    #[test]
    fn test_can_build_rules() {
        let path = MapPath::default();
        let mut occupied = OccupiedCells::default();
        let refuge_pos = Vec2::new(480.0, 120.0);

        // A valid open build spot
        let valid_coord = (0, 0);
        let valid_world = grid_to_world(valid_coord);
        assert!(can_build(
            valid_coord,
            valid_world,
            &occupied,
            &path,
            refuge_pos
        ));

        // Mark as occupied
        occupied.occupy(valid_coord);
        assert!(!can_build(
            valid_coord,
            valid_world,
            &occupied,
            &path,
            refuge_pos
        ));

        // Directly on top of path
        let on_path = (-6, 4); // (-240, 160) which is waypoint 1
        assert!(!can_build(
            on_path,
            grid_to_world(on_path),
            &occupied,
            &path,
            refuge_pos
        ));

        // Directly on top of refuge
        let on_refuge = world_to_grid(refuge_pos);
        assert!(!can_build(
            on_refuge,
            grid_to_world(on_refuge),
            &occupied,
            &path,
            refuge_pos
        ));
    }
}
