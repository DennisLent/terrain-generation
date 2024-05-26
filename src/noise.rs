
// simplex noise generation
// inspired by perlin noise and algorithm used in the resources
// taken from: https://procedural-content-generation.fandom.com/wiki/Simplex_Noise#:~:text=Simplex%20noise%20is%20an%20improved,the%20shape%20is%20a%20triangle.
pub struct SimplexNoise;

impl SimplexNoise {
    const GRAD3: [[i32; 3]; 12] = [
        [1, 1, 0], [-1, 1, 0], [1, -1, 0], [-1, -1, 0],
        [1, 0, 1], [-1, 0, 1], [1, 0, -1], [-1, 0, -1],
        [0, 1, 1], [0, -1, 1], [0, 1, -1], [0, -1, -1]
    ];

    const P: [i32; 256] = [
        151, 160, 137, 91, 90, 15, 131, 13, 201, 95, 96, 53, 194, 233, 7, 225, 140, 36, 103, 30, 69, 142, 8, 99, 37, 240, 21, 10, 23,
        190, 6, 148, 247, 120, 234, 75, 0, 26, 197, 62, 94, 252, 219, 203, 117, 35, 11, 32, 57, 177, 33, 88, 237, 149, 56, 87, 174, 20,
        125, 136, 171, 168, 68, 175, 74, 165, 71, 134, 139, 48, 27, 166, 77, 146, 158, 231, 83, 111, 229, 122, 60, 211, 133, 230, 220,
        105, 92, 41, 55, 46, 245, 40, 244, 102, 143, 54, 65, 25, 63, 161, 1, 216, 80, 73, 209, 76, 132, 187, 208, 89, 18, 169, 200, 196,
        135, 130, 116, 188, 159, 86, 164, 100, 109, 198, 173, 186, 3, 64, 52, 217, 226, 250, 124, 123, 5, 202, 38, 147, 118, 126, 255,
        82, 85, 212, 207, 206, 59, 227, 47, 16, 58, 17, 182, 189, 28, 42, 223, 183, 170, 213, 119, 248, 152, 2, 44, 154, 163, 70, 221,
        153, 101, 155, 167, 43, 172, 9, 129, 22, 39, 253, 19, 98, 108, 110, 79, 113, 224, 232, 178, 185, 112, 104, 218, 246, 97, 228,
        251, 34, 242, 193, 238, 210, 144, 12, 191, 179, 162, 241, 81, 51, 145, 235, 249, 14, 239, 107, 49, 192, 214, 31, 181, 199, 106,
        157, 184, 84, 204, 176, 115, 121, 50, 45, 127, 4, 150, 254, 138, 236, 205, 93, 222, 114, 67, 29, 24, 72, 243, 141, 128, 195,
        78, 66, 215, 61, 156, 180
    ];

    fn perm() -> [i32; 512] {
        let mut perm = [0; 512];
        for i in 0..512 {
            perm[i] = Self::P[i & 255];
        }
        perm
    }

    fn fastfloor(x: f32) -> i32 {
        if x > 0.0 {
            x as i32
        } else {
            (x as i32) - 1
        }
    }

    fn dot(g: [i32; 3], x: f32, y: f32) -> f32 {
        g[0] as f32 * x + g[1] as f32 * y
    }

    pub fn noise(xin: f32, yin: f32) -> f32 {
        let perm = Self::perm();
        let grad3 = Self::GRAD3;

        let f2 = 0.5 * (3.0_f32.sqrt() - 1.0);
        let s = (xin + yin) * f2;
        let i = Self::fastfloor(xin + s);
        let j = Self::fastfloor(yin + s);

        let g2 = (3.0 - 3.0_f32.sqrt()) / 6.0;
        let t = (i + j) as f32 * g2;
        let x0 = xin - (i as f32 - t);
        let y0 = yin - (j as f32 - t);

        let (i1, j1) = if x0 > y0 { (1, 0) } else { (0, 1) };

        let x1 = x0 - i1 as f32 + g2;
        let y1 = y0 - j1 as f32 + g2;
        let x2 = x0 - 1.0 + 2.0 * g2;
        let y2 = y0 - 1.0 + 2.0 * g2;

        let ii = i & 255;
        let jj = j & 255;
        let gi0 = perm[(ii + perm[jj as usize]) as usize] % 12;
        let gi1 = perm[(ii + i1 + perm[(jj + j1) as usize]) as usize] % 12;
        let gi2 = perm[(ii + 1 + perm[(jj + 1) as usize]) as usize] % 12;

        let t0 = 0.5 - x0 * x0 - y0 * y0;
        let n0 = if t0 < 0.0 {
            0.0
        } else {
            let t0 = t0 * t0;
            t0 * t0 * Self::dot(grad3[gi0 as usize], x0, y0)
        };

        let t1 = 0.5 - x1 * x1 - y1 * y1;
        let n1 = if t1 < 0.0 {
            0.0
        } else {
            let t1 = t1 * t1;
            t1 * t1 * Self::dot(grad3[gi1 as usize], x1, y1)
        };

        let t2 = 0.5 - x2 * x2 - y2 * y2;
        let n2 = if t2 < 0.0 {
            0.0
        } else {
            let t2 = t2 * t2;
            t2 * t2 * Self::dot(grad3[gi2 as usize], x2, y2)
        };

        70.0 * (n0 + n1 + n2)
    }

    pub fn fractal_noise(x: f32, y: f32, octaves: u32, persistence: f32, frequency: f32) -> f32 {
        let mut total = 0.0;
        let mut amplitude = 1.0;
        let mut max_value = 0.0;
        let mut freq = frequency;
    
        for _ in 0..octaves {
            total += SimplexNoise::noise(x * freq, y * freq) * amplitude;
            max_value += amplitude;
            amplitude *= persistence;
            freq *= 2.0;
        }
    
        total / max_value
    }
}