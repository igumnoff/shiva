use crate::{Lab, Rgb, Rgba, Luma, ToLuma, Channel, color_space::{Srgb, WhitePoint}};

use rand::Rng;

use kmeans_colors::{Hamerly, HamerlyCentroids, HamerlyPoint};

pub use kmeans_colors::{get_kmeans, Calculate, Kmeans, CentroidData, Sort};

impl<Wp: WhitePoint> Calculate for Lab<f32, Wp> {
    fn get_closest_centroid(lab: &[Lab<f32, Wp>], centroids: &[Lab<f32, Wp>], indices: &mut Vec<u8>) {
        for color in lab.iter() {
            let mut index = 0;
            let mut diff;
            let mut min = core::f32::MAX;
            for (idx, cent) in centroids.iter().enumerate() {
                diff = Self::difference(color, cent);
                if diff < min {
                    min = diff;
                    index = idx;
                }
            }
            indices.push(index as u8);
        }
    }

    fn recalculate_centroids(
        mut rng: &mut impl Rng,
        buf: &[Lab<f32, Wp>],
        centroids: &mut [Lab<f32, Wp>],
        indices: &[u8],
    ) {
        for (idx, cent) in centroids.iter_mut().enumerate() {
            let mut l = 0.0;
            let mut a = 0.0;
            let mut b = 0.0;
            let mut counter: u64 = 0;
            for (jdx, color) in indices.iter().zip(buf) {
                if *jdx == idx as u8 {
                    l += color.l;
                    a += color.a;
                    b += color.b;
                    counter += 1;
                }
            }
            if counter != 0 {
                *cent = Lab::new(
                    l / (counter as f32),
                    a / (counter as f32),
                    b / (counter as f32),
                );
            } else {
                *cent = Self::create_random(&mut rng);
            }
        }
    }

    fn check_loop(centroids: &[Lab<f32, Wp>], old_centroids: &[Lab<f32, Wp>]) -> f32 {
        let mut l = 0.0;
        let mut a = 0.0;
        let mut b = 0.0;
        for c in centroids.iter().zip(old_centroids) {
            l += (c.0).l - (c.1).l;
            a += (c.0).a - (c.1).a;
            b += (c.0).b - (c.1).b;
        }

        l * l + a * a + b * b
    }

    #[inline]
    fn create_random(rng: &mut impl Rng) -> Lab<f32, Wp> {
        Lab::new(
            rng.gen_range(0.0, 100.0),
            rng.gen_range(-128.0, 127.0),
            rng.gen_range(-128.0, 127.0),
        )
    }

    #[inline]
    fn difference(c1: &Lab<f32, Wp>, c2: &Lab<f32, Wp>) -> f32 {
        (c1.l - c2.l) * (c1.l - c2.l)
            + (c1.a - c2.a) * (c1.a - c2.a)
            + (c1.b - c2.b) * (c1.b - c2.b)
    }
}

impl Calculate for Rgb<f32, Srgb> {
    fn get_closest_centroid(rgb: &[Rgb<f32, Srgb>], centroids: &[Rgb<f32, Srgb>], indices: &mut Vec<u8>) {
        for color in rgb.iter() {
            let mut index = 0;
            let mut diff;
            let mut min = core::f32::MAX;
            for (idx, cent) in centroids.iter().enumerate() {
                diff = Self::difference(color, cent);
                if diff < min {
                    min = diff;
                    index = idx;
                }
            }
            indices.push(index as u8);
        }
    }

    fn recalculate_centroids(
        mut rng: &mut impl Rng,
        buf: &[Rgb<f32, Srgb>],
        centroids: &mut [Rgb<f32, Srgb>],
        indices: &[u8],
    ) {
        for (idx, cent) in centroids.iter_mut().enumerate() {
            let mut red = 0.0;
            let mut green = 0.0;
            let mut blue = 0.0;
            let mut counter: u64 = 0;
            for (jdx, color) in indices.iter().zip(buf) {
                if *jdx == idx as u8 {
                    red += color.r;
                    green += color.g;
                    blue += color.b;
                    counter += 1;
                }
            }
            if counter != 0 {
                *cent = Rgb::new(
                    red / (counter as f32),
                    green / (counter as f32),
                    blue / (counter as f32)
                );
            } else {
                *cent = Self::create_random(&mut rng);
            }
        }
    }

    fn check_loop(centroids: &[Rgb<f32, Srgb>], old_centroids: &[Rgb<f32, Srgb>]) -> f32 {
        let mut red = 0.0;
        let mut green = 0.0;
        let mut blue = 0.0;
        for c in centroids.iter().zip(old_centroids) {
            red += (c.0).r - (c.1).r;
            green += (c.0).g - (c.1).g;
            blue += (c.0).b - (c.1).b;
        }

        red * red + green * green + blue * blue
    }

    #[inline]
    fn create_random(rng: &mut impl Rng) -> Rgb<f32, Srgb> {
        Rgb::new(rng.gen(), rng.gen(), rng.gen())
    }

    #[inline]
    fn difference(c1: &Rgb<f32, Srgb>, c2: &Rgb<f32, Srgb>) -> f32 {
        (c1.r - c2.r) * (c1.r - c2.r)
            + (c1.g - c2.g) * (c1.g - c2.g)
            + (c1.b - c2.b) * (c1.b - c2.b)
    }
}

impl<Wp: WhitePoint> Hamerly for Lab<f32, Wp> {
    fn compute_half_distances(centers: &mut HamerlyCentroids<Self>) {
        // Find each center's closest center
        for ((i, ci), half_dist) in centers
            .centroids
            .iter()
            .enumerate()
            .zip(centers.half_distances.iter_mut())
        {
            let mut diff;
            let mut min = f32::MAX;
            for (j, cj) in centers.centroids.iter().enumerate() {
                // Don't compare centroid to itself
                if i == j {
                    continue;
                }
                diff = Self::difference(&ci, &cj);
                if diff < min {
                    min = diff;
                }
            }
            *half_dist = min.sqrt() * 0.5;
        }
    }

    fn get_closest_centroid_hamerly(
        buffer: &[Self],
        centers: &HamerlyCentroids<Self>,
        points: &mut [HamerlyPoint],
    ) {
        for (val, point) in buffer.iter().zip(points.iter_mut()) {
            // Assign max of lower bound and half distance to z
            let z = centers
                .half_distances
                .get(point.index as usize)
                .unwrap()
                .max(point.lower_bound);

            if point.upper_bound <= z {
                continue;
            }

            // Tighten upper bound
            point.upper_bound =
                Self::difference(val, centers.centroids.get(point.index as usize).unwrap()).sqrt();

            if point.upper_bound <= z {
                continue;
            }

            // Find the two closest centers to current point and their distances
            if centers.centroids.len() < 2 {
                continue;
            }

            let mut min1 = Self::difference(val, centers.centroids.get(0).unwrap());
            let mut min2 = f32::MAX;
            let mut c1 = 0;
            for j in 1..centers.centroids.len() {
                let diff = Self::difference(val, centers.centroids.get(j).unwrap());
                if diff < min1 {
                    min2 = min1;
                    min1 = diff;
                    c1 = j;
                    continue;
                }
                if diff < min2 {
                    min2 = diff;
                }
            }

            if c1 as u8 != point.index {
                point.index = c1 as u8;
                point.upper_bound = min1.sqrt();
            }
            point.lower_bound = min2.sqrt();
        }
    }

    fn recalculate_centroids_hamerly(
        mut rng: &mut impl Rng,
        buf: &[Self],
        centers: &mut HamerlyCentroids<Self>,
        points: &[HamerlyPoint],
    ) {
        for ((idx, cent), delta) in centers
            .centroids
            .iter_mut()
            .enumerate()
            .zip(centers.deltas.iter_mut())
        {
            let mut l = 0.0;
            let mut a = 0.0;
            let mut b = 0.0;
            let mut counter: u64 = 0;
            for (point, color) in points.iter().zip(buf) {
                if point.index == idx as u8 {
                    l += color.l;
                    a += color.a;
                    b += color.b;
                    counter += 1;
                }
            }
            if counter != 0 {
                let new_color = Lab::new(
                    l / (counter as f32),
                    a / (counter as f32),
                    b / (counter as f32),
                );
                *delta = Self::difference(cent, &new_color).sqrt();
                *cent = new_color;
            } else {
                let new_color = Self::create_random(&mut rng);
                *delta = Self::difference(cent, &new_color).sqrt();
                *cent = new_color;
            }
        }
    }

    fn update_bounds(centers: &HamerlyCentroids<Self>, points: &mut [HamerlyPoint]) {
        let mut delta_p = 0.0;
        for c in centers.deltas.iter() {
            if *c > delta_p {
                delta_p = *c;
            }
        }

        for point in points.iter_mut() {
            point.upper_bound += centers.deltas.get(point.index as usize).unwrap();
            point.lower_bound -= delta_p;
        }
    }
}

impl Hamerly for Rgb<f32, Srgb> {
    fn compute_half_distances(centers: &mut HamerlyCentroids<Self>) {
        // Find each center's closest center
        for ((i, ci), half_dist) in centers
            .centroids
            .iter()
            .enumerate()
            .zip(centers.half_distances.iter_mut())
        {
            let mut diff;
            let mut min = f32::MAX;
            for (j, cj) in centers.centroids.iter().enumerate() {
                // Don't compare centroid to itself
                if i == j {
                    continue;
                }
                diff = Self::difference(&ci, &cj);
                if diff < min {
                    min = diff;
                }
            }
            *half_dist = min.sqrt() * 0.5;
        }
    }

    fn get_closest_centroid_hamerly(
        buffer: &[Self],
        centers: &HamerlyCentroids<Self>,
        points: &mut [HamerlyPoint],
    ) {
        for (val, point) in buffer.iter().zip(points.iter_mut()) {
            // Assign max of lower bound and half distance to z
            let z = centers
                .half_distances
                .get(point.index as usize)
                .unwrap()
                .max(point.lower_bound);

            if point.upper_bound <= z {
                continue;
            }

            // Tighten upper bound
            point.upper_bound =
                Self::difference(val, centers.centroids.get(point.index as usize).unwrap()).sqrt();

            if point.upper_bound <= z {
                continue;
            }

            // Find the two closest centers to current point and their distances
            if centers.centroids.len() < 2 {
                continue;
            }

            let mut min1 = Self::difference(val, centers.centroids.get(0).unwrap());
            let mut min2 = f32::MAX;
            let mut c1 = 0;
            for j in 1..centers.centroids.len() {
                let diff = Self::difference(val, centers.centroids.get(j).unwrap());
                if diff < min1 {
                    min2 = min1;
                    min1 = diff;
                    c1 = j;
                    continue;
                }
                if diff < min2 {
                    min2 = diff;
                }
            }

            if c1 as u8 != point.index {
                point.index = c1 as u8;
                point.upper_bound = min1.sqrt();
            }
            point.lower_bound = min2.sqrt();
        }
    }

    fn recalculate_centroids_hamerly(
        mut rng: &mut impl Rng,
        buf: &[Self],
        centers: &mut HamerlyCentroids<Self>,
        points: &[HamerlyPoint],
    ) {
        for ((idx, cent), delta) in centers
            .centroids
            .iter_mut()
            .enumerate()
            .zip(centers.deltas.iter_mut())
        {
            let mut red = 0.0;
            let mut green = 0.0;
            let mut blue = 0.0;
            let mut counter: u64 = 0;
            for (point, color) in points.iter().zip(buf) {
                if point.index == idx as u8 {
                    red += color.r;
                    green += color.g;
                    blue += color.b;
                    counter += 1;
                }
            }
            if counter != 0 {
                let new_color = Rgb::new(
                    red / (counter as f32),
                    green / (counter as f32),
                    blue / (counter as f32),
                );
                *delta = Self::difference(cent, &new_color).sqrt();
                *cent = new_color;
            } else {
                let new_color = Self::create_random(&mut rng);
                *delta = Self::difference(cent, &new_color).sqrt();
                *cent = new_color;
            }
        }
    }

    fn update_bounds(centers: &HamerlyCentroids<Self>, points: &mut [HamerlyPoint]) {
        let mut delta_p = 0.0;
        for c in centers.deltas.iter() {
            if *c > delta_p {
                delta_p = *c;
            }
        }

        for point in points.iter_mut() {
            point.upper_bound += centers.deltas.get(point.index as usize).unwrap();
            point.lower_bound -= delta_p;
        }
    }
}

/// A trait for mapping colors to their corresponding centroids.
pub trait MapColor: Sized {
    /// Map pixel indices to each centroid for output buffer.
    fn map_indices_to_centroids(centroids: &[Self], indices: &[u8]) -> Vec<Self>;
}

impl<Wp> MapColor for Lab<f32, Wp>
where
    Wp: WhitePoint + Copy,
{
    #[inline]
    fn map_indices_to_centroids(centroids: &[Self], indices: &[u8]) -> Vec<Self> {
        indices
            .iter()
            .map(|x| {
                *centroids
                    .get(*x as usize)
                    .unwrap_or_else(|| centroids.last().unwrap())
            })
            .collect()
    }
}

// impl<Wp> MapColor for Laba<Wp>
// where
//     Wp: WhitePoint,
// {
//     #[inline]
//     fn map_indices_to_centroids(centroids: &[Self], indices: &[u8]) -> Vec<Self> {
//         indices
//             .iter()
//             .map(|x| {
//                 *centroids
//                     .get(*x as usize)
//                     .unwrap_or_else(|| centroids.last().unwrap())
//             })
//             .collect()
//     }
// }

impl<T> MapColor for Rgb<T, Srgb>
where
    T: Channel,
{
    #[inline]
    fn map_indices_to_centroids(centroids: &[Self], indices: &[u8]) -> Vec<Self> {
        indices
            .iter()
            .map(|x| {
                *centroids
                    .get(*x as usize)
                    .unwrap_or_else(|| centroids.last().unwrap())
            })
            .collect()
    }
}

impl<T> MapColor for Rgba<T, Srgb>
where
    T: Channel,
{
    #[inline]
    fn map_indices_to_centroids(centroids: &[Self], indices: &[u8]) -> Vec<Self> {
        indices
            .iter()
            .map(|x| {
                *centroids
                    .get(*x as usize)
                    .unwrap_or_else(|| centroids.last().unwrap())
            })
            .collect()
    }
}


impl<Wp: WhitePoint + Copy> Sort for Lab<f32, Wp> {
    fn get_dominant_color(data: &[CentroidData<Self>]) -> Option<Self> {
        let res = data
            .iter()
            .max_by(|a, b| (a.percentage).partial_cmp(&b.percentage).unwrap())
            .unwrap();

        Some(res.centroid)
    }

    fn sort_indexed_colors(centroids: &Vec<Self>, indices: &[u8]) -> Vec<CentroidData<Self>> {
        // Count occurences of each color - "histogram"
        let mut map: std::collections::HashMap<u8, u64> = std::collections::HashMap::new();
        for (i, _) in centroids.iter().enumerate() {
            map.insert(i as u8, 0);
        }
        for i in indices {
            let count = map.entry(*i).or_insert(0);
            *count += 1;
        }

        let len = indices.len();
        assert!(len > 0);
        let mut colors: Vec<(u8, f32)> = Vec::with_capacity(centroids.len());
        for (i, _) in centroids.iter().enumerate() {
            let count = map.get(&(i as u8));
            match count {
                Some(x) => colors.push((i as u8, (*x as f32) / (len as f32))),
                None => continue,
            }
        }

        // Sort by increasing luminosity
        let mut lab: Vec<(u8, Self)> = centroids
            .iter()
            .enumerate()
            .map(|(i, x)| (i as u8, *x))
            .collect();
        lab.sort_unstable_by(|a, b| (a.1.l).partial_cmp(&b.1.l).unwrap());

        // Pack the colors and their percentages into the return vector.
        // Get the lab's key from the map, if the key value is greater than one
        // attempt to find the index of it in the colors vec. Push that to the
        // output vec tuple if successful.
        lab.iter()
            .filter_map(|x| map.get_key_value(&x.0))
            .filter(|x| *x.1 > 0)
            .filter_map(|x| match colors.get(*x.0 as usize) {
                Some(x) => match colors.iter().position(|a| a.0 == x.0 as u8) {
                    Some(y) => Some(CentroidData {
                        centroid: *(centroids.get(colors.get(y).unwrap().0 as usize).unwrap()),
                        percentage: colors.get(y).unwrap().1,
                        index: y as u8,
                    }),
                    None => None,
                },
                None => None,
            })
            .collect()
    }
}

impl Sort for Rgb<f32, Srgb> {
    fn get_dominant_color(data: &[CentroidData<Self>]) -> Option<Self> {
        let res = data
            .iter()
            .max_by(|a, b| (a.percentage).partial_cmp(&b.percentage).unwrap())
            .unwrap();

        Some(res.centroid)
    }

    fn sort_indexed_colors(centroids: &Vec<Self>, indices: &[u8]) -> Vec<CentroidData<Self>> {
        // Count occurences of each color - "histogram"
        let mut map: std::collections::HashMap<u8, u64> = std::collections::HashMap::new();
        for (i, _) in centroids.iter().enumerate() {
            map.insert(i as u8, 0);
        }
        for i in indices {
            let count = map.entry(*i).or_insert(0);
            *count += 1;
        }

        let len = indices.len();
        assert!(len > 0);
        let mut colors: Vec<(u8, f32)> = Vec::with_capacity(centroids.len());
        for (i, _) in centroids.iter().enumerate() {
            let count = map.get(&(i as u8));
            match count {
                Some(x) => colors.push((i as u8, (*x as f32) / (len as f32))),
                None => continue,
            }
        }

        // Sort by increasing luminosity
        let mut lab: Vec<(u8, Luma<f32>)> = centroids
            .iter()
            .enumerate()
            .map(|(i, x)| (i as u8, x.to_luma().into()))
            .collect();
        lab.sort_unstable_by(|a, b| (a.1.l).partial_cmp(&b.1.l).unwrap());

        // Pack the colors and their percentages into the return vector
        lab.iter()
            .filter_map(|x| map.get_key_value(&x.0))
            .filter(|x| *x.1 > 0)
            .filter_map(|x| match colors.get(*x.0 as usize) {
                Some(x) => match colors.iter().position(|a| a.0 == x.0 as u8) {
                    Some(y) => Some(CentroidData {
                        centroid: *(centroids.get(colors.get(y).unwrap().0 as usize).unwrap()),
                        percentage: colors.get(y).unwrap().1,
                        index: y as u8,
                    }),
                    None => None,
                },
                None => None,
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::kmeans::{CentroidData, Sort};
    use crate::{Rgb, color_space::Srgb};

    #[test]
    fn dominant_color() {
        let res = vec![
            CentroidData::<Rgb<f32, Srgb>> {
                centroid: Rgb::new(0.0, 0.0, 0.0),
                percentage: 0.5,
                index: 0,
            },
            CentroidData::<Rgb<f32, Srgb>> {
                centroid: Rgb::new(0.5, 0.5, 0.5),
                percentage: 0.80,
                index: 0,
            },
            CentroidData::<Rgb<f32, Srgb>> {
                centroid: Rgb::new(1.0, 1.0, 1.0),
                percentage: 0.15,
                index: 0,
            },
        ];
        assert_eq!(
            Rgb::get_dominant_color(&res).unwrap(),
            Rgb::new(0.5, 0.5, 0.5)
        );
    }
}