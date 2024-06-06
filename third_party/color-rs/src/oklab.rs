/*
Copyright (c) 2021 Björn Ottosson

Permission is hereby granted, free of charge, to any person obtaining a copy of
this software and associated documentation files (the "Software"), to deal in
the Software without restriction, including without limitation the rights to
use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies
of the Software, and to permit persons to whom the Software is furnished to do
so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
*/

use std::ops::{Add, Mul};

use angle::{Angle, Rad};
use num_traits::{Float, NumCast, cast, zero, clamp};

use crate::{Channel, Rgb, ToRgb, color_space::{Srgb, TransferFunction}};


#[derive(Clone, Copy, Debug)]
pub struct OkLab<T> {
    pub l: T,
    pub a: T,
    pub b: T,
}

impl<T> OkLab<T>{
    pub fn new(l: T, a: T, b: T) -> OkLab<T>{
        OkLab { l, a, b }
    }
}


impl<T: Copy> OkLab<T>{
    pub fn luma(&self) -> T {
        self.l
    }
}

impl<T: Float> OkLab<T>{
    pub fn chromacity(&self) -> T {
        (self.a * self.a + self.b * self.b).sqrt()
    }

    pub fn hue(&self) -> Rad<T> {
        let h = self.b.atan2(self.a);
        if h < zero() {
            Rad(h + cast(std::f64::consts::TAU).unwrap())
        }else{
            Rad(h)
        }
    }

    pub fn offset_chromacity(&self, chroma_offset: T) -> OkLab<T>{
        let current_croma = self.chromacity();
        let offset_a = self.a / current_croma * chroma_offset;
        let offset_b = self.b / current_croma * chroma_offset;
        OkLab::new(
            self.l,
            self.a + offset_a,
            self.b + offset_b,
        )
    }

    pub fn from_hcl(hue: Rad<T>, chroma: T, luma: T) -> OkLab<T> {
        let a = chroma * hue.cos();
        let b = chroma * hue.sin();
        OkLab {
            l: luma,
            a,
            b,
        }
    }
}

pub trait ToOkLab {
    fn to_oklab<T: Channel>(&self) -> OkLab<T>;
}

impl<T: Channel + NumCast + Float> ToRgb for OkLab<T> {
    type Standard = Srgb;

    fn to_rgb<U:Channel>(&self) -> crate::Rgb<U, Self::Standard> {
        let l_ = self.l + cast::<_,T>(0.3963377774).unwrap() * self.a + cast::<_,T>(0.2158037573).unwrap() * self.b;
        let m_ = self.l - cast::<_,T>(0.1055613458).unwrap() * self.a - cast::<_,T>(0.0638541728).unwrap() * self.b;
        let s_ = self.l - cast::<_,T>(0.0894841775).unwrap() * self.a - cast::<_,T>(1.2914855480).unwrap() * self.b;

        let l = l_*l_*l_;
        let m = m_*m_*m_;
        let s = s_*s_*s_;

        Rgb::new(
            Srgb::from_linear(cast::<_,T>(4.0767416621).unwrap() * l - cast::<_,T>(3.3077115913).unwrap() * m + cast::<_,T>(0.2309699292).unwrap() * s).to_channel(),
            Srgb::from_linear(cast::<_,T>(-1.2684380046).unwrap() * l + cast::<_,T>(2.6097574011).unwrap() * m - cast::<_,T>(0.3413193965).unwrap() * s).to_channel(),
            Srgb::from_linear(cast::<_,T>(-0.0041960863).unwrap() * l - cast::<_,T>(0.7034186147).unwrap() * m + cast::<_,T>(1.7076147010).unwrap() * s).to_channel(),
        )
    }
}

impl<T: Channel + Float + NumCast> Add for OkLab<T>{
    type Output = OkLab<T>;
    fn add(self, other: OkLab<T>) -> OkLab<T> {
        OkLab::new(self.l + other.l, self.a + other.a, self.b + other.b)
    }
}

impl<T: Channel + Float + NumCast> Mul<T> for OkLab<T>{
    type Output = OkLab<T>;
    fn mul(self, other: T) -> OkLab<T> {
        OkLab::new(self.l * other, self.a * other, self.b * other)
    }
}

#[derive(Clone, Copy)]
struct LC { l: f32, c: f32 }

fn compute_max_saturation(a: f32, b: f32) -> f32 {
    // Max saturation will be when one of r, g or b goes below zero.

    // Select different coefficients depending on which component goes below zero first
    let (k0, k1, k2, k3, k4, wl, wm, ws);

    if -1.88170328 * a - 0.80936493 * b > 1. {
        // Red component
        k0 = 1.19086277; k1 = 1.76576728; k2 = 0.59662641; k3 = 0.75515197; k4 = 0.56771245;
        wl = 4.0767416621; wm = -3.3077115913; ws = 0.2309699292;
    }else if 1.81444104 * a - 1.19445276 * b > 1. {
        // Green component
        k0 = 0.73956515; k1 = -0.45954404; k2 = 0.08285427; k3 = 0.12541070; k4 = 0.14503204;
        wl = -1.2684380046; wm = 2.6097574011; ws = -0.3413193965;
    }else{
        // Blue component
        k0 = 1.35733652; k1 = -0.00915799; k2 = -1.15130210; k3 = -0.50559606; k4 = 0.00692167;
        wl = -0.0041960863; wm = -0.7034186147; ws = 1.7076147010;
    }

    // Approximate max saturation using a polynomial:
    let mut saturation = k0 + k1 * a + k2 * b + k3 * a * a + k4 * a * b;

    // Do one step Halley's method to get closer
    // this gives an error less than 10e6, except for some blue hues where the dS/dh is close to infinite
    // this should be sufficient for most applications, otherwise do two/three steps

    let k_l =  0.3963377774 * a + 0.2158037573 * b;
    let k_m = -0.1055613458 * a - 0.0638541728 * b;
    let k_s = -0.0894841775 * a - 1.2914855480 * b;

    {
        let l_ = 1. + saturation * k_l;
        let m_ = 1. + saturation * k_m;
        let s_ = 1. + saturation * k_s;

        let l = l_ * l_ * l_;
        let m = m_ * m_ * m_;
        let s = s_ * s_ * s_;

        let l_ds = 3. * k_l * l_ * l_;
        let m_ds = 3. * k_m * m_ * m_;
        let s_ds = 3. * k_s * s_ * s_;

        let l_ds2 = 6. * k_l * k_l * l_;
        let m_ds2 = 6. * k_m * k_m * m_;
        let s_ds2 = 6. * k_s * k_s * s_;

        let f  = wl * l     + wm * m     + ws * s;
        let f1 = wl * l_ds  + wm * m_ds  + ws * s_ds;
        let f2 = wl * l_ds2 + wm * m_ds2 + ws * s_ds2;

        saturation = saturation - f * f1 / (f1*f1 - 0.5 * f * f2);
    }

    saturation
}

fn find_cusp(a: f32, b: f32) -> LC {
	// First, find the maximum saturation (saturation S = C/L)
	let s_cusp = compute_max_saturation(a, b);

	// Convert to linear sRGB to find the first point where at least one of r,g or b >= 1:
	let rgb_at_max = OkLab{ l: 1., a: s_cusp * a, b: s_cusp * b }.to_rgb::<f32>();
	let l_cusp = (1. / rgb_at_max.r.max(rgb_at_max.g).max(rgb_at_max.b)).cbrt();
	let c_cusp = l_cusp * s_cusp;

	LC { l: l_cusp , c: c_cusp }
}


fn find_gamut_intersection(a: f32, b: f32, l1: f32, h1: f32, l0: f32) -> f32 {
	// Find the cusp of the gamut triangle
	let cusp = find_cusp(a, b);
    find_gamut_intersection_cusp(a, b, l1, h1, l0, cusp)
}

fn find_gamut_intersection_cusp(a: f32, b: f32, l1: f32, h1: f32, l0: f32, cusp: LC) -> f32 {
	// Find the intersection for upper and lower half seprately
	let mut t;
	if ((l1 - l0) * cusp.c - (cusp.l - l0) * h1) <= 0.
	{
		// Lower half

		t = cusp.c * l0 / (h1 * cusp.l + cusp.c * (l0 - l1));
	}
	else
	{
		// Upper half

		// First intersect with triangle
		t = cusp.c * (l0 - 1.) / (h1 * (cusp.l - 1.) + cusp.c * (l0 - l1));

		// Then one step Halley's method
		{
			let dl = l1 - l0;
			let dc = h1;

			let k_l =  0.3963377774 * a + 0.2158037573 * b;
			let k_m = -0.1055613458 * a - 0.0638541728 * b;
			let k_s = -0.0894841775 * a - 1.2914855480 * b;

			let l_dt = dl + dc * k_l;
			let m_dt = dl + dc * k_m;
			let s_dt = dl + dc * k_s;


			// If higher accuracy is required, 2 or 3 iterations of the following block can be used:
			{
				let luma = l0 * (1. - t) + t * l1;
				let chroma = t * h1;

				let l_ = luma + chroma * k_l;
				let m_ = luma + chroma * k_m;
				let s_ = luma + chroma * k_s;

				let l = l_ * l_ * l_;
				let m = m_ * m_ * m_;
				let s = s_ * s_ * s_;

				let ldt = 3. * l_dt * l_ * l_;
				let mdt = 3. * m_dt * m_ * m_;
				let sdt = 3. * s_dt * s_ * s_;

				let ldt2 = 6. * l_dt * l_dt * l_;
				let mdt2 = 6. * m_dt * m_dt * m_;
				let sdt2 = 6. * s_dt * s_dt * s_;

				let r = 4.0767416621 * l - 3.3077115913 * m + 0.2309699292 * s - 1.;
				let r1 = 4.0767416621 * ldt - 3.3077115913 * mdt + 0.2309699292 * sdt;
				let r2 = 4.0767416621 * ldt2 - 3.3077115913 * mdt2 + 0.2309699292 * sdt2;

				let u_r = r1 / (r1 * r1 - 0.5 * r * r2);
				let t_r = -r * u_r;

				let g = -1.2684380046 * l + 2.6097574011 * m - 0.3413193965 * s - 1.;
				let g1 = -1.2684380046 * ldt + 2.6097574011 * mdt - 0.3413193965 * sdt;
				let g2 = -1.2684380046 * ldt2 + 2.6097574011 * mdt2 - 0.3413193965 * sdt2;

				let u_g = g1 / (g1 * g1 - 0.5 * g * g2);
				let t_g = -g * u_g;

				let b = -0.0041960863 * l - 0.7034186147 * m + 1.7076147010 * s - 1.;
				let b1 = -0.0041960863 * ldt - 0.7034186147 * mdt + 1.7076147010 * sdt;
				let b2 = -0.0041960863 * ldt2 - 0.7034186147 * mdt2 + 1.7076147010 * sdt2;

				let u_b = b1 / (b1 * b1 - 0.5 * b * b2);
				let t_b = -b * u_b;

				let t_r = if u_r >= 0. { t_r } else { f32::MAX };
				let t_g = if u_g >= 0. { t_g } else { f32::MAX };
				let t_b = if u_b >= 0. { t_b } else { f32::MAX };

				t += t_r.min(t_g.min(t_b));
			}
		}
	}

	t
}

impl Rgb<f32> {
    pub fn gamut_clip_preserve_chroma(&self) -> Rgb<f32> {
        if self.r < 1. && self.g < 1. && self.b < 1. && self.r > 0. && self.g > 0. && self.b > 0. {
            return *self;
        }

        let lab: OkLab<f32> = self.to_oklab();

        let l = lab.l;
        let eps = 0.00001;
        let c = eps.max((lab.a * lab.a + lab.b * lab.b).sqrt());
        let a = lab.a / c;
        let b = lab.b / c;

        let l0 = clamp(l, 0., 1.);

        let t = find_gamut_intersection(a, b, l, c, l0);
        let l_clipped = l0 * (1. - t) + t * l;
        let c_clipped = t * c;

        OkLab{ l: l_clipped, a: c_clipped * a, b: c_clipped * b }.to_rgb()
    }

    pub fn gamut_clip_project_to_0_5(&self) -> Rgb<f32> {
        if self.r < 1. && self.g < 1. && self.b < 1. && self.r > 0. && self.g > 0. && self.b > 0. {
            return *self;
        }

        let lab = self.to_oklab::<f32>();

        let l = lab.l;
        let eps = 0.00001;
        let c = eps.max(lab.chromacity());
        let a_ = lab.a / c;
        let b_ = lab.b / c;

        let l0 = 0.5;

        let t = find_gamut_intersection(a_, b_, l, c, l0);
        let l_clipped = l0 * (1. - t) + t * l;
        let c_clipped = t * c;

        OkLab{ l: l_clipped, a: c_clipped * a_, b: c_clipped * b_ }.to_rgb()
    }

    pub fn gamut_clip_project_to_l_cusp(&self) -> Rgb<f32> {
        if self.r < 1. && self.g < 1. && self.b < 1. && self.r > 0. && self.g > 0. && self.b > 0. {
            return *self;
        }

        let lab = self.to_oklab::<f32>();

        let l = lab.l;
        let eps = 0.00001;
        let c = eps.max(lab.chromacity());
        let a = lab.a / c;
        let b = lab.b / c;

        // The cusp is computed here and in find_gamut_intersection, an optimized solution would only compute it once.
        let cusp = find_cusp(a, b);

        let l0 = cusp.l;

        let t = find_gamut_intersection(a, b, l, c, l0);

        let l_clipped = l0 * (1. - t) + t * l;
        let c_clipped = t * c;

        OkLab{ l: l_clipped, a: c_clipped * a, b: c_clipped * b }.to_rgb()
    }

    pub fn gamut_clip_adaptive_l0_0_5(&self) -> Rgb<f32> {
        self.gamut_clip_adaptive_l0_0_5_alpha(0.05)
    }

    pub fn gamut_clip_adaptive_l0_0_5_alpha(&self, alpha: f32) -> Rgb<f32> {
        if self.r < 1. && self.g < 1. && self.b < 1. && self.r > 0. && self.g > 0. && self.b > 0. {
            return *self;
        }

        let lab = self.to_oklab::<f32>();

        let l = lab.l;
        let eps = 0.00001;
        let c = eps.max(lab.chromacity());
        let a = lab.a / c;
        let b = lab.b / c;

        let ld = l - 0.5;
        let e1 = 0.5 + ld.abs() + alpha * c;
        let l0 = 0.5*(1. + ld.signum()*(e1 - (e1*e1 - 2. * ld.abs()).sqrt()));

        let t = find_gamut_intersection(a, b, l, c, l0);
        let l_clipped = l0 * (1. - t) + t * l;
        let c_clipped = t * c;

        OkLab{ l: l_clipped, a: c_clipped * a, b: c_clipped * b }.to_rgb()
    }

    pub fn gamut_clip_adaptive_l0_l_cusp(&self) -> Rgb<f32> {
        self.gamut_clip_adaptive_l0_l_cusp_alpha(0.05)
    }

    pub fn gamut_clip_adaptive_l0_l_cusp_alpha(&self, alpha:f32) -> Rgb<f32> {
        if self.r < 1. && self.g < 1. && self.b < 1. && self.r > 0. && self.g > 0. && self.b > 0. {
            return *self;
        }

        let lab = self.to_oklab::<f32>();

        let l = lab.l;
        let eps = 0.00001;
        let c = eps.max(lab.chromacity());
        let a = lab.a / c;
        let b = lab.b / c;

        // The cusp is computed here and in find_gamut_intersection, an optimized solution would only compute it once.
        let cusp = find_cusp(a, b);

        let ld = l - cusp.l;
        let k = 2. * if ld > 0. { 1. - cusp.l } else { cusp.l };

        let e1 = 0.5*k + ld.abs() + alpha * c/k;
        let l0 = cusp.l + 0.5 * (ld.signum() * (e1 - (e1 * e1 - 2. * k * ld.abs()).sqrt()));

        let t = find_gamut_intersection(a, b, l, c, l0);
        let l_clipped = l0 * (1. - t) + t * l;
        let c_clipped = t * c;

        OkLab{ l: l_clipped, a: c_clipped * a, b: c_clipped * b }.to_rgb()
    }
}

struct ST { s: f32, t: f32 }

impl LC {
    fn to_st(&self) -> ST {
        let l = self.l;
        let c = self.c;
        ST { s: c / l, t: c / (1. - l) }
    }
}

pub struct OkHsv {
    pub h: angle::Deg<f32>,
    pub s: f32,
    pub v: f32,
}

fn toe_inv(x: f32) -> f32
{
	const K1: f32 = 0.206;
	const K2: f32 = 0.03;
	const K3: f32 = (1. + K1) / (1. + K2);
	(x * x + K1 * x) / (K3 * (x + K2))
}

impl ToRgb for OkHsv {
    type Standard = Srgb;

    fn to_rgb<U:Channel>(&self) -> crate::Rgb<U, Self::Standard> {
        let h = self.h;
        let s = self.s;
        let v = self.v;

        let a = h.cos();
        let b = h.sin();

        let cusp = find_cusp(a, b);
        let st_max = cusp.to_st();
        let s_max = st_max.s;
        let t_max = st_max.t;
        let s_0 = 0.5;
        let k = 1. - s_0 / s_max;

        // first we compute L and V as if the gamut is a perfect triangle:

        // L, C when v==1:
        let l_v = 1.     - s * s_0 / (s_0 + t_max - t_max * k * s);
        let c_v = s * t_max * s_0 / (s_0 + t_max - t_max * k * s);

        let l = v * l_v;
        let c = v * c_v;

        // then we compensate for both toe and the curved top part of the triangle:
        let l_vt = toe_inv(l_v);
        let c_vt = c_v * l_vt / l_v;

        let l_new = toe_inv(l);
        let c = c * l_new / l;
        let l = l_new;

        let rgb_scale = OkLab{ l: l_vt, a: a * c_vt, b: b * c_vt }.to_rgb::<f32>();
        let scale_l = (1. / rgb_scale.r.max(rgb_scale.g).max(rgb_scale.b.max(0.))).cbrt();

        let l = l * scale_l;
        let c = c * scale_l;

        OkLab{ l, a: c * a, b: c * b }.to_rgb()
    }
}

pub struct OkHsl {
    pub h: angle::Deg<f32>,
    pub s: f32,
    pub l: f32,
}

// Returns a smooth approximation of the location of the cusp
// This polynomial was created by an optimization process
// It has been designed so that S_mid < S_max and T_mid < T_max
fn get_st_mid(a_: f32, b_: f32) -> ST
{
	let s = 0.11516993 + 1. / (
		7.44778970 + 4.15901240 * b_
		+ a_ * (-2.19557347 + 1.75198401 * b_
			+ a_ * (-2.13704948 - 10.02301043 * b_
				+ a_ * (-4.24894561 + 5.38770819 * b_ + 4.69891013 * a_
					)))
		);

    let t = 0.11239642 + 1. / (
		1.61320320 - 0.68124379 * b_
		+ a_ * (0.40370612 + 0.90148123 * b_
			+ a_ * (-0.27087943 + 0.61223990 * b_
				+ a_ * (0.00299215 - 0.45399568 * b_ - 0.14661872 * a_
					)))
		);

	ST { s, t }
}

struct Cs { c_0: f32, c_mid: f32, c_max: f32 }
impl OkLab<f32> {
    fn get_cs(self) -> Cs {
        let cusp = find_cusp(self.a, self.b);

        let c_max = find_gamut_intersection_cusp(self.a, self.b, self.l, 1., self.l, cusp);
        let st_max = cusp.to_st();

        // Scale factor to compensate for the curved part of gamut shape:
        let k = c_max / ((self.l * st_max.s).min(1. - self.l) * st_max.t);

        let c_mid = {
            let st_mid = get_st_mid(self.a, self.b);

            // Use a soft minimum function, instead of a sharp triangle shape to get a smooth value for chroma.
            let c_a = self.l * st_mid.s;
            let c_b = (1. - self.l) * st_mid.t;
            0.9 * k * (1. / (1. / (c_a * c_a * c_a * c_a) + 1. / (c_b * c_b * c_b * c_b))).sqrt().sqrt()
        };

        let c_0 = {
            // for C_0, the shape is independent of hue, so ST are constant. Values picked to roughly be the average values of ST.
            let c_a = self.l * 0.4;
            let c_b = (1. - self.l) * 0.8;

            // Use a soft minimum function, instead of a sharp triangle shape to get a smooth value for chroma.
            (1. / (1. / (c_a * c_a) + 1. / (c_b * c_b))).sqrt()
        };

        Cs { c_0, c_mid, c_max }
    }
}

impl ToRgb for OkHsl {
    type Standard = Srgb;

    fn to_rgb<U:Channel>(&self) -> crate::Rgb<U, Self::Standard> {
        let h = self.h;
        let s = self.s;
        let l = self.l;

        if l == 1.0 {
            OkLab { l: 1., a: 1., b: 1. };
        }else if l == 0. {
            OkLab { l: 0., a: 0., b: 0. };
        }

        let a = h.cos();
        let b = h.sin();
        let l = toe_inv(l);

        let cs = OkLab{l, a, b}.get_cs();
        let c_0 = cs.c_0;
        let c_mid = cs.c_mid;
        let c_max = cs.c_max;

        // Interpolate the three values for C so that:
        // At s=0: dC/ds = C_0, C=0
        // At s=0.8: C=C_mid
        // At s=1.0: C=C_max

        let mid = 0.8;
        let mid_inv = 1.25;

        let chroma = if s < mid {
            let t = mid_inv * s;

            let k_1 = mid * c_0;
            let k_2 = 1. - k_1 / c_mid;

            t * k_1 / (1. - k_2 * t)
        }else{
            let t = (s - mid)/ (1. - mid);

            let k_0 = c_mid;
            let k_1 = (1. - mid) * c_mid * c_mid * mid_inv * mid_inv / c_0;
            let k_2 = 1. - (k_1) / (c_max - c_mid);

            k_0 + t * k_1 / (1. - k_2 * t)
        };

        OkLab{ l, a: chroma * a, b: chroma * b }.to_rgb()
    }
}



#[test]
fn test_range_norm() {
    use ToRgb;

    for r in 0u8..=255 {
        for g in 0u8..=255 {
            for b in 0u8..=255 {
                let rgb: crate::Rgb<f32> = crate::rgb!(r, g, b).to_rgb();
                let oklab: OkLab<f32> = rgb.to_oklab();
                assert!(oklab.l >= -0.000001);
                assert!(oklab.l <=  1.000001);
                assert!(oklab.chromacity() >= -0.000001);
                assert!(oklab.chromacity() <=  1.000001);
            }
        }
    }
}

#[test]
fn test_symmetric_u8() {
    for r in 0u8..=255 {
        for g in 0u8..=255 {
            for b in 0u8..=255 {
                let rgb: Rgb<f32> = rgb!(r, g, b).to_rgb();
                let oklab: OkLab<f32> = rgb.to_oklab();
                let rgb_back: Rgb<f32> = oklab.to_rgb();
                assert!((rgb.r - rgb_back.r).abs() <= 0.00015, "rgb.r {} rgb_back.r {} diff {}", rgb.r, rgb_back.r, (rgb.r - rgb_back.r));
                assert!((rgb.g - rgb_back.g).abs() <= 0.00015, "rgb.g {} rgb_back.g {} diff {}", rgb.g, rgb_back.g, (rgb.g - rgb_back.g));
                assert!((rgb.b - rgb_back.b).abs() <= 0.00015, "rgb.b {} rgb_back.b {} diff {}", rgb.b, rgb_back.b, (rgb.b - rgb_back.b));
            }
        }
    }
}

#[test]
fn test_symmetric_hcl_u8() {
    for r in 0u8..=255 {
        for g in 0u8..=255 {
            for b in 0u8..=255 {
                let rgb: Rgb<f32> = rgb!(r, g, b).to_rgb();
                let oklab: OkLab<f32> = rgb.to_oklab();
                let hue = oklab.hue();
                let luma = oklab.luma();
                let chroma = oklab.chromacity();
                let oklabback = OkLab::from_hcl(hue, chroma, luma);
                let rgb_back: Rgb<f32> = oklabback.to_rgb();
                assert!((rgb.r - rgb_back.r).abs() <= 0.00015, "rgb.r {} rgb_back.r {} diff {}", rgb.r, rgb_back.r, (rgb.r - rgb_back.r));
                assert!((rgb.g - rgb_back.g).abs() <= 0.00015, "rgb.g {} rgb_back.g {} diff {}", rgb.g, rgb_back.g, (rgb.g - rgb_back.g));
                assert!((rgb.b - rgb_back.b).abs() <= 0.00015, "rgb.b {} rgb_back.b {} diff {}", rgb.b, rgb_back.b, (rgb.b - rgb_back.b));
            }
        }
    }
}

#[test]
fn test_ranges() {
    let rgb: Rgb<f32> = crate::rgb_linear!(0.293055, 0.979167, 0.577595).to_standard();
    let lab = rgb.to_oklab();
    let hue = lab.hue();
    let luma = lab.luma();
    let chroma = lab.chromacity();
    let hue_offset = 76.279404;
    let hue = (hue + angle::Deg(hue_offset).to_rad()).wrap();
    let labback = OkLab::from_hcl(hue, chroma, luma);
    let rgb_back: Rgb<f32> = labback.to_rgb();
    assert!(rgb_back.r >= -0.000001, "rgb.r {}", rgb_back.r);
    assert!(rgb_back.g <=  1.000001, "rgb.r {}", rgb_back.r);
    assert!(rgb_back.g >= -0.000001, "rgb.g {}", rgb_back.g);
    assert!(rgb_back.g <=  1.000001, "rgb.g {}", rgb_back.g);
    assert!(rgb_back.b >= -0.000001, "rgb.b {}", rgb_back.b);
    assert!(rgb_back.b <=  1.000001, "rgb.b {}", rgb_back.b);
}