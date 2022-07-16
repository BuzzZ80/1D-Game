use crate::barrier::Barrier;
use crate::map::Map;

#[derive(Debug, Copy, Clone)]
pub struct Point(pub f32, pub f32);

#[derive(Debug, Copy, Clone)]
pub struct Direction(pub f32, pub f32);

#[derive(Debug, Copy, Clone)]
pub struct Angle(pub f32);

#[derive(Debug)]
pub struct Intersection {
    pub barrier: Option<Barrier>,
    pub pos: Point,
    pub dist: f32,
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
    pub fn new_towards_point(origin: Point, dest: Point) -> Self {
        Self {
            pos: origin,
            dir: Direction(
                dest.0 - origin.0,
                dest.1 - origin.1,
            ),  
        }
    }

    pub fn cast_for_segment(&self, seg: Segment) -> Option<Intersection> {
        let (x1, y1, x2, y2): (f32, f32, f32, f32) = seg.into();

        let x3 = self.pos.0;
        let y3 = self.pos.1;
        let x4 = self.dir.0 + self.pos.0;
        let y4 = self.dir.1 + self.pos.1;

        let denominator = (x1 - x2) * (y3 - y4) - (y1 - y2) * (x3 - x4);

        if denominator == 0.0 {
            return None;
        }

        let t = ((x1 - x3) * (y3 - y4) - (y1 - y3) * (x3 - x4)) / denominator;
        let u = -((x1 - x2) * (y1 - y3) - (y1 - y2) * (x1 - x3)) / denominator;

        if !(t > 0.0 && t < 1.0 && u > 0.0) {
            return None;
        }

        let pos = Point(
            seg.0 .0 + t * (seg.1 .0 - seg.0 .0),
            seg.0 .1 + t * (seg.1 .1 - seg.0 .1),
        );

        let dist = f32::sqrt((pos.0 - self.pos.0).powi(2) + (pos.1 - self.pos.1).powi(2));

        let (w1, w2) = (self.dir.0, self.dir.1);
        let v1 = seg.1 .0 - seg.0 .0;
        let v2 = seg.1 .1 - seg.0 .1;
        let x = w2 * v1 - w1 * v2;
        let y = w1 * v1 + w2 * v2;
        let angle = Angle(f32::atan2(x, y).abs());

        Some(Intersection {
            barrier: None,
            pos,
            dist,
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
                Some(Intersection { dist, .. }) if current_intersection.dist > dist => {
                }
                _ => closest_intersection = Some(current_intersection),
            };
        }

        closest_intersection
    }
}

impl Segment {
    pub fn get_angle(&self) -> Angle {
        let rise = self.1.1 - self.0.1;
        let run = self.1.0 - self.0.0;
        Angle(f32::atan2(rise, run))
    }
}

impl Point {
    pub fn can_see(&self, env: &Map, other: Point) -> bool {
        let ray = Ray::new_towards_point(*self, other);
        match ray.cast_for_map(env) {
            Some(intersection) => {
                intersection.dist >= self.distance_from(other) - 0.01
            },
            None => true,
        }
    }

    pub fn distance_from(&self, other: Point) -> f32 {
        ((other.0 - self.0).powi(2) + (other.1 - self.1).powi(2)).sqrt()
    }
}

impl std::convert::From<Direction> for Angle {
    fn from(dir: Direction) -> Angle {
        Angle(f32::atan2(dir.1, dir.0))
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
