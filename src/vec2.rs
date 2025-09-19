use std::ops::{Add, Mul, Sub};

#[derive(Clone, Copy, Debug)]
pub struct Vec2
{
    pub x: f32,
    pub y: f32,
}

impl Add for Vec2
{
    type Output = Self;
    fn add(self, andra: Self) -> Self
    {
        Vec2 
        {
            x: self.x + andra.x,
            y: self.y + andra.y,
        }
    }
}

impl Sub for Vec2
{
    type Output = Self;
    fn sub(self, andra: Self) -> Self
    {
        Vec2
        {
            x: self.x - andra.x,
            y: self.y - andra.y,
        }
    }
}

impl Mul<f32> for Vec2
{
    type Output = Self;
    fn mul(self, scalar: f32) -> Self
    {
        Vec2
        {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }
}

impl Vec2
{
    pub fn length(&self) -> f32
    {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    pub fn normalize(&self) -> Self
    {
        let l = self.length();
        if l > 0.0 { *self * (1.0 / l) } else { *self }
    }
}
