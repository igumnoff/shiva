// Copyright 2013 The color-rs developers. For a full listing of the authors,
// refer to the AUTHORS file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use num_traits::{Saturating, NumCast, Num};
use std::ops::{Mul, Div, Add, Sub};
use std::marker::PhantomData;

use {Color, FloatColor};
use {Channel, FloatChannel};
use {Hsv, ToHsv};
use {ToRgb, Rgb};
use alpha::{ToRgba, Rgba};
use color_space::{Srgb, TransferFunction};


#[derive(Serialize, Deserialize, Debug)]
pub struct Luma<T, S = Srgb> { pub l: T, pub standard: PhantomData<S> }

impl<T: Clone, S> Clone for Luma<T, S>{
    fn clone(&self) -> Luma<T, S>{
        Luma{ l: self.l.clone(), standard: PhantomData }
    }
}

impl<T: Copy, S> Copy for Luma<T, S>{}

impl<N: PartialEq, S> PartialEq for Luma<N, S>{
	#[inline]
	fn eq(&self, other: &Luma<N, S>) -> bool{
		self.l.eq(&other.l)
	}
}

impl<N: Clone + PartialEq + Eq + Num + NumCast, S> Eq for Luma<N, S>{}

impl<T, S> Luma<T, S> {
    pub const fn new(l: T) -> Luma<T, S> {
        Luma { l, standard: PhantomData }
    }
}

impl<T:Channel, S> Luma<T, S> {
    pub fn from_hex(hex: u8) -> Luma<T> {
        Luma::<u8>::new(hex).to_luma()
    }
}

impl<T:Channel, S> Color<T> for Luma<T, S> {
    /// Clamps the components of the color to the range `(lo,hi)`.
    #[inline]
    fn clamp_s(self, lo: T, hi: T) -> Luma<T,S> {
        Luma::new(self.l.clamp(lo, hi))
    }

    /// Clamps the components of the color component-wise between `lo` and `hi`.
    #[inline]
    fn clamp_c(self, lo: Luma<T,S>, hi: Luma<T,S>) -> Luma<T,S> {
        Luma::new(self.l.clamp(lo.l, hi.l))
    }

    /// Inverts the color.
    #[inline]
    fn inverse(self) -> Luma<T,S> {
        Luma::new(self.l.invert_channel())
    }

    #[inline]
    fn mix(self, other: Self, value: T) -> Self {
        Luma::new(self.l.mix(other.l, value))
    }
}

impl<T:FloatChannel, S> FloatColor<T> for Luma<T, S> {
    /// Clamps the components of the color to the range `(0,1)`.
    #[inline]
    fn saturate(self) -> Luma<T, S> {
        Luma::new(self.l.saturate())
    }
}

pub trait ToLuma {
    type Standard: TransferFunction;
    fn to_luma<U:Channel>(&self) -> Luma<U, Self::Standard>;
}

impl ToLuma for u8 {
    type Standard = Srgb;
    fn to_luma<U: Channel>(&self) -> Luma<U, Srgb> {
        Luma::new(Channel::from(*self))
    }
}

impl<T: Channel, S: TransferFunction> ToLuma for Luma<T, S> {
    type Standard = S;
    fn to_luma<U: Channel>(&self) -> Luma<U, S> {
        Luma::new(Channel::from(self.l))
    }
}

impl<T:Clone + Channel, S: TransferFunction> ToRgb for Luma<T, S> {
    type Standard = S;
    #[inline]
    fn to_rgb<U:Channel>(&self) -> Rgb<U, S> {
        let r = self.l.to_channel();
        Rgb::new(r, r, r)
    }
}

impl<T:Clone + Channel, S: TransferFunction> ToRgba for Luma<T, S> {
    type Standard = S;
    #[inline]
    fn to_rgba<U:Channel>(&self) -> Rgba<U, S> {
        let r = self.l.to_channel();
        Rgba::new(Rgb::new(r, r, r), 1f32.to_channel())
    }
}

impl<T:Channel + NumCast + Num, S: TransferFunction> ToHsv for Luma<T,S> {
    type Standard = S;
    #[inline]
    fn to_hsv<U:Channel + NumCast + Num>(&self) -> Hsv<U, S> {
        self.to_rgb::<U>().to_hsv()
    }
}

impl<T:Channel, S> Mul for Luma<T, S> {
    type Output = Luma<T, S>;

    #[inline]
    fn mul(self, rhs: Luma<T, S>) -> Luma<T, S> {
        Luma::new(self.l.normalized_mul(rhs.l))
    }
}

impl<T:Channel + Mul<T,Output=T>, S> Mul<T> for Luma<T, S> {
    type Output = Luma<T, S>;

    #[inline]
    fn mul(self, rhs: T) -> Luma<T, S> {
        Luma::new(self.l * rhs)
    }
}


impl<T:Channel, S> Div for Luma<T, S> {
    type Output = Luma<T, S>;

    #[inline]
    fn div(self, rhs: Luma<T, S>) -> Luma<T, S> {
        Luma::new(self.l.normalized_div(rhs.l))
    }
}

impl<T:Channel + Div<T,Output=T>, S> Div<T> for Luma<T, S> {
    type Output = Luma<T, S>;

    #[inline]
    fn div(self, rhs: T) -> Luma<T, S> {
        Luma::new(self.l / rhs)
    }
}

impl<T:Channel + Add<T,Output=T>, S> Add for Luma<T, S> {
    type Output = Luma<T, S>;

    #[inline]
    fn add(self, rhs: Luma<T, S>) -> Luma<T, S> {
        Luma::new(self.l + rhs.l)
    }
}

impl<T:Channel + Sub<T,Output=T>, S> Sub for Luma<T, S> {
    type Output = Luma<T, S>;

    #[inline]
    fn sub(self, rhs: Luma<T, S>) -> Luma<T, S> {
        Luma::new(self.l - rhs.l)
    }
}

impl<T:Channel + Saturating, S> Saturating for Luma<T, S> {
    fn saturating_add(self, v: Luma<T, S>) -> Luma<T, S> {
        Luma::new(self.l.saturating_add(v.l))
    }

    fn saturating_sub(self, v: Luma<T, S>) -> Luma<T, S> {
        Luma::new(self.l.saturating_sub(v.l))
    }
}

#[cfg(feature = "bytemuck")]
unsafe impl<T, S> bytemuck::Pod for Luma<T, S>
where T: Copy + 'static, S: TransferFunction {}

#[cfg(feature = "bytemuck")]
unsafe impl<T, S> bytemuck::Zeroable for Luma<T, S>
where S: TransferFunction {}