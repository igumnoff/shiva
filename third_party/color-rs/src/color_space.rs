use xyz::Xyz;
use yxy::Yxy;
use channel::Channel;
use rgb::Rgb;
use num_traits::{Float, cast};
use std::ops::{Mul, Index};


pub trait MatrixColorSpace{
    type WhitePoint: WhitePoint;
    fn red<T: Channel + Float>() -> Yxy<T, D50>;
    fn green<T: Channel + Float>() -> Yxy<T, D50>;
    fn blue<T: Channel + Float>() -> Yxy<T, D50>;
    fn magenta<T: Channel + Float>() -> Yxy<T, D50>{
        Self::red() + Self::blue()
    }
    fn yellow<T: Channel + Float>() -> Yxy<T, D50>{
        Self::red() + Self::green()
    }
    fn cyan<T: Channel + Float>() -> Yxy<T, D50>{
        Self::green() + Self::blue()
    }

    fn to_xyz_matrix<T: Channel + Float>() -> Mat3<T>;
    fn to_rgb_matrix<T: Channel + Float>() -> Mat3<T>;
}

pub trait TransferFunction: 'static{
    fn from_linear<T: Float>(x: T) -> T;
    fn to_linear<T: Float>(x: T) -> T;
}

pub trait WhitePoint: Default{
    fn xyz<T: Channel + Float>() -> Xyz<T,D50>;
}

#[derive(Default, Clone, Copy, Debug)]
pub struct A;
#[derive(Default, Clone, Copy, Debug)]
pub struct D50;
#[derive(Default, Clone, Copy, Debug)]
pub struct D55;
#[derive(Default, Clone, Copy, Debug)]
pub struct D65;
#[derive(Default, Clone, Copy, Debug)]
pub struct D75;
#[derive(Default, Clone, Copy, Debug)]
pub struct E;

// Incandescent / Tungsten
impl WhitePoint for A {
    fn xyz<T: Channel + Float>() -> Xyz<T,D50> {
        Xyz::new(1.09850.to_channel(), 1.0.to_channel(), 0.35585.to_channel())
    }
}

// ICC profile PCS
impl WhitePoint for D50 {
    fn xyz<T: Channel + Float>() -> Xyz<T,D50> {
        Xyz::new(0.96422.to_channel(), 1.0.to_channel(), 0.8251.to_channel())
    }
}

// Mid-morning daylight
impl WhitePoint for D55 {
    fn xyz<T: Channel + Float>() -> Xyz<T,D50> {
        Xyz::new(0.95682.to_channel(), 1.0.to_channel(), 0.92149.to_channel())
    }
}

// Daylight, sRGB, Adobe-RGB
impl WhitePoint for D65 {
    fn xyz<T: Channel + Float>() -> Xyz<T,D50> {
        Xyz::new(0.95047.to_channel(), 1.0.to_channel(), 1.08883.to_channel())
    }
}

// North sky daylight
impl WhitePoint for D75 {
    fn xyz<T: Channel + Float>() -> Xyz<T,D50> {
        Xyz::new(0.94972.to_channel(), 1.0.to_channel(), 1.22638.to_channel())
    }
}

// Equal energy
impl WhitePoint for E {
    fn xyz<T: Channel + Float>() -> Xyz<T,D50> {
        Xyz::new(1.0.to_channel(), 1.0.to_channel(), 1.0.to_channel())
    }
}


#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Srgb;

impl MatrixColorSpace for Srgb {
    type WhitePoint = D65;

    fn red<T: Channel + Float>() -> Yxy<T, D50> {
        Yxy::new(0.6400.to_channel(), 0.3300.to_channel(), 0.212656.to_channel())
    }
    fn green<T: Channel + Float>() -> Yxy<T, D50> {
        Yxy::new(0.3000.to_channel(), 0.6000.to_channel(), 0.715158.to_channel())
    }
    fn blue<T: Channel + Float>() -> Yxy<T, D50> {
        Yxy::new(0.1500.to_channel(), 0.0600.to_channel(), 0.072186.to_channel())
    }
    fn to_xyz_matrix<T: Channel + Float>() -> Mat3<T>{
        Mat3([
            0.4124564.to_channel(),  0.3575761.to_channel(),  0.1804375.to_channel(),
            0.2126729.to_channel(),  0.7151522.to_channel(),  0.0721750.to_channel(),
            0.0193339.to_channel(),  0.1191920.to_channel(),  0.9503041.to_channel(),
        ])
    }
    fn to_rgb_matrix<T: Channel + Float>() -> Mat3<T>{
        Mat3([
            3.2404542.to_channel(), (-1.5371385).to_channel(), (-0.4985314).to_channel(),
            (-0.9692660).to_channel(),  1.8760108.to_channel(),  0.0415560.to_channel(),
            0.0556434.to_channel(), (-0.2040259).to_channel(),  1.0572252.to_channel(),
        ])
    }
}

impl TransferFunction for Srgb{
    fn from_linear<T: Float>(x: T) -> T {
        if x > cast(0.0031308).unwrap() {
            cast::<f32, T>(1.055).unwrap() * x.powf(cast(1. / 2.4).unwrap()) - cast(0.055).unwrap()
        }else{
            cast::<f32, T>(12.95).unwrap() * x
        }
    }

    fn to_linear<T: Float>(x: T) -> T {
        if x > cast(0.04045).unwrap() {
            ((x + cast(0.055).unwrap()) / cast(1.055).unwrap()).powf(cast(2.4).unwrap())
        }else{
            x / cast(12.92).unwrap()
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct LinearRgb;

impl TransferFunction for LinearRgb {
    fn to_linear<T: Float>(x: T) -> T {
        x
    }

    fn from_linear<T: Float>(x: T) -> T {
        x
    }
}

pub struct Mat3<T>(pub [T;9]);
pub struct Vec3<T>(pub [T;3]);

impl<T: Channel + Float + Mul> Mul<Vec3<T>> for Mat3<T>{
    type Output = Vec3<T>;
    fn mul(self, xyz: Vec3<T>) -> Vec3<T> {
        let [m0, m1, m2, m3, m4, m5, m6, m7, m8] = self.0;
        let [x, y, z] = xyz.0;
        let x1 = m0 * x;
        let y1 = m3 * x;
        let z1 = m6 * x;
        let x2 = m1 * y;
        let y2 = m4 * y;
        let z2 = m7 * y;
        let x3 = m2 * z;
        let y3 = m5 * z;
        let z3 = m8 * z;

        Vec3([
            x1 + x2 + x3,
            y1 + y2 + y3,
            z1 + z2 + z3,
        ])
    }
}

impl<T: Channel + Float, W> From<Xyz<T,W>> for Vec3<T> {
    fn from(xyz: Xyz<T,W>) -> Self {
        Vec3([xyz.x, xyz.y, xyz.z])
    }
}

impl<T: Channel, S> From<Rgb<T, S>> for Vec3<T> {
    fn from(rgb: Rgb<T, S>) -> Self {
        Vec3([rgb.r, rgb.g, rgb.b])
    }
}

impl<T> Index<usize> for Vec3<T>{
    type Output = T;
    fn index(&self, index: usize) -> &T{
        self.0.index(index)
    }
}