use bevy::prelude::*;

/// The predefined pathway that hostile creatures follow toward the human Refuge.
#[derive(Resource, Debug, Clone)]
pub struct MapPath {
    pub waypoints: Vec<Vec2>,
}

impl Default for MapPath {
    fn default() -> Self {
        Self {
            waypoints: vec![
                Vec2::new(-540.0, 160.0),  // Spawn Portal
                Vec2::new(-240.0, 160.0),  // Turn south
                Vec2::new(-240.0, -120.0), // Turn east
                Vec2::new(120.0, -120.0),  // Turn north
                Vec2::new(120.0, 120.0),   // Turn east toward refuge
                Vec2::new(460.0, 120.0),   // Refuge Gate
            ],
        }
    }
}

impl MapPath {
    #[allow(dead_code)]
    pub fn new(waypoints: Vec<Vec2>) -> Self {
        Self { waypoints }
    }

    pub fn spawn_point(&self) -> Vec2 {
        *self.waypoints.first().unwrap_or(&Vec2::ZERO)
    }

    #[allow(dead_code)]
    pub fn refuge_gate(&self) -> Vec2 {
        *self.waypoints.last().unwrap_or(&Vec2::ZERO)
    }

    /// Calculates the shortest Euclidean distance from a test point to a line segment [a, b].
    pub fn distance_to_segment(point: Vec2, a: Vec2, b: Vec2) -> f32 {
        let ab = b - a;
        let len_sq = ab.length_squared();
        if len_sq == 0.0 {
            return point.distance(a);
        }
        let t = ((point - a).dot(ab) / len_sq).clamp(0.0, 1.0);
        let projection = a + ab * t;
        point.distance(projection)
    }

    /// Checks whether a given point is within `clearance` distance of any segment along the path.
    pub fn is_near_path(&self, point: Vec2, clearance: f32) -> bool {
        for window in self.waypoints.windows(2) {
            let (a, b) = (window[0], window[1]);
            if Self::distance_to_segment(point, a, b) < clearance {
                return true;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distance_to_segment() {
        let a = Vec2::new(0.0, 0.0);
        let b = Vec2::new(10.0, 0.0);

        // Point directly perpendicular to middle of segment
        let p1 = Vec2::new(5.0, 3.0);
        assert!((MapPath::distance_to_segment(p1, a, b) - 3.0).abs() < 1e-4);

        // Point before segment start
        let p2 = Vec2::new(-4.0, 0.0);
        assert!((MapPath::distance_to_segment(p2, a, b) - 4.0).abs() < 1e-4);

        // Point after segment end
        let p3 = Vec2::new(14.0, 3.0);
        assert!((MapPath::distance_to_segment(p3, a, b) - 5.0).abs() < 1e-4);
    }

    #[test]
    fn test_is_near_path() {
        let path = MapPath::default();

        // Exactly on waypoint 0
        assert!(path.is_near_path(Vec2::new(-540.0, 160.0), 30.0));

        // On segment between (-540, 160) and (-240, 160)
        assert!(path.is_near_path(Vec2::new(-400.0, 170.0), 20.0));

        // Far away in open terrain
        assert!(!path.is_near_path(Vec2::new(0.0, 0.0), 40.0));
    }
}
