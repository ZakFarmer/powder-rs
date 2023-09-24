#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vector2d {
    pub x: f32,
    pub y: f32,
}

impl Vector2d {
    pub fn dot(&self, rhs: Vector2d) -> f32 {
        self.x * rhs.x + self.y * rhs.y
    }

    pub fn magnitude(&self) -> f32 {
        self.dot(*self).sqrt()
    }

    pub fn normalize(&self) -> Vector2d {
        *self / self.magnitude()
    }

    pub fn new(x: f32, y: f32) -> Vector2d {
        Vector2d { x, y }
    }

    pub fn to_ndc(&self, screen_width: f32, screen_height: f32) -> Vector2d {
        Vector2d {
            x: 2.0 * self.x / screen_width - 1.0,
            y: 1.0 - 2.0 * self.y / screen_height, // y is inverted as the origin is at the top-left
        }
    }
}

impl Into<[f32; 2]> for Vector2d {
    fn into(self) -> [f32; 2] {
        [self.x, self.y]
    }
}

impl std::ops::Add for Vector2d {
    type Output = Vector2d;

    fn add(self, rhs: Vector2d) -> Vector2d {
        Vector2d {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl std::ops::Sub for Vector2d {
    type Output = Vector2d;

    fn sub(self, rhs: Vector2d) -> Vector2d {
        Vector2d {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl std::ops::Mul<f32> for Vector2d {
    type Output = Vector2d;

    fn mul(self, rhs: f32) -> Vector2d {
        Vector2d {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl std::ops::Div<f32> for Vector2d {
    type Output = Vector2d;

    fn div(self, rhs: f32) -> Vector2d {
        self * (1.0 / rhs)
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vector3d {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector3d {
    pub fn dot(&self, rhs: Vector3d) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub fn magnitude(&self) -> f32 {
        self.dot(*self).sqrt()
    }

    pub fn normalize(&self) -> Vector3d {
        *self / self.magnitude()
    }

    pub fn new(x: f32, y: f32, z: f32) -> Vector3d {
        Vector3d { x, y, z }
    }
}

impl std::ops::Add for Vector3d {
    type Output = Vector3d;

    fn add(self, rhs: Vector3d) -> Vector3d {
        Vector3d {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl std::ops::Sub for Vector3d {
    type Output = Vector3d;

    fn sub(self, rhs: Vector3d) -> Vector3d {
        Vector3d {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl std::ops::Mul<f32> for Vector3d {
    type Output = Vector3d;

    fn mul(self, rhs: f32) -> Vector3d {
        Vector3d {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl std::ops::Div<f32> for Vector3d {
    type Output = Vector3d;

    fn div(self, rhs: f32) -> Vector3d {
        self * (1.0 / rhs)
    }
}