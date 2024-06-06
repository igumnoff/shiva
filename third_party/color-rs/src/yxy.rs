use channel::Channel;
use num_traits::{Float, one, zero};
use color_space::{WhitePoint, D65};
use xyz::{ToXyz, Xyz};
use std::ops::Add;

#[derive(Clone, Copy, Debug)]
pub struct Yxy<T = f32, Wp = D65> {
    pub x: T,
    pub y: T,
    pub luma: T,
    pub white_point: Wp,
}

impl<T: Channel + Float, Wp: WhitePoint> Yxy<T,Wp> {
    pub fn new(x: T, y: T, luma: T) -> Yxy<T,Wp> {
        Yxy {
            x,
            y,
            luma,
            white_point: Wp::default(),
        }
    }
}

pub trait ToYxy{
    type WhitePoint: WhitePoint;
    fn to_yxy<T: Channel + Float>(&self) -> Yxy<T, Self::WhitePoint>;
}

impl<T: Channel + Float, Wp: WhitePoint> ToXyz for Yxy<T,Wp> {
    type WhitePoint = Wp;
    fn to_xyz<U: Channel + Float>(&self) -> Xyz<U, Wp> {
        let y = self.luma;
        let x;
        let z;
        if self.y < zero() || self.y > zero() {
            x = self.luma * self.x / self.y;
            z = self.luma * (one::<T>() - self.x - self.y) / self.y;
        }else{
            x = Wp::xyz().x;
            z = Wp::xyz().z;
        }

        Xyz::new(x.to_channel(), y.to_channel(), z.to_channel())
    }
}

impl<T, Wp> Add<Yxy<T, Wp>> for Yxy<T, Wp>
where
    T: Channel + Float,
    Wp: WhitePoint,
{
    type Output = Yxy<T, Wp>;

    fn add(self, other: Yxy<T, Wp>) -> Self::Output {
        Yxy {
            x: self.x + other.x,
            y: self.y + other.y,
            luma: self.luma + other.luma,
            white_point: Wp::default(),
        }
    }
}

// impl<T, Wp> AddAssign<Yxy<T, Wp>> for Yxy<T, Wp>
// where
//     T: Channel + Float,
//     Wp: WhitePoint,
// {
//     fn add_assign(&mut self, other: Yxy<T, Wp>) {
//         self.x += other.x;
//         self.y += other.y;
//         self.luma += other.luma;
//     }
// }

#[cfg(feature = "bytemuck")]
unsafe impl<T, Wp> bytemuck::Pod for Yxy<T, Wp>
where T: Copy + 'static, Wp: WhitePoint + Copy + 'static {}

#[cfg(feature = "bytemuck")]
unsafe impl<T, Wp> bytemuck::Zeroable for Yxy<T, Wp>
where Wp: WhitePoint {}