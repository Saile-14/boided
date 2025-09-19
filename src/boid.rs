use crate::vec2::Vec2;

const WINDOW_WIDTH: f32 = 1200.0;
const WINDOW_HEIGHT: f32 = 800.0;
const SPEED_LIMIT: f32 = 4.0;
const PERCEPTION_RADIUS: f32 = 75.0;
const ALIGN_FACTOR: f32 = 0.5;
const COHESION_FACTOR: f32 = 0.05;
const SEPARATION_FACTOR: f32 = 0.1;

#[derive(Clone)]
pub struct Boid
{
    pub pos: Vec2,
    pub vel: Vec2,
}

impl Boid
{
    fn bounce(&mut self)
    {
        if self.pos.x < 0.0 || self.pos.x > WINDOW_WIDTH { self.vel.x = -self.vel.x; }
        if self.pos.y < 0.0 || self.pos.y > WINDOW_HEIGHT { self.vel.y = -self.vel.y }
    }

    fn avoid_edges(&mut self)
    {
        let margin = 50.0;
        let turn_factor = 0.3;

        if self.pos.x < margin
        {
            self.vel.x += turn_factor;
        }
        else if self.pos.x > WINDOW_WIDTH - margin
        {
            self.vel.x -= turn_factor;
        }

        if self.pos.y < margin
        {
            self.vel.y += turn_factor;
        }
        else if self.pos.y > WINDOW_HEIGHT - margin
        {
            self.vel.y -= turn_factor;
        }
    }

    pub fn update(&mut self, boids: &Vec<Boid>)
    {
        let perception = PERCEPTION_RADIUS;
        let mut align = Vec2 { x: 0.0, y: 0.0 };
        let mut cohesion = Vec2 { x: 0.0, y: 0.0 };
        let mut separation = Vec2 { x: 0.0, y: 0.0 };
        let mut total = 0;

        for other in boids
        {
            let d = (other.pos - self.pos).length();

            if d > 0.0 && d < perception
            {
                align = align + other.vel;
                cohesion = cohesion + other.pos;
                separation = separation + (self.pos - other.pos) * (1.0 / d);
                total += 1;
            }
        }

        if total > 0
        {
            align = (align * (1.0 / total as f32)).normalize() * ALIGN_FACTOR;
            cohesion = ((cohesion * (1.0 / total as f32)) - self.pos).normalize() * COHESION_FACTOR;
            separation = (separation * (1.0 / total as f32)).normalize() * SEPARATION_FACTOR;
        }

        self.vel = self.vel + align + cohesion + separation;

        if self.vel.length() > SPEED_LIMIT
        {
            self.vel = self.vel.normalize() * SPEED_LIMIT;
        }

        self.pos = self.pos + self.vel;
        self.bounce();
        self.avoid_edges();
    }
}
