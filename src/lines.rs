use crate::map::Map;
use crate::barrier::Barrier;

#[derive(Debug, Copy, Clone)]
pub struct Point(pub f32, pub f32);

#[derive(Debug, Copy, Clone)]
pub struct Direction(pub f32, pub f32);

#[derive(Debug, Copy, Clone)]
pub struct Angle(pub f32);

#[derive(Debug)]
pub struct Intersection {
    pub barrier: Option<Barrier>,
    pub point: Point,
    pub distance: f32,
    pub angle: Angle,
}

#[derive(Debug, Copy, Clone)]
pub struct Ray {
    pub pos: Point,
    pub dir: Direction,
}

// x1 y1 x2 y2
#[derive(Debug, Copy, Clone)]
pub struct Segment(pub Point, pub Point);

impl Ray {
    pub fn cast_for_segment(&self, seg: Segment) -> Option<Intersection> {
        let (x3, y3, x4, y4) = (
            self.pos.0,
            self.pos.1,
            self.dir.0 + self.pos.0,
            self.dir.1 + self.pos.1,
        );

        let (x1, y1, x2, y2): (f32, f32, f32, f32) = seg.into();
        let denominator = (x1 - x2) * (y3 - y4) - (y1 - y2) * (x3 - x4);

        if denominator == 0.0 {
            return None;
        }

        let t = ((x1 - x3) * (y3 - y4) - (y1 - y3) * (x3 - x4)) / denominator;
        let u = -((x1 - x2) * (y1 - y3) - (y1 - y2) * (x1 - x3)) / denominator;

        if !(t > 0.0 && t < 1.0 && u > 0.0) {
            return None;
        }

        let point = Point(
            seg.0 .0 + t * (seg.1 .0 - seg.0 .0),
            seg.0 .1 + t * (seg.1 .1 - seg.0 .1),
        );

        let distance =
            f32::sqrt((point.0 - self.pos.0).powf(2.0) + (point.1 - self.pos.1).powf(2.0));

        let (w1, w2) = (self.dir.0, self.dir.1);
        let v1 = seg.1.0 - seg.0.0;
        let v2 = seg.1.1 - seg.0.1;
        let x = w2 * v1 - w1 * v2;
        let y = w1 * v1 + w2 * v2;
        let angle = Angle(f32::atan2(x, y).abs());

        Some(Intersection {
            barrier: None,
            point,
            distance,
            angle,
        })
    }

    pub fn cast_for_map(&self, env: &Map) -> Option<Intersection> {
        let mut closest_intersection: Option<Intersection> = None;

        for bar in &env.barriers {
            let current_intersection = match self.cast_for_segment(bar.seg) {
                Some(mut intersection) => {
                    intersection.barrier = Some(*bar);
                    intersection
                }
                None => continue,
            };

            match closest_intersection {
                Some(Intersection{distance, ..}) if current_intersection.distance > distance => {}
                _ => closest_intersection = Some(current_intersection),
            };
        };

        closest_intersection
    }
}

impl std::convert::From<Segment> for (f32, f32, f32, f32) {
    fn from(seg: Segment) -> (f32, f32, f32, f32) {
        (seg.0 .0, seg.0 .1, seg.1 .0, seg.1 .1)
    }
}

impl std::ops::Add<Point> for Point {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Point(self.0 + other.0, self.1 + other.1)
    }
}