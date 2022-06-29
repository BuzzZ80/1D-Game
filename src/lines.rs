#[derive(Copy, Clone)]
pub struct Point(pub f32, pub f32);

#[derive(Copy, Clone)]
pub struct Ray {
    pub pos: Point,
    pub dir: Point,
}

// x1 y1 x2 y2
#[derive(Copy, Clone)]
pub struct Segment(pub Point, pub Point);

impl Ray {
    pub fn cast_for_block_at(&self, pos: (f32, f32)) -> Option<Point> {
        let block_segments: [Segment; 4] = [
            // top left to top right
            Segment(
                Point(pos.0, pos.1), 
                Point(pos.0 + 1.0, pos.1)
            ),
            // top left to bottom left
            Segment(
                Point(pos.0, pos.1), 
                Point(pos.0, pos.1 + 1.0)
            ),
            // bottom left to bottom right
            Segment(
                Point(pos.0, pos.1 + 1.0), 
                Point(pos.0 + 1.0, pos.1 + 1.0)
            ),
            // top right to bottom right
            Segment(
                Point(pos.0 + 1.0, pos.1), 
                Point(pos.0 + 1.0, pos.1 + 1.0)
            ),
        ];

        let mut intersection_ts: [Option<f32>; 4] = [None; 4];
        let mut points: [Option<Point>; 4] = [None; 4];

        let (x3, y3, x4, y4) = (
            self.pos.0,
            self.pos.1,
            self.dir.0 + self.pos.0,
            self.dir.1 + self.pos.1,
        );

        for (i, segment) in block_segments.iter().enumerate() {
            let (x1, y1, x2, y2): (f32, f32, f32, f32) = segment.into();
            let denominator = (x1 - x2) * (y3 - y4) - (y1 - y2) * (x3 - x4);

            if denominator == 0.0 {
                continue;
            }

            let t = ((x1 - x3) * (y3 - y4) - (y1 - y3) * (x3 - x4)) / denominator;
            let u = -((x1 - x2) * (y1 - y3) - (y1 - y2) * (x1 - x3)) / denominator;

            if t > 0.0 && t < 1.0 && u > 0.0 {
                intersection_ts[i] = Some(t);
            }
        }
        
        for (i, t) in intersection_ts.iter().enumerate() {
            let t = match t {
                Some(t) => t,
                None => continue,
            };
            let seg = block_segments[i];
            points[i] = Some(Point(
                seg.0.0 + t * (seg.1.0 - seg.0.0),
                seg.0.1 + t * (seg.1.1 - seg.0.1),
            ));
        }

        let mut closest_point = None;
        let mut least_distance = f32::MAX;
        for p in points {
            let p = match p {
                Some(p) => p,
                None => continue,
            };

            let distance = f32::sqrt((p.0 - self.pos.0).powf(2.0) + (p.1 - self.pos.1).powf(2.0));
            if distance < least_distance {
                least_distance = distance;
                closest_point = Some(p);
            }
        }

        closest_point
    }
}

impl std::convert::From<&Segment> for (f32, f32, f32, f32) {
    fn from(seg: &Segment) -> (f32, f32, f32, f32) {
        (seg.0.0, seg.0.1, seg.1.0, seg.1.1)
    }
}