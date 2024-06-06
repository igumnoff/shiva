use channel::Channel;
use color_space::WhitePoint;
use num_traits::{Float, NumCast, cast, zero};
use xyz::{Xyz, ToXyz};
use std::ops::{Add, Mul};

#[derive(Clone, Copy, Debug)]
pub struct Lab<T, Wp>{
    pub l: T,
    pub a: T,
    pub b: T,
    pub white_point: Wp,
}

impl<T, Wp: WhitePoint> Lab<T, Wp>{
    pub fn new(l: T, a: T, b: T) -> Lab<T, Wp>{
        Lab { l, a, b, white_point: Wp::default() }
    }
}


impl<T: Copy, Wp: WhitePoint> Lab<T, Wp>{
    pub fn brightness(&self) -> T {
        self.l
    }
}

impl<T: Float, Wp: WhitePoint> Lab<T, Wp>{
    pub fn chromacity(&self) -> T {
        (self.a.powi(2) + self.b.powi(2)).sqrt()
    }

    pub fn hue(&self) -> T {
        let h = self.b.atan2(self.a);
        if h < zero() {
            h + cast(std::f64::consts::TAU).unwrap()
        }else{
            h
        }
    }

    pub fn offset_chromacity(&self, chroma_offset: T) -> Lab<T, Wp>{
        let current_croma = self.chromacity();
        let offset_a = self.a / current_croma * chroma_offset;
        let offset_b = self.b / current_croma * chroma_offset;
        Lab::new(
            self.l,
            self.a + offset_a,
            self.b + offset_b,
        )
    }
}

pub trait ToLab {
    type WhitePoint: WhitePoint;
    fn to_lab<T: Channel>(&self) -> Lab<T, Self::WhitePoint>;
}

impl<T: Channel + Float + NumCast, Wp: WhitePoint> ToXyz for Lab<T, Wp> {
    type WhitePoint = Wp;
    fn to_xyz<U: Channel + Float>(&self) -> Xyz<U, Wp> {
        let fy = (self.l + cast(16).unwrap()) / cast(116).unwrap();
        let fx = self.a / cast(500).unwrap() + fy;
        let fz = fy - self.b / cast(200).unwrap();
        let fxcb=fx*fx*fx;
        let fzcb=fz*fz*fz;
        let mut xyz = [fxcb, cast(0.).unwrap(), fzcb];
        let eps= cast(216.0 / 24389.).unwrap(); // See BruceLindbloom.com
        if fxcb <= eps {
            xyz[0] = (cast::<f64,T>(108.0).unwrap() * fx / cast(841).unwrap()) - cast::<f64,T>(432.0).unwrap() / cast(24389.).unwrap()
        };
        if fzcb <= eps{
             xyz[2] = (cast::<f64,T>(108.0).unwrap() * fz / cast(841).unwrap()) - cast::<f64,T>(432.0).unwrap() / cast(24389.).unwrap()
        }
        if self.l > cast(8.).unwrap() { // See BruceLindbloom.com
            xyz[1]=fy.powi(3)
        }else{
            xyz[1]=self.l * cast(27.0).unwrap() / cast(24389).unwrap(); // See BruceLindbloom.com
        }
        xyz[0] = xyz[0] * Wp::xyz().x;
        xyz[1] = xyz[1] * Wp::xyz().y;
        xyz[2] = xyz[2] * Wp::xyz().z;

        Xyz::new(xyz[0].to_channel(), xyz[1].to_channel(), xyz[2].to_channel())
    }
}

impl<T: Channel + Float + NumCast, Wp: WhitePoint> Add for Lab<T,Wp>{
    type Output = Lab<T, Wp>;
    fn add(self, other: Lab<T, Wp>) -> Lab<T, Wp> {
        Lab::new(self.l + other.l, self.a + other.a, self.b + other.b)
    }
}

impl<T: Channel + Float + NumCast, Wp: WhitePoint> Mul<T> for Lab<T,Wp>{
    type Output = Lab<T, Wp>;
    fn mul(self, other: T) -> Lab<T, Wp> {
        Lab::new(self.l * other, self.a * other, self.b * other)
    }
}

#[cfg(feature = "bytemuck")]
unsafe impl<T, Wp> bytemuck::Pod for Lab<T, Wp>
where T: Copy + 'static, Wp: WhitePoint + Copy + 'static {}

#[cfg(feature = "bytemuck")]
unsafe impl<T, Wp> bytemuck::Zeroable for Lab<T, Wp>
where Wp: WhitePoint {}