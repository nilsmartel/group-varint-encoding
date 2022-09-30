#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tst1() {
        let data = compress([0u32, 0, 0, 0]);
        assert_eq!(data, [0, 0, 0, 0, 0]);
    }

    #[test]
    fn tst2() {
        let data = compress([0xff, 0xff, 0xff, 0xff]);
        assert_eq!(data, [0, 0xff, 0xff, 0xff]);
    }

    #[test]
    fn tst3() {
        let data = compress([0xffff, 0xff, 0xff, 0]);
        assert_eq!(data, [1, 0xff, 0xff, 0xff, 0]);

        let data = compress([0xff, 0xff01, 0xff, 0]);
        assert_eq!(data, [1 << 2, 0xff, 0xff, 1, 0xff, 0]);

        let data = compress([0xff, 0xff, 0xff01, 0]);
        assert_eq!(data, [1 << 4, 0xff, 0xff, 0xff, 3, 0]);

        let data = compress([0xff, 0xff, 0xff, 0xff03]);
        assert_eq!(data, [1 << 6, 0xff, 0xff, 0xff, 0xff, 3]);
    }
}

pub struct UInt32Decompressor<'a> {
    data: &'a [u8],
}

impl<'a> Iterator for UInt32Decompressor<'a> {
    type Item = [u32; 4];

    fn next(&mut self) -> Option<Self::Item> {
        if self.data.len() == 0 {
            return None;
        }

        let v = self.data[0];
        let data = &self.data[1..];

        let (a, b, c, d) = decode_block(v, data);

        Some([a, b, c, d])
    }
}

fn decode_block(v: u8, data: &[u8]) -> (u32, u32, u32, u32) {
    match v {
        // GEN TABLE HERE
        0 => {
            let v0 = data[0] as u32;
            let v1 = data[1] as u32;
            let v2 = data[2] as u32;
            let v3 = data[3] as u32;
            (v0, v1, v2, v3)
        }
        1 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8;
            let v1 = data[2] as u32;
            let v2 = data[3] as u32;
            let v3 = data[4] as u32;
            (v0, v1, v2, v3)
        }
        2 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8 | (data[2] as u32) << 16;
            let v1 = data[3] as u32;
            let v2 = data[4] as u32;
            let v3 = data[5] as u32;
            (v0, v1, v2, v3)
        }
        3 => {
            let v0 = (data[0] as u32)
                | (data[1] as u32) << 8
                | (data[2] as u32) << 16
                | (data[3] as u32) << 24;
            let v1 = data[4] as u32;
            let v2 = data[5] as u32;
            let v3 = data[6] as u32;
            (v0, v1, v2, v3)
        }
        4 => {
            let v0 = data[0] as u32;
            let v1 = (data[1] as u32) | (data[2] as u32) << 8;
            let v2 = data[3] as u32;
            let v3 = data[4] as u32;
            (v0, v1, v2, v3)
        }
        5 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8;
            let v1 = (data[2] as u32) | (data[3] as u32) << 8;
            let v2 = data[4] as u32;
            let v3 = data[5] as u32;
            (v0, v1, v2, v3)
        }
        6 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8 | (data[2] as u32) << 16;
            let v1 = (data[3] as u32) | (data[4] as u32) << 8;
            let v2 = data[5] as u32;
            let v3 = data[6] as u32;
            (v0, v1, v2, v3)
        }
        7 => {
            let v0 = (data[0] as u32)
                | (data[1] as u32) << 8
                | (data[2] as u32) << 16
                | (data[3] as u32) << 24;
            let v1 = (data[4] as u32) | (data[5] as u32) << 8;
            let v2 = data[6] as u32;
            let v3 = data[7] as u32;
            (v0, v1, v2, v3)
        }
        8 => {
            let v0 = data[0] as u32;
            let v1 = (data[1] as u32) | (data[2] as u32) << 8 | (data[3] as u32) << 16;
            let v2 = data[4] as u32;
            let v3 = data[5] as u32;
            (v0, v1, v2, v3)
        }
        9 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8;
            let v1 = (data[2] as u32) | (data[3] as u32) << 8 | (data[4] as u32) << 16;
            let v2 = data[5] as u32;
            let v3 = data[6] as u32;
            (v0, v1, v2, v3)
        }
        10 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8 | (data[2] as u32) << 16;
            let v1 = (data[3] as u32) | (data[4] as u32) << 8 | (data[5] as u32) << 16;
            let v2 = data[6] as u32;
            let v3 = data[7] as u32;
            (v0, v1, v2, v3)
        }
        11 => {
            let v0 = (data[0] as u32)
                | (data[1] as u32) << 8
                | (data[2] as u32) << 16
                | (data[3] as u32) << 24;
            let v1 = (data[4] as u32) | (data[5] as u32) << 8 | (data[6] as u32) << 16;
            let v2 = data[7] as u32;
            let v3 = data[8] as u32;
            (v0, v1, v2, v3)
        }
        12 => {
            let v0 = data[0] as u32;
            let v1 = (data[1] as u32)
                | (data[2] as u32) << 8
                | (data[3] as u32) << 16
                | (data[4] as u32) << 24;
            let v2 = data[5] as u32;
            let v3 = data[6] as u32;
            (v0, v1, v2, v3)
        }
        13 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8;
            let v1 = (data[2] as u32)
                | (data[3] as u32) << 8
                | (data[4] as u32) << 16
                | (data[5] as u32) << 24;
            let v2 = data[6] as u32;
            let v3 = data[7] as u32;
            (v0, v1, v2, v3)
        }
        14 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8 | (data[2] as u32) << 16;
            let v1 = (data[3] as u32)
                | (data[4] as u32) << 8
                | (data[5] as u32) << 16
                | (data[6] as u32) << 24;
            let v2 = data[7] as u32;
            let v3 = data[8] as u32;
            (v0, v1, v2, v3)
        }
        15 => {
            let v0 = (data[0] as u32)
                | (data[1] as u32) << 8
                | (data[2] as u32) << 16
                | (data[3] as u32) << 24;
            let v1 = (data[4] as u32)
                | (data[5] as u32) << 8
                | (data[6] as u32) << 16
                | (data[7] as u32) << 24;
            let v2 = data[8] as u32;
            let v3 = data[9] as u32;
            (v0, v1, v2, v3)
        }
        16 => {
            let v0 = data[0] as u32;
            let v1 = data[1] as u32;
            let v2 = (data[2] as u32) | (data[3] as u32) << 8;
            let v3 = data[4] as u32;
            (v0, v1, v2, v3)
        }
        17 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8;
            let v1 = data[2] as u32;
            let v2 = (data[3] as u32) | (data[4] as u32) << 8;
            let v3 = data[5] as u32;
            (v0, v1, v2, v3)
        }
        18 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8 | (data[2] as u32) << 16;
            let v1 = data[3] as u32;
            let v2 = (data[4] as u32) | (data[5] as u32) << 8;
            let v3 = data[6] as u32;
            (v0, v1, v2, v3)
        }
        19 => {
            let v0 = (data[0] as u32)
                | (data[1] as u32) << 8
                | (data[2] as u32) << 16
                | (data[3] as u32) << 24;
            let v1 = data[4] as u32;
            let v2 = (data[5] as u32) | (data[6] as u32) << 8;
            let v3 = data[7] as u32;
            (v0, v1, v2, v3)
        }
        20 => {
            let v0 = data[0] as u32;
            let v1 = (data[1] as u32) | (data[2] as u32) << 8;
            let v2 = (data[3] as u32) | (data[4] as u32) << 8;
            let v3 = data[5] as u32;
            (v0, v1, v2, v3)
        }
        21 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8;
            let v1 = (data[2] as u32) | (data[3] as u32) << 8;
            let v2 = (data[4] as u32) | (data[5] as u32) << 8;
            let v3 = data[6] as u32;
            (v0, v1, v2, v3)
        }
        22 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8 | (data[2] as u32) << 16;
            let v1 = (data[3] as u32) | (data[4] as u32) << 8;
            let v2 = (data[5] as u32) | (data[6] as u32) << 8;
            let v3 = data[7] as u32;
            (v0, v1, v2, v3)
        }
        23 => {
            let v0 = (data[0] as u32)
                | (data[1] as u32) << 8
                | (data[2] as u32) << 16
                | (data[3] as u32) << 24;
            let v1 = (data[4] as u32) | (data[5] as u32) << 8;
            let v2 = (data[6] as u32) | (data[7] as u32) << 8;
            let v3 = data[8] as u32;
            (v0, v1, v2, v3)
        }
        24 => {
            let v0 = data[0] as u32;
            let v1 = (data[1] as u32) | (data[2] as u32) << 8 | (data[3] as u32) << 16;
            let v2 = (data[4] as u32) | (data[5] as u32) << 8;
            let v3 = data[6] as u32;
            (v0, v1, v2, v3)
        }
        25 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8;
            let v1 = (data[2] as u32) | (data[3] as u32) << 8 | (data[4] as u32) << 16;
            let v2 = (data[5] as u32) | (data[6] as u32) << 8;
            let v3 = data[7] as u32;
            (v0, v1, v2, v3)
        }
        26 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8 | (data[2] as u32) << 16;
            let v1 = (data[3] as u32) | (data[4] as u32) << 8 | (data[5] as u32) << 16;
            let v2 = (data[6] as u32) | (data[7] as u32) << 8;
            let v3 = data[8] as u32;
            (v0, v1, v2, v3)
        }
        27 => {
            let v0 = (data[0] as u32)
                | (data[1] as u32) << 8
                | (data[2] as u32) << 16
                | (data[3] as u32) << 24;
            let v1 = (data[4] as u32) | (data[5] as u32) << 8 | (data[6] as u32) << 16;
            let v2 = (data[7] as u32) | (data[8] as u32) << 8;
            let v3 = data[9] as u32;
            (v0, v1, v2, v3)
        }
        28 => {
            let v0 = data[0] as u32;
            let v1 = (data[1] as u32)
                | (data[2] as u32) << 8
                | (data[3] as u32) << 16
                | (data[4] as u32) << 24;
            let v2 = (data[5] as u32) | (data[6] as u32) << 8;
            let v3 = data[7] as u32;
            (v0, v1, v2, v3)
        }
        29 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8;
            let v1 = (data[2] as u32)
                | (data[3] as u32) << 8
                | (data[4] as u32) << 16
                | (data[5] as u32) << 24;
            let v2 = (data[6] as u32) | (data[7] as u32) << 8;
            let v3 = data[8] as u32;
            (v0, v1, v2, v3)
        }
        30 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8 | (data[2] as u32) << 16;
            let v1 = (data[3] as u32)
                | (data[4] as u32) << 8
                | (data[5] as u32) << 16
                | (data[6] as u32) << 24;
            let v2 = (data[7] as u32) | (data[8] as u32) << 8;
            let v3 = data[9] as u32;
            (v0, v1, v2, v3)
        }
        31 => {
            let v0 = (data[0] as u32)
                | (data[1] as u32) << 8
                | (data[2] as u32) << 16
                | (data[3] as u32) << 24;
            let v1 = (data[4] as u32)
                | (data[5] as u32) << 8
                | (data[6] as u32) << 16
                | (data[7] as u32) << 24;
            let v2 = (data[8] as u32) | (data[9] as u32) << 8;
            let v3 = data[10] as u32;
            (v0, v1, v2, v3)
        }
        32 => {
            let v0 = data[0] as u32;
            let v1 = data[1] as u32;
            let v2 = (data[2] as u32) | (data[3] as u32) << 8 | (data[4] as u32) << 16;
            let v3 = data[5] as u32;
            (v0, v1, v2, v3)
        }
        33 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8;
            let v1 = data[2] as u32;
            let v2 = (data[3] as u32) | (data[4] as u32) << 8 | (data[5] as u32) << 16;
            let v3 = data[6] as u32;
            (v0, v1, v2, v3)
        }
        34 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8 | (data[2] as u32) << 16;
            let v1 = data[3] as u32;
            let v2 = (data[4] as u32) | (data[5] as u32) << 8 | (data[6] as u32) << 16;
            let v3 = data[7] as u32;
            (v0, v1, v2, v3)
        }
        35 => {
            let v0 = (data[0] as u32)
                | (data[1] as u32) << 8
                | (data[2] as u32) << 16
                | (data[3] as u32) << 24;
            let v1 = data[4] as u32;
            let v2 = (data[5] as u32) | (data[6] as u32) << 8 | (data[7] as u32) << 16;
            let v3 = data[8] as u32;
            (v0, v1, v2, v3)
        }
        36 => {
            let v0 = data[0] as u32;
            let v1 = (data[1] as u32) | (data[2] as u32) << 8;
            let v2 = (data[3] as u32) | (data[4] as u32) << 8 | (data[5] as u32) << 16;
            let v3 = data[6] as u32;
            (v0, v1, v2, v3)
        }
        37 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8;
            let v1 = (data[2] as u32) | (data[3] as u32) << 8;
            let v2 = (data[4] as u32) | (data[5] as u32) << 8 | (data[6] as u32) << 16;
            let v3 = data[7] as u32;
            (v0, v1, v2, v3)
        }
        38 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8 | (data[2] as u32) << 16;
            let v1 = (data[3] as u32) | (data[4] as u32) << 8;
            let v2 = (data[5] as u32) | (data[6] as u32) << 8 | (data[7] as u32) << 16;
            let v3 = data[8] as u32;
            (v0, v1, v2, v3)
        }
        39 => {
            let v0 = (data[0] as u32)
                | (data[1] as u32) << 8
                | (data[2] as u32) << 16
                | (data[3] as u32) << 24;
            let v1 = (data[4] as u32) | (data[5] as u32) << 8;
            let v2 = (data[6] as u32) | (data[7] as u32) << 8 | (data[8] as u32) << 16;
            let v3 = data[9] as u32;
            (v0, v1, v2, v3)
        }
        40 => {
            let v0 = data[0] as u32;
            let v1 = (data[1] as u32) | (data[2] as u32) << 8 | (data[3] as u32) << 16;
            let v2 = (data[4] as u32) | (data[5] as u32) << 8 | (data[6] as u32) << 16;
            let v3 = data[7] as u32;
            (v0, v1, v2, v3)
        }
        41 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8;
            let v1 = (data[2] as u32) | (data[3] as u32) << 8 | (data[4] as u32) << 16;
            let v2 = (data[5] as u32) | (data[6] as u32) << 8 | (data[7] as u32) << 16;
            let v3 = data[8] as u32;
            (v0, v1, v2, v3)
        }
        42 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8 | (data[2] as u32) << 16;
            let v1 = (data[3] as u32) | (data[4] as u32) << 8 | (data[5] as u32) << 16;
            let v2 = (data[6] as u32) | (data[7] as u32) << 8 | (data[8] as u32) << 16;
            let v3 = data[9] as u32;
            (v0, v1, v2, v3)
        }
        43 => {
            let v0 = (data[0] as u32)
                | (data[1] as u32) << 8
                | (data[2] as u32) << 16
                | (data[3] as u32) << 24;
            let v1 = (data[4] as u32) | (data[5] as u32) << 8 | (data[6] as u32) << 16;
            let v2 = (data[7] as u32) | (data[8] as u32) << 8 | (data[9] as u32) << 16;
            let v3 = data[10] as u32;
            (v0, v1, v2, v3)
        }
        44 => {
            let v0 = data[0] as u32;
            let v1 = (data[1] as u32)
                | (data[2] as u32) << 8
                | (data[3] as u32) << 16
                | (data[4] as u32) << 24;
            let v2 = (data[5] as u32) | (data[6] as u32) << 8 | (data[7] as u32) << 16;
            let v3 = data[8] as u32;
            (v0, v1, v2, v3)
        }
        45 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8;
            let v1 = (data[2] as u32)
                | (data[3] as u32) << 8
                | (data[4] as u32) << 16
                | (data[5] as u32) << 24;
            let v2 = (data[6] as u32) | (data[7] as u32) << 8 | (data[8] as u32) << 16;
            let v3 = data[9] as u32;
            (v0, v1, v2, v3)
        }
        46 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8 | (data[2] as u32) << 16;
            let v1 = (data[3] as u32)
                | (data[4] as u32) << 8
                | (data[5] as u32) << 16
                | (data[6] as u32) << 24;
            let v2 = (data[7] as u32) | (data[8] as u32) << 8 | (data[9] as u32) << 16;
            let v3 = data[10] as u32;
            (v0, v1, v2, v3)
        }
        47 => {
            let v0 = (data[0] as u32)
                | (data[1] as u32) << 8
                | (data[2] as u32) << 16
                | (data[3] as u32) << 24;
            let v1 = (data[4] as u32)
                | (data[5] as u32) << 8
                | (data[6] as u32) << 16
                | (data[7] as u32) << 24;
            let v2 = (data[8] as u32) | (data[9] as u32) << 8 | (data[10] as u32) << 16;
            let v3 = data[11] as u32;
            (v0, v1, v2, v3)
        }
        48 => {
            let v0 = data[0] as u32;
            let v1 = data[1] as u32;
            let v2 = (data[2] as u32)
                | (data[3] as u32) << 8
                | (data[4] as u32) << 16
                | (data[5] as u32) << 24;
            let v3 = data[6] as u32;
            (v0, v1, v2, v3)
        }
        49 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8;
            let v1 = data[2] as u32;
            let v2 = (data[3] as u32)
                | (data[4] as u32) << 8
                | (data[5] as u32) << 16
                | (data[6] as u32) << 24;
            let v3 = data[7] as u32;
            (v0, v1, v2, v3)
        }
        50 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8 | (data[2] as u32) << 16;
            let v1 = data[3] as u32;
            let v2 = (data[4] as u32)
                | (data[5] as u32) << 8
                | (data[6] as u32) << 16
                | (data[7] as u32) << 24;
            let v3 = data[8] as u32;
            (v0, v1, v2, v3)
        }
        51 => {
            let v0 = (data[0] as u32)
                | (data[1] as u32) << 8
                | (data[2] as u32) << 16
                | (data[3] as u32) << 24;
            let v1 = data[4] as u32;
            let v2 = (data[5] as u32)
                | (data[6] as u32) << 8
                | (data[7] as u32) << 16
                | (data[8] as u32) << 24;
            let v3 = data[9] as u32;
            (v0, v1, v2, v3)
        }
        52 => {
            let v0 = data[0] as u32;
            let v1 = (data[1] as u32) | (data[2] as u32) << 8;
            let v2 = (data[3] as u32)
                | (data[4] as u32) << 8
                | (data[5] as u32) << 16
                | (data[6] as u32) << 24;
            let v3 = data[7] as u32;
            (v0, v1, v2, v3)
        }
        53 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8;
            let v1 = (data[2] as u32) | (data[3] as u32) << 8;
            let v2 = (data[4] as u32)
                | (data[5] as u32) << 8
                | (data[6] as u32) << 16
                | (data[7] as u32) << 24;
            let v3 = data[8] as u32;
            (v0, v1, v2, v3)
        }
        54 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8 | (data[2] as u32) << 16;
            let v1 = (data[3] as u32) | (data[4] as u32) << 8;
            let v2 = (data[5] as u32)
                | (data[6] as u32) << 8
                | (data[7] as u32) << 16
                | (data[8] as u32) << 24;
            let v3 = data[9] as u32;
            (v0, v1, v2, v3)
        }
        55 => {
            let v0 = (data[0] as u32)
                | (data[1] as u32) << 8
                | (data[2] as u32) << 16
                | (data[3] as u32) << 24;
            let v1 = (data[4] as u32) | (data[5] as u32) << 8;
            let v2 = (data[6] as u32)
                | (data[7] as u32) << 8
                | (data[8] as u32) << 16
                | (data[9] as u32) << 24;
            let v3 = data[10] as u32;
            (v0, v1, v2, v3)
        }
        56 => {
            let v0 = data[0] as u32;
            let v1 = (data[1] as u32) | (data[2] as u32) << 8 | (data[3] as u32) << 16;
            let v2 = (data[4] as u32)
                | (data[5] as u32) << 8
                | (data[6] as u32) << 16
                | (data[7] as u32) << 24;
            let v3 = data[8] as u32;
            (v0, v1, v2, v3)
        }
        57 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8;
            let v1 = (data[2] as u32) | (data[3] as u32) << 8 | (data[4] as u32) << 16;
            let v2 = (data[5] as u32)
                | (data[6] as u32) << 8
                | (data[7] as u32) << 16
                | (data[8] as u32) << 24;
            let v3 = data[9] as u32;
            (v0, v1, v2, v3)
        }
        58 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8 | (data[2] as u32) << 16;
            let v1 = (data[3] as u32) | (data[4] as u32) << 8 | (data[5] as u32) << 16;
            let v2 = (data[6] as u32)
                | (data[7] as u32) << 8
                | (data[8] as u32) << 16
                | (data[9] as u32) << 24;
            let v3 = data[10] as u32;
            (v0, v1, v2, v3)
        }
        59 => {
            let v0 = (data[0] as u32)
                | (data[1] as u32) << 8
                | (data[2] as u32) << 16
                | (data[3] as u32) << 24;
            let v1 = (data[4] as u32) | (data[5] as u32) << 8 | (data[6] as u32) << 16;
            let v2 = (data[7] as u32)
                | (data[8] as u32) << 8
                | (data[9] as u32) << 16
                | (data[10] as u32) << 24;
            let v3 = data[11] as u32;
            (v0, v1, v2, v3)
        }
        60 => {
            let v0 = data[0] as u32;
            let v1 = (data[1] as u32)
                | (data[2] as u32) << 8
                | (data[3] as u32) << 16
                | (data[4] as u32) << 24;
            let v2 = (data[5] as u32)
                | (data[6] as u32) << 8
                | (data[7] as u32) << 16
                | (data[8] as u32) << 24;
            let v3 = data[9] as u32;
            (v0, v1, v2, v3)
        }
        61 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8;
            let v1 = (data[2] as u32)
                | (data[3] as u32) << 8
                | (data[4] as u32) << 16
                | (data[5] as u32) << 24;
            let v2 = (data[6] as u32)
                | (data[7] as u32) << 8
                | (data[8] as u32) << 16
                | (data[9] as u32) << 24;
            let v3 = data[10] as u32;
            (v0, v1, v2, v3)
        }
        62 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8 | (data[2] as u32) << 16;
            let v1 = (data[3] as u32)
                | (data[4] as u32) << 8
                | (data[5] as u32) << 16
                | (data[6] as u32) << 24;
            let v2 = (data[7] as u32)
                | (data[8] as u32) << 8
                | (data[9] as u32) << 16
                | (data[10] as u32) << 24;
            let v3 = data[11] as u32;
            (v0, v1, v2, v3)
        }
        63 => {
            let v0 = (data[0] as u32)
                | (data[1] as u32) << 8
                | (data[2] as u32) << 16
                | (data[3] as u32) << 24;
            let v1 = (data[4] as u32)
                | (data[5] as u32) << 8
                | (data[6] as u32) << 16
                | (data[7] as u32) << 24;
            let v2 = (data[8] as u32)
                | (data[9] as u32) << 8
                | (data[10] as u32) << 16
                | (data[11] as u32) << 24;
            let v3 = data[12] as u32;
            (v0, v1, v2, v3)
        }
        64 => {
            let v0 = data[0] as u32;
            let v1 = data[1] as u32;
            let v2 = data[2] as u32;
            let v3 = (data[3] as u32) | (data[4] as u32) << 8;
            (v0, v1, v2, v3)
        }
        65 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8;
            let v1 = data[2] as u32;
            let v2 = data[3] as u32;
            let v3 = (data[4] as u32) | (data[5] as u32) << 8;
            (v0, v1, v2, v3)
        }
        66 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8 | (data[2] as u32) << 16;
            let v1 = data[3] as u32;
            let v2 = data[4] as u32;
            let v3 = (data[5] as u32) | (data[6] as u32) << 8;
            (v0, v1, v2, v3)
        }
        67 => {
            let v0 = (data[0] as u32)
                | (data[1] as u32) << 8
                | (data[2] as u32) << 16
                | (data[3] as u32) << 24;
            let v1 = data[4] as u32;
            let v2 = data[5] as u32;
            let v3 = (data[6] as u32) | (data[7] as u32) << 8;
            (v0, v1, v2, v3)
        }
        68 => {
            let v0 = data[0] as u32;
            let v1 = (data[1] as u32) | (data[2] as u32) << 8;
            let v2 = data[3] as u32;
            let v3 = (data[4] as u32) | (data[5] as u32) << 8;
            (v0, v1, v2, v3)
        }
        69 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8;
            let v1 = (data[2] as u32) | (data[3] as u32) << 8;
            let v2 = data[4] as u32;
            let v3 = (data[5] as u32) | (data[6] as u32) << 8;
            (v0, v1, v2, v3)
        }
        70 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8 | (data[2] as u32) << 16;
            let v1 = (data[3] as u32) | (data[4] as u32) << 8;
            let v2 = data[5] as u32;
            let v3 = (data[6] as u32) | (data[7] as u32) << 8;
            (v0, v1, v2, v3)
        }
        71 => {
            let v0 = (data[0] as u32)
                | (data[1] as u32) << 8
                | (data[2] as u32) << 16
                | (data[3] as u32) << 24;
            let v1 = (data[4] as u32) | (data[5] as u32) << 8;
            let v2 = data[6] as u32;
            let v3 = (data[7] as u32) | (data[8] as u32) << 8;
            (v0, v1, v2, v3)
        }
        72 => {
            let v0 = data[0] as u32;
            let v1 = (data[1] as u32) | (data[2] as u32) << 8 | (data[3] as u32) << 16;
            let v2 = data[4] as u32;
            let v3 = (data[5] as u32) | (data[6] as u32) << 8;
            (v0, v1, v2, v3)
        }
        73 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8;
            let v1 = (data[2] as u32) | (data[3] as u32) << 8 | (data[4] as u32) << 16;
            let v2 = data[5] as u32;
            let v3 = (data[6] as u32) | (data[7] as u32) << 8;
            (v0, v1, v2, v3)
        }
        74 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8 | (data[2] as u32) << 16;
            let v1 = (data[3] as u32) | (data[4] as u32) << 8 | (data[5] as u32) << 16;
            let v2 = data[6] as u32;
            let v3 = (data[7] as u32) | (data[8] as u32) << 8;
            (v0, v1, v2, v3)
        }
        75 => {
            let v0 = (data[0] as u32)
                | (data[1] as u32) << 8
                | (data[2] as u32) << 16
                | (data[3] as u32) << 24;
            let v1 = (data[4] as u32) | (data[5] as u32) << 8 | (data[6] as u32) << 16;
            let v2 = data[7] as u32;
            let v3 = (data[8] as u32) | (data[9] as u32) << 8;
            (v0, v1, v2, v3)
        }
        76 => {
            let v0 = data[0] as u32;
            let v1 = (data[1] as u32)
                | (data[2] as u32) << 8
                | (data[3] as u32) << 16
                | (data[4] as u32) << 24;
            let v2 = data[5] as u32;
            let v3 = (data[6] as u32) | (data[7] as u32) << 8;
            (v0, v1, v2, v3)
        }
        77 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8;
            let v1 = (data[2] as u32)
                | (data[3] as u32) << 8
                | (data[4] as u32) << 16
                | (data[5] as u32) << 24;
            let v2 = data[6] as u32;
            let v3 = (data[7] as u32) | (data[8] as u32) << 8;
            (v0, v1, v2, v3)
        }
        78 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8 | (data[2] as u32) << 16;
            let v1 = (data[3] as u32)
                | (data[4] as u32) << 8
                | (data[5] as u32) << 16
                | (data[6] as u32) << 24;
            let v2 = data[7] as u32;
            let v3 = (data[8] as u32) | (data[9] as u32) << 8;
            (v0, v1, v2, v3)
        }
        79 => {
            let v0 = (data[0] as u32)
                | (data[1] as u32) << 8
                | (data[2] as u32) << 16
                | (data[3] as u32) << 24;
            let v1 = (data[4] as u32)
                | (data[5] as u32) << 8
                | (data[6] as u32) << 16
                | (data[7] as u32) << 24;
            let v2 = data[8] as u32;
            let v3 = (data[9] as u32) | (data[10] as u32) << 8;
            (v0, v1, v2, v3)
        }
        80 => {
            let v0 = data[0] as u32;
            let v1 = data[1] as u32;
            let v2 = (data[2] as u32) | (data[3] as u32) << 8;
            let v3 = (data[4] as u32) | (data[5] as u32) << 8;
            (v0, v1, v2, v3)
        }
        81 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8;
            let v1 = data[2] as u32;
            let v2 = (data[3] as u32) | (data[4] as u32) << 8;
            let v3 = (data[5] as u32) | (data[6] as u32) << 8;
            (v0, v1, v2, v3)
        }
        82 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8 | (data[2] as u32) << 16;
            let v1 = data[3] as u32;
            let v2 = (data[4] as u32) | (data[5] as u32) << 8;
            let v3 = (data[6] as u32) | (data[7] as u32) << 8;
            (v0, v1, v2, v3)
        }
        83 => {
            let v0 = (data[0] as u32)
                | (data[1] as u32) << 8
                | (data[2] as u32) << 16
                | (data[3] as u32) << 24;
            let v1 = data[4] as u32;
            let v2 = (data[5] as u32) | (data[6] as u32) << 8;
            let v3 = (data[7] as u32) | (data[8] as u32) << 8;
            (v0, v1, v2, v3)
        }
        84 => {
            let v0 = data[0] as u32;
            let v1 = (data[1] as u32) | (data[2] as u32) << 8;
            let v2 = (data[3] as u32) | (data[4] as u32) << 8;
            let v3 = (data[5] as u32) | (data[6] as u32) << 8;
            (v0, v1, v2, v3)
        }
        85 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8;
            let v1 = (data[2] as u32) | (data[3] as u32) << 8;
            let v2 = (data[4] as u32) | (data[5] as u32) << 8;
            let v3 = (data[6] as u32) | (data[7] as u32) << 8;
            (v0, v1, v2, v3)
        }
        86 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8 | (data[2] as u32) << 16;
            let v1 = (data[3] as u32) | (data[4] as u32) << 8;
            let v2 = (data[5] as u32) | (data[6] as u32) << 8;
            let v3 = (data[7] as u32) | (data[8] as u32) << 8;
            (v0, v1, v2, v3)
        }
        87 => {
            let v0 = (data[0] as u32)
                | (data[1] as u32) << 8
                | (data[2] as u32) << 16
                | (data[3] as u32) << 24;
            let v1 = (data[4] as u32) | (data[5] as u32) << 8;
            let v2 = (data[6] as u32) | (data[7] as u32) << 8;
            let v3 = (data[8] as u32) | (data[9] as u32) << 8;
            (v0, v1, v2, v3)
        }
        88 => {
            let v0 = data[0] as u32;
            let v1 = (data[1] as u32) | (data[2] as u32) << 8 | (data[3] as u32) << 16;
            let v2 = (data[4] as u32) | (data[5] as u32) << 8;
            let v3 = (data[6] as u32) | (data[7] as u32) << 8;
            (v0, v1, v2, v3)
        }
        89 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8;
            let v1 = (data[2] as u32) | (data[3] as u32) << 8 | (data[4] as u32) << 16;
            let v2 = (data[5] as u32) | (data[6] as u32) << 8;
            let v3 = (data[7] as u32) | (data[8] as u32) << 8;
            (v0, v1, v2, v3)
        }
        90 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8 | (data[2] as u32) << 16;
            let v1 = (data[3] as u32) | (data[4] as u32) << 8 | (data[5] as u32) << 16;
            let v2 = (data[6] as u32) | (data[7] as u32) << 8;
            let v3 = (data[8] as u32) | (data[9] as u32) << 8;
            (v0, v1, v2, v3)
        }
        91 => {
            let v0 = (data[0] as u32)
                | (data[1] as u32) << 8
                | (data[2] as u32) << 16
                | (data[3] as u32) << 24;
            let v1 = (data[4] as u32) | (data[5] as u32) << 8 | (data[6] as u32) << 16;
            let v2 = (data[7] as u32) | (data[8] as u32) << 8;
            let v3 = (data[9] as u32) | (data[10] as u32) << 8;
            (v0, v1, v2, v3)
        }
        92 => {
            let v0 = data[0] as u32;
            let v1 = (data[1] as u32)
                | (data[2] as u32) << 8
                | (data[3] as u32) << 16
                | (data[4] as u32) << 24;
            let v2 = (data[5] as u32) | (data[6] as u32) << 8;
            let v3 = (data[7] as u32) | (data[8] as u32) << 8;
            (v0, v1, v2, v3)
        }
        93 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8;
            let v1 = (data[2] as u32)
                | (data[3] as u32) << 8
                | (data[4] as u32) << 16
                | (data[5] as u32) << 24;
            let v2 = (data[6] as u32) | (data[7] as u32) << 8;
            let v3 = (data[8] as u32) | (data[9] as u32) << 8;
            (v0, v1, v2, v3)
        }
        94 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8 | (data[2] as u32) << 16;
            let v1 = (data[3] as u32)
                | (data[4] as u32) << 8
                | (data[5] as u32) << 16
                | (data[6] as u32) << 24;
            let v2 = (data[7] as u32) | (data[8] as u32) << 8;
            let v3 = (data[9] as u32) | (data[10] as u32) << 8;
            (v0, v1, v2, v3)
        }
        95 => {
            let v0 = (data[0] as u32)
                | (data[1] as u32) << 8
                | (data[2] as u32) << 16
                | (data[3] as u32) << 24;
            let v1 = (data[4] as u32)
                | (data[5] as u32) << 8
                | (data[6] as u32) << 16
                | (data[7] as u32) << 24;
            let v2 = (data[8] as u32) | (data[9] as u32) << 8;
            let v3 = (data[10] as u32) | (data[11] as u32) << 8;
            (v0, v1, v2, v3)
        }
        96 => {
            let v0 = data[0] as u32;
            let v1 = data[1] as u32;
            let v2 = (data[2] as u32) | (data[3] as u32) << 8 | (data[4] as u32) << 16;
            let v3 = (data[5] as u32) | (data[6] as u32) << 8;
            (v0, v1, v2, v3)
        }
        97 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8;
            let v1 = data[2] as u32;
            let v2 = (data[3] as u32) | (data[4] as u32) << 8 | (data[5] as u32) << 16;
            let v3 = (data[6] as u32) | (data[7] as u32) << 8;
            (v0, v1, v2, v3)
        }
        98 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8 | (data[2] as u32) << 16;
            let v1 = data[3] as u32;
            let v2 = (data[4] as u32) | (data[5] as u32) << 8 | (data[6] as u32) << 16;
            let v3 = (data[7] as u32) | (data[8] as u32) << 8;
            (v0, v1, v2, v3)
        }
        99 => {
            let v0 = (data[0] as u32)
                | (data[1] as u32) << 8
                | (data[2] as u32) << 16
                | (data[3] as u32) << 24;
            let v1 = data[4] as u32;
            let v2 = (data[5] as u32) | (data[6] as u32) << 8 | (data[7] as u32) << 16;
            let v3 = (data[8] as u32) | (data[9] as u32) << 8;
            (v0, v1, v2, v3)
        }
        100 => {
            let v0 = data[0] as u32;
            let v1 = (data[1] as u32) | (data[2] as u32) << 8;
            let v2 = (data[3] as u32) | (data[4] as u32) << 8 | (data[5] as u32) << 16;
            let v3 = (data[6] as u32) | (data[7] as u32) << 8;
            (v0, v1, v2, v3)
        }
        101 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8;
            let v1 = (data[2] as u32) | (data[3] as u32) << 8;
            let v2 = (data[4] as u32) | (data[5] as u32) << 8 | (data[6] as u32) << 16;
            let v3 = (data[7] as u32) | (data[8] as u32) << 8;
            (v0, v1, v2, v3)
        }
        102 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8 | (data[2] as u32) << 16;
            let v1 = (data[3] as u32) | (data[4] as u32) << 8;
            let v2 = (data[5] as u32) | (data[6] as u32) << 8 | (data[7] as u32) << 16;
            let v3 = (data[8] as u32) | (data[9] as u32) << 8;
            (v0, v1, v2, v3)
        }
        103 => {
            let v0 = (data[0] as u32)
                | (data[1] as u32) << 8
                | (data[2] as u32) << 16
                | (data[3] as u32) << 24;
            let v1 = (data[4] as u32) | (data[5] as u32) << 8;
            let v2 = (data[6] as u32) | (data[7] as u32) << 8 | (data[8] as u32) << 16;
            let v3 = (data[9] as u32) | (data[10] as u32) << 8;
            (v0, v1, v2, v3)
        }
        104 => {
            let v0 = data[0] as u32;
            let v1 = (data[1] as u32) | (data[2] as u32) << 8 | (data[3] as u32) << 16;
            let v2 = (data[4] as u32) | (data[5] as u32) << 8 | (data[6] as u32) << 16;
            let v3 = (data[7] as u32) | (data[8] as u32) << 8;
            (v0, v1, v2, v3)
        }
        105 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8;
            let v1 = (data[2] as u32) | (data[3] as u32) << 8 | (data[4] as u32) << 16;
            let v2 = (data[5] as u32) | (data[6] as u32) << 8 | (data[7] as u32) << 16;
            let v3 = (data[8] as u32) | (data[9] as u32) << 8;
            (v0, v1, v2, v3)
        }
        106 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8 | (data[2] as u32) << 16;
            let v1 = (data[3] as u32) | (data[4] as u32) << 8 | (data[5] as u32) << 16;
            let v2 = (data[6] as u32) | (data[7] as u32) << 8 | (data[8] as u32) << 16;
            let v3 = (data[9] as u32) | (data[10] as u32) << 8;
            (v0, v1, v2, v3)
        }
        107 => {
            let v0 = (data[0] as u32)
                | (data[1] as u32) << 8
                | (data[2] as u32) << 16
                | (data[3] as u32) << 24;
            let v1 = (data[4] as u32) | (data[5] as u32) << 8 | (data[6] as u32) << 16;
            let v2 = (data[7] as u32) | (data[8] as u32) << 8 | (data[9] as u32) << 16;
            let v3 = (data[10] as u32) | (data[11] as u32) << 8;
            (v0, v1, v2, v3)
        }
        108 => {
            let v0 = data[0] as u32;
            let v1 = (data[1] as u32)
                | (data[2] as u32) << 8
                | (data[3] as u32) << 16
                | (data[4] as u32) << 24;
            let v2 = (data[5] as u32) | (data[6] as u32) << 8 | (data[7] as u32) << 16;
            let v3 = (data[8] as u32) | (data[9] as u32) << 8;
            (v0, v1, v2, v3)
        }
        109 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8;
            let v1 = (data[2] as u32)
                | (data[3] as u32) << 8
                | (data[4] as u32) << 16
                | (data[5] as u32) << 24;
            let v2 = (data[6] as u32) | (data[7] as u32) << 8 | (data[8] as u32) << 16;
            let v3 = (data[9] as u32) | (data[10] as u32) << 8;
            (v0, v1, v2, v3)
        }
        110 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8 | (data[2] as u32) << 16;
            let v1 = (data[3] as u32)
                | (data[4] as u32) << 8
                | (data[5] as u32) << 16
                | (data[6] as u32) << 24;
            let v2 = (data[7] as u32) | (data[8] as u32) << 8 | (data[9] as u32) << 16;
            let v3 = (data[10] as u32) | (data[11] as u32) << 8;
            (v0, v1, v2, v3)
        }
        111 => {
            let v0 = (data[0] as u32)
                | (data[1] as u32) << 8
                | (data[2] as u32) << 16
                | (data[3] as u32) << 24;
            let v1 = (data[4] as u32)
                | (data[5] as u32) << 8
                | (data[6] as u32) << 16
                | (data[7] as u32) << 24;
            let v2 = (data[8] as u32) | (data[9] as u32) << 8 | (data[10] as u32) << 16;
            let v3 = (data[11] as u32) | (data[12] as u32) << 8;
            (v0, v1, v2, v3)
        }
        112 => {
            let v0 = data[0] as u32;
            let v1 = data[1] as u32;
            let v2 = (data[2] as u32)
                | (data[3] as u32) << 8
                | (data[4] as u32) << 16
                | (data[5] as u32) << 24;
            let v3 = (data[6] as u32) | (data[7] as u32) << 8;
            (v0, v1, v2, v3)
        }
        113 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8;
            let v1 = data[2] as u32;
            let v2 = (data[3] as u32)
                | (data[4] as u32) << 8
                | (data[5] as u32) << 16
                | (data[6] as u32) << 24;
            let v3 = (data[7] as u32) | (data[8] as u32) << 8;
            (v0, v1, v2, v3)
        }
        114 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8 | (data[2] as u32) << 16;
            let v1 = data[3] as u32;
            let v2 = (data[4] as u32)
                | (data[5] as u32) << 8
                | (data[6] as u32) << 16
                | (data[7] as u32) << 24;
            let v3 = (data[8] as u32) | (data[9] as u32) << 8;
            (v0, v1, v2, v3)
        }
        115 => {
            let v0 = (data[0] as u32)
                | (data[1] as u32) << 8
                | (data[2] as u32) << 16
                | (data[3] as u32) << 24;
            let v1 = data[4] as u32;
            let v2 = (data[5] as u32)
                | (data[6] as u32) << 8
                | (data[7] as u32) << 16
                | (data[8] as u32) << 24;
            let v3 = (data[9] as u32) | (data[10] as u32) << 8;
            (v0, v1, v2, v3)
        }
        116 => {
            let v0 = data[0] as u32;
            let v1 = (data[1] as u32) | (data[2] as u32) << 8;
            let v2 = (data[3] as u32)
                | (data[4] as u32) << 8
                | (data[5] as u32) << 16
                | (data[6] as u32) << 24;
            let v3 = (data[7] as u32) | (data[8] as u32) << 8;
            (v0, v1, v2, v3)
        }
        117 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8;
            let v1 = (data[2] as u32) | (data[3] as u32) << 8;
            let v2 = (data[4] as u32)
                | (data[5] as u32) << 8
                | (data[6] as u32) << 16
                | (data[7] as u32) << 24;
            let v3 = (data[8] as u32) | (data[9] as u32) << 8;
            (v0, v1, v2, v3)
        }
        118 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8 | (data[2] as u32) << 16;
            let v1 = (data[3] as u32) | (data[4] as u32) << 8;
            let v2 = (data[5] as u32)
                | (data[6] as u32) << 8
                | (data[7] as u32) << 16
                | (data[8] as u32) << 24;
            let v3 = (data[9] as u32) | (data[10] as u32) << 8;
            (v0, v1, v2, v3)
        }
        119 => {
            let v0 = (data[0] as u32)
                | (data[1] as u32) << 8
                | (data[2] as u32) << 16
                | (data[3] as u32) << 24;
            let v1 = (data[4] as u32) | (data[5] as u32) << 8;
            let v2 = (data[6] as u32)
                | (data[7] as u32) << 8
                | (data[8] as u32) << 16
                | (data[9] as u32) << 24;
            let v3 = (data[10] as u32) | (data[11] as u32) << 8;
            (v0, v1, v2, v3)
        }
        120 => {
            let v0 = data[0] as u32;
            let v1 = (data[1] as u32) | (data[2] as u32) << 8 | (data[3] as u32) << 16;
            let v2 = (data[4] as u32)
                | (data[5] as u32) << 8
                | (data[6] as u32) << 16
                | (data[7] as u32) << 24;
            let v3 = (data[8] as u32) | (data[9] as u32) << 8;
            (v0, v1, v2, v3)
        }
        121 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8;
            let v1 = (data[2] as u32) | (data[3] as u32) << 8 | (data[4] as u32) << 16;
            let v2 = (data[5] as u32)
                | (data[6] as u32) << 8
                | (data[7] as u32) << 16
                | (data[8] as u32) << 24;
            let v3 = (data[9] as u32) | (data[10] as u32) << 8;
            (v0, v1, v2, v3)
        }
        122 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8 | (data[2] as u32) << 16;
            let v1 = (data[3] as u32) | (data[4] as u32) << 8 | (data[5] as u32) << 16;
            let v2 = (data[6] as u32)
                | (data[7] as u32) << 8
                | (data[8] as u32) << 16
                | (data[9] as u32) << 24;
            let v3 = (data[10] as u32) | (data[11] as u32) << 8;
            (v0, v1, v2, v3)
        }
        123 => {
            let v0 = (data[0] as u32)
                | (data[1] as u32) << 8
                | (data[2] as u32) << 16
                | (data[3] as u32) << 24;
            let v1 = (data[4] as u32) | (data[5] as u32) << 8 | (data[6] as u32) << 16;
            let v2 = (data[7] as u32)
                | (data[8] as u32) << 8
                | (data[9] as u32) << 16
                | (data[10] as u32) << 24;
            let v3 = (data[11] as u32) | (data[12] as u32) << 8;
            (v0, v1, v2, v3)
        }
        124 => {
            let v0 = data[0] as u32;
            let v1 = (data[1] as u32)
                | (data[2] as u32) << 8
                | (data[3] as u32) << 16
                | (data[4] as u32) << 24;
            let v2 = (data[5] as u32)
                | (data[6] as u32) << 8
                | (data[7] as u32) << 16
                | (data[8] as u32) << 24;
            let v3 = (data[9] as u32) | (data[10] as u32) << 8;
            (v0, v1, v2, v3)
        }
        125 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8;
            let v1 = (data[2] as u32)
                | (data[3] as u32) << 8
                | (data[4] as u32) << 16
                | (data[5] as u32) << 24;
            let v2 = (data[6] as u32)
                | (data[7] as u32) << 8
                | (data[8] as u32) << 16
                | (data[9] as u32) << 24;
            let v3 = (data[10] as u32) | (data[11] as u32) << 8;
            (v0, v1, v2, v3)
        }
        126 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8 | (data[2] as u32) << 16;
            let v1 = (data[3] as u32)
                | (data[4] as u32) << 8
                | (data[5] as u32) << 16
                | (data[6] as u32) << 24;
            let v2 = (data[7] as u32)
                | (data[8] as u32) << 8
                | (data[9] as u32) << 16
                | (data[10] as u32) << 24;
            let v3 = (data[11] as u32) | (data[12] as u32) << 8;
            (v0, v1, v2, v3)
        }
        127 => {
            let v0 = (data[0] as u32)
                | (data[1] as u32) << 8
                | (data[2] as u32) << 16
                | (data[3] as u32) << 24;
            let v1 = (data[4] as u32)
                | (data[5] as u32) << 8
                | (data[6] as u32) << 16
                | (data[7] as u32) << 24;
            let v2 = (data[8] as u32)
                | (data[9] as u32) << 8
                | (data[10] as u32) << 16
                | (data[11] as u32) << 24;
            let v3 = (data[12] as u32) | (data[13] as u32) << 8;
            (v0, v1, v2, v3)
        }
        128 => {
            let v0 = data[0] as u32;
            let v1 = data[1] as u32;
            let v2 = data[2] as u32;
            let v3 = (data[3] as u32) | (data[4] as u32) << 8 | (data[5] as u32) << 16;
            (v0, v1, v2, v3)
        }
        129 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8;
            let v1 = data[2] as u32;
            let v2 = data[3] as u32;
            let v3 = (data[4] as u32) | (data[5] as u32) << 8 | (data[6] as u32) << 16;
            (v0, v1, v2, v3)
        }
        130 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8 | (data[2] as u32) << 16;
            let v1 = data[3] as u32;
            let v2 = data[4] as u32;
            let v3 = (data[5] as u32) | (data[6] as u32) << 8 | (data[7] as u32) << 16;
            (v0, v1, v2, v3)
        }
        131 => {
            let v0 = (data[0] as u32)
                | (data[1] as u32) << 8
                | (data[2] as u32) << 16
                | (data[3] as u32) << 24;
            let v1 = data[4] as u32;
            let v2 = data[5] as u32;
            let v3 = (data[6] as u32) | (data[7] as u32) << 8 | (data[8] as u32) << 16;
            (v0, v1, v2, v3)
        }
        132 => {
            let v0 = data[0] as u32;
            let v1 = (data[1] as u32) | (data[2] as u32) << 8;
            let v2 = data[3] as u32;
            let v3 = (data[4] as u32) | (data[5] as u32) << 8 | (data[6] as u32) << 16;
            (v0, v1, v2, v3)
        }
        133 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8;
            let v1 = (data[2] as u32) | (data[3] as u32) << 8;
            let v2 = data[4] as u32;
            let v3 = (data[5] as u32) | (data[6] as u32) << 8 | (data[7] as u32) << 16;
            (v0, v1, v2, v3)
        }
        134 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8 | (data[2] as u32) << 16;
            let v1 = (data[3] as u32) | (data[4] as u32) << 8;
            let v2 = data[5] as u32;
            let v3 = (data[6] as u32) | (data[7] as u32) << 8 | (data[8] as u32) << 16;
            (v0, v1, v2, v3)
        }
        135 => {
            let v0 = (data[0] as u32)
                | (data[1] as u32) << 8
                | (data[2] as u32) << 16
                | (data[3] as u32) << 24;
            let v1 = (data[4] as u32) | (data[5] as u32) << 8;
            let v2 = data[6] as u32;
            let v3 = (data[7] as u32) | (data[8] as u32) << 8 | (data[9] as u32) << 16;
            (v0, v1, v2, v3)
        }
        136 => {
            let v0 = data[0] as u32;
            let v1 = (data[1] as u32) | (data[2] as u32) << 8 | (data[3] as u32) << 16;
            let v2 = data[4] as u32;
            let v3 = (data[5] as u32) | (data[6] as u32) << 8 | (data[7] as u32) << 16;
            (v0, v1, v2, v3)
        }
        137 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8;
            let v1 = (data[2] as u32) | (data[3] as u32) << 8 | (data[4] as u32) << 16;
            let v2 = data[5] as u32;
            let v3 = (data[6] as u32) | (data[7] as u32) << 8 | (data[8] as u32) << 16;
            (v0, v1, v2, v3)
        }
        138 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8 | (data[2] as u32) << 16;
            let v1 = (data[3] as u32) | (data[4] as u32) << 8 | (data[5] as u32) << 16;
            let v2 = data[6] as u32;
            let v3 = (data[7] as u32) | (data[8] as u32) << 8 | (data[9] as u32) << 16;
            (v0, v1, v2, v3)
        }
        139 => {
            let v0 = (data[0] as u32)
                | (data[1] as u32) << 8
                | (data[2] as u32) << 16
                | (data[3] as u32) << 24;
            let v1 = (data[4] as u32) | (data[5] as u32) << 8 | (data[6] as u32) << 16;
            let v2 = data[7] as u32;
            let v3 = (data[8] as u32) | (data[9] as u32) << 8 | (data[10] as u32) << 16;
            (v0, v1, v2, v3)
        }
        140 => {
            let v0 = data[0] as u32;
            let v1 = (data[1] as u32)
                | (data[2] as u32) << 8
                | (data[3] as u32) << 16
                | (data[4] as u32) << 24;
            let v2 = data[5] as u32;
            let v3 = (data[6] as u32) | (data[7] as u32) << 8 | (data[8] as u32) << 16;
            (v0, v1, v2, v3)
        }
        141 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8;
            let v1 = (data[2] as u32)
                | (data[3] as u32) << 8
                | (data[4] as u32) << 16
                | (data[5] as u32) << 24;
            let v2 = data[6] as u32;
            let v3 = (data[7] as u32) | (data[8] as u32) << 8 | (data[9] as u32) << 16;
            (v0, v1, v2, v3)
        }
        142 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8 | (data[2] as u32) << 16;
            let v1 = (data[3] as u32)
                | (data[4] as u32) << 8
                | (data[5] as u32) << 16
                | (data[6] as u32) << 24;
            let v2 = data[7] as u32;
            let v3 = (data[8] as u32) | (data[9] as u32) << 8 | (data[10] as u32) << 16;
            (v0, v1, v2, v3)
        }
        143 => {
            let v0 = (data[0] as u32)
                | (data[1] as u32) << 8
                | (data[2] as u32) << 16
                | (data[3] as u32) << 24;
            let v1 = (data[4] as u32)
                | (data[5] as u32) << 8
                | (data[6] as u32) << 16
                | (data[7] as u32) << 24;
            let v2 = data[8] as u32;
            let v3 = (data[9] as u32) | (data[10] as u32) << 8 | (data[11] as u32) << 16;
            (v0, v1, v2, v3)
        }
        144 => {
            let v0 = data[0] as u32;
            let v1 = data[1] as u32;
            let v2 = (data[2] as u32) | (data[3] as u32) << 8;
            let v3 = (data[4] as u32) | (data[5] as u32) << 8 | (data[6] as u32) << 16;
            (v0, v1, v2, v3)
        }
        145 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8;
            let v1 = data[2] as u32;
            let v2 = (data[3] as u32) | (data[4] as u32) << 8;
            let v3 = (data[5] as u32) | (data[6] as u32) << 8 | (data[7] as u32) << 16;
            (v0, v1, v2, v3)
        }
        146 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8 | (data[2] as u32) << 16;
            let v1 = data[3] as u32;
            let v2 = (data[4] as u32) | (data[5] as u32) << 8;
            let v3 = (data[6] as u32) | (data[7] as u32) << 8 | (data[8] as u32) << 16;
            (v0, v1, v2, v3)
        }
        147 => {
            let v0 = (data[0] as u32)
                | (data[1] as u32) << 8
                | (data[2] as u32) << 16
                | (data[3] as u32) << 24;
            let v1 = data[4] as u32;
            let v2 = (data[5] as u32) | (data[6] as u32) << 8;
            let v3 = (data[7] as u32) | (data[8] as u32) << 8 | (data[9] as u32) << 16;
            (v0, v1, v2, v3)
        }
        148 => {
            let v0 = data[0] as u32;
            let v1 = (data[1] as u32) | (data[2] as u32) << 8;
            let v2 = (data[3] as u32) | (data[4] as u32) << 8;
            let v3 = (data[5] as u32) | (data[6] as u32) << 8 | (data[7] as u32) << 16;
            (v0, v1, v2, v3)
        }
        149 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8;
            let v1 = (data[2] as u32) | (data[3] as u32) << 8;
            let v2 = (data[4] as u32) | (data[5] as u32) << 8;
            let v3 = (data[6] as u32) | (data[7] as u32) << 8 | (data[8] as u32) << 16;
            (v0, v1, v2, v3)
        }
        150 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8 | (data[2] as u32) << 16;
            let v1 = (data[3] as u32) | (data[4] as u32) << 8;
            let v2 = (data[5] as u32) | (data[6] as u32) << 8;
            let v3 = (data[7] as u32) | (data[8] as u32) << 8 | (data[9] as u32) << 16;
            (v0, v1, v2, v3)
        }
        151 => {
            let v0 = (data[0] as u32)
                | (data[1] as u32) << 8
                | (data[2] as u32) << 16
                | (data[3] as u32) << 24;
            let v1 = (data[4] as u32) | (data[5] as u32) << 8;
            let v2 = (data[6] as u32) | (data[7] as u32) << 8;
            let v3 = (data[8] as u32) | (data[9] as u32) << 8 | (data[10] as u32) << 16;
            (v0, v1, v2, v3)
        }
        152 => {
            let v0 = data[0] as u32;
            let v1 = (data[1] as u32) | (data[2] as u32) << 8 | (data[3] as u32) << 16;
            let v2 = (data[4] as u32) | (data[5] as u32) << 8;
            let v3 = (data[6] as u32) | (data[7] as u32) << 8 | (data[8] as u32) << 16;
            (v0, v1, v2, v3)
        }
        153 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8;
            let v1 = (data[2] as u32) | (data[3] as u32) << 8 | (data[4] as u32) << 16;
            let v2 = (data[5] as u32) | (data[6] as u32) << 8;
            let v3 = (data[7] as u32) | (data[8] as u32) << 8 | (data[9] as u32) << 16;
            (v0, v1, v2, v3)
        }
        154 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8 | (data[2] as u32) << 16;
            let v1 = (data[3] as u32) | (data[4] as u32) << 8 | (data[5] as u32) << 16;
            let v2 = (data[6] as u32) | (data[7] as u32) << 8;
            let v3 = (data[8] as u32) | (data[9] as u32) << 8 | (data[10] as u32) << 16;
            (v0, v1, v2, v3)
        }
        155 => {
            let v0 = (data[0] as u32)
                | (data[1] as u32) << 8
                | (data[2] as u32) << 16
                | (data[3] as u32) << 24;
            let v1 = (data[4] as u32) | (data[5] as u32) << 8 | (data[6] as u32) << 16;
            let v2 = (data[7] as u32) | (data[8] as u32) << 8;
            let v3 = (data[9] as u32) | (data[10] as u32) << 8 | (data[11] as u32) << 16;
            (v0, v1, v2, v3)
        }
        156 => {
            let v0 = data[0] as u32;
            let v1 = (data[1] as u32)
                | (data[2] as u32) << 8
                | (data[3] as u32) << 16
                | (data[4] as u32) << 24;
            let v2 = (data[5] as u32) | (data[6] as u32) << 8;
            let v3 = (data[7] as u32) | (data[8] as u32) << 8 | (data[9] as u32) << 16;
            (v0, v1, v2, v3)
        }
        157 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8;
            let v1 = (data[2] as u32)
                | (data[3] as u32) << 8
                | (data[4] as u32) << 16
                | (data[5] as u32) << 24;
            let v2 = (data[6] as u32) | (data[7] as u32) << 8;
            let v3 = (data[8] as u32) | (data[9] as u32) << 8 | (data[10] as u32) << 16;
            (v0, v1, v2, v3)
        }
        158 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8 | (data[2] as u32) << 16;
            let v1 = (data[3] as u32)
                | (data[4] as u32) << 8
                | (data[5] as u32) << 16
                | (data[6] as u32) << 24;
            let v2 = (data[7] as u32) | (data[8] as u32) << 8;
            let v3 = (data[9] as u32) | (data[10] as u32) << 8 | (data[11] as u32) << 16;
            (v0, v1, v2, v3)
        }
        159 => {
            let v0 = (data[0] as u32)
                | (data[1] as u32) << 8
                | (data[2] as u32) << 16
                | (data[3] as u32) << 24;
            let v1 = (data[4] as u32)
                | (data[5] as u32) << 8
                | (data[6] as u32) << 16
                | (data[7] as u32) << 24;
            let v2 = (data[8] as u32) | (data[9] as u32) << 8;
            let v3 = (data[10] as u32) | (data[11] as u32) << 8 | (data[12] as u32) << 16;
            (v0, v1, v2, v3)
        }
        160 => {
            let v0 = data[0] as u32;
            let v1 = data[1] as u32;
            let v2 = (data[2] as u32) | (data[3] as u32) << 8 | (data[4] as u32) << 16;
            let v3 = (data[5] as u32) | (data[6] as u32) << 8 | (data[7] as u32) << 16;
            (v0, v1, v2, v3)
        }
        161 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8;
            let v1 = data[2] as u32;
            let v2 = (data[3] as u32) | (data[4] as u32) << 8 | (data[5] as u32) << 16;
            let v3 = (data[6] as u32) | (data[7] as u32) << 8 | (data[8] as u32) << 16;
            (v0, v1, v2, v3)
        }
        162 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8 | (data[2] as u32) << 16;
            let v1 = data[3] as u32;
            let v2 = (data[4] as u32) | (data[5] as u32) << 8 | (data[6] as u32) << 16;
            let v3 = (data[7] as u32) | (data[8] as u32) << 8 | (data[9] as u32) << 16;
            (v0, v1, v2, v3)
        }
        163 => {
            let v0 = (data[0] as u32)
                | (data[1] as u32) << 8
                | (data[2] as u32) << 16
                | (data[3] as u32) << 24;
            let v1 = data[4] as u32;
            let v2 = (data[5] as u32) | (data[6] as u32) << 8 | (data[7] as u32) << 16;
            let v3 = (data[8] as u32) | (data[9] as u32) << 8 | (data[10] as u32) << 16;
            (v0, v1, v2, v3)
        }
        164 => {
            let v0 = data[0] as u32;
            let v1 = (data[1] as u32) | (data[2] as u32) << 8;
            let v2 = (data[3] as u32) | (data[4] as u32) << 8 | (data[5] as u32) << 16;
            let v3 = (data[6] as u32) | (data[7] as u32) << 8 | (data[8] as u32) << 16;
            (v0, v1, v2, v3)
        }
        165 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8;
            let v1 = (data[2] as u32) | (data[3] as u32) << 8;
            let v2 = (data[4] as u32) | (data[5] as u32) << 8 | (data[6] as u32) << 16;
            let v3 = (data[7] as u32) | (data[8] as u32) << 8 | (data[9] as u32) << 16;
            (v0, v1, v2, v3)
        }
        166 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8 | (data[2] as u32) << 16;
            let v1 = (data[3] as u32) | (data[4] as u32) << 8;
            let v2 = (data[5] as u32) | (data[6] as u32) << 8 | (data[7] as u32) << 16;
            let v3 = (data[8] as u32) | (data[9] as u32) << 8 | (data[10] as u32) << 16;
            (v0, v1, v2, v3)
        }
        167 => {
            let v0 = (data[0] as u32)
                | (data[1] as u32) << 8
                | (data[2] as u32) << 16
                | (data[3] as u32) << 24;
            let v1 = (data[4] as u32) | (data[5] as u32) << 8;
            let v2 = (data[6] as u32) | (data[7] as u32) << 8 | (data[8] as u32) << 16;
            let v3 = (data[9] as u32) | (data[10] as u32) << 8 | (data[11] as u32) << 16;
            (v0, v1, v2, v3)
        }
        168 => {
            let v0 = data[0] as u32;
            let v1 = (data[1] as u32) | (data[2] as u32) << 8 | (data[3] as u32) << 16;
            let v2 = (data[4] as u32) | (data[5] as u32) << 8 | (data[6] as u32) << 16;
            let v3 = (data[7] as u32) | (data[8] as u32) << 8 | (data[9] as u32) << 16;
            (v0, v1, v2, v3)
        }
        169 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8;
            let v1 = (data[2] as u32) | (data[3] as u32) << 8 | (data[4] as u32) << 16;
            let v2 = (data[5] as u32) | (data[6] as u32) << 8 | (data[7] as u32) << 16;
            let v3 = (data[8] as u32) | (data[9] as u32) << 8 | (data[10] as u32) << 16;
            (v0, v1, v2, v3)
        }
        170 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8 | (data[2] as u32) << 16;
            let v1 = (data[3] as u32) | (data[4] as u32) << 8 | (data[5] as u32) << 16;
            let v2 = (data[6] as u32) | (data[7] as u32) << 8 | (data[8] as u32) << 16;
            let v3 = (data[9] as u32) | (data[10] as u32) << 8 | (data[11] as u32) << 16;
            (v0, v1, v2, v3)
        }
        171 => {
            let v0 = (data[0] as u32)
                | (data[1] as u32) << 8
                | (data[2] as u32) << 16
                | (data[3] as u32) << 24;
            let v1 = (data[4] as u32) | (data[5] as u32) << 8 | (data[6] as u32) << 16;
            let v2 = (data[7] as u32) | (data[8] as u32) << 8 | (data[9] as u32) << 16;
            let v3 = (data[10] as u32) | (data[11] as u32) << 8 | (data[12] as u32) << 16;
            (v0, v1, v2, v3)
        }
        172 => {
            let v0 = data[0] as u32;
            let v1 = (data[1] as u32)
                | (data[2] as u32) << 8
                | (data[3] as u32) << 16
                | (data[4] as u32) << 24;
            let v2 = (data[5] as u32) | (data[6] as u32) << 8 | (data[7] as u32) << 16;
            let v3 = (data[8] as u32) | (data[9] as u32) << 8 | (data[10] as u32) << 16;
            (v0, v1, v2, v3)
        }
        173 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8;
            let v1 = (data[2] as u32)
                | (data[3] as u32) << 8
                | (data[4] as u32) << 16
                | (data[5] as u32) << 24;
            let v2 = (data[6] as u32) | (data[7] as u32) << 8 | (data[8] as u32) << 16;
            let v3 = (data[9] as u32) | (data[10] as u32) << 8 | (data[11] as u32) << 16;
            (v0, v1, v2, v3)
        }
        174 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8 | (data[2] as u32) << 16;
            let v1 = (data[3] as u32)
                | (data[4] as u32) << 8
                | (data[5] as u32) << 16
                | (data[6] as u32) << 24;
            let v2 = (data[7] as u32) | (data[8] as u32) << 8 | (data[9] as u32) << 16;
            let v3 = (data[10] as u32) | (data[11] as u32) << 8 | (data[12] as u32) << 16;
            (v0, v1, v2, v3)
        }
        175 => {
            let v0 = (data[0] as u32)
                | (data[1] as u32) << 8
                | (data[2] as u32) << 16
                | (data[3] as u32) << 24;
            let v1 = (data[4] as u32)
                | (data[5] as u32) << 8
                | (data[6] as u32) << 16
                | (data[7] as u32) << 24;
            let v2 = (data[8] as u32) | (data[9] as u32) << 8 | (data[10] as u32) << 16;
            let v3 = (data[11] as u32) | (data[12] as u32) << 8 | (data[13] as u32) << 16;
            (v0, v1, v2, v3)
        }
        176 => {
            let v0 = data[0] as u32;
            let v1 = data[1] as u32;
            let v2 = (data[2] as u32)
                | (data[3] as u32) << 8
                | (data[4] as u32) << 16
                | (data[5] as u32) << 24;
            let v3 = (data[6] as u32) | (data[7] as u32) << 8 | (data[8] as u32) << 16;
            (v0, v1, v2, v3)
        }
        177 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8;
            let v1 = data[2] as u32;
            let v2 = (data[3] as u32)
                | (data[4] as u32) << 8
                | (data[5] as u32) << 16
                | (data[6] as u32) << 24;
            let v3 = (data[7] as u32) | (data[8] as u32) << 8 | (data[9] as u32) << 16;
            (v0, v1, v2, v3)
        }
        178 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8 | (data[2] as u32) << 16;
            let v1 = data[3] as u32;
            let v2 = (data[4] as u32)
                | (data[5] as u32) << 8
                | (data[6] as u32) << 16
                | (data[7] as u32) << 24;
            let v3 = (data[8] as u32) | (data[9] as u32) << 8 | (data[10] as u32) << 16;
            (v0, v1, v2, v3)
        }
        179 => {
            let v0 = (data[0] as u32)
                | (data[1] as u32) << 8
                | (data[2] as u32) << 16
                | (data[3] as u32) << 24;
            let v1 = data[4] as u32;
            let v2 = (data[5] as u32)
                | (data[6] as u32) << 8
                | (data[7] as u32) << 16
                | (data[8] as u32) << 24;
            let v3 = (data[9] as u32) | (data[10] as u32) << 8 | (data[11] as u32) << 16;
            (v0, v1, v2, v3)
        }
        180 => {
            let v0 = data[0] as u32;
            let v1 = (data[1] as u32) | (data[2] as u32) << 8;
            let v2 = (data[3] as u32)
                | (data[4] as u32) << 8
                | (data[5] as u32) << 16
                | (data[6] as u32) << 24;
            let v3 = (data[7] as u32) | (data[8] as u32) << 8 | (data[9] as u32) << 16;
            (v0, v1, v2, v3)
        }
        181 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8;
            let v1 = (data[2] as u32) | (data[3] as u32) << 8;
            let v2 = (data[4] as u32)
                | (data[5] as u32) << 8
                | (data[6] as u32) << 16
                | (data[7] as u32) << 24;
            let v3 = (data[8] as u32) | (data[9] as u32) << 8 | (data[10] as u32) << 16;
            (v0, v1, v2, v3)
        }
        182 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8 | (data[2] as u32) << 16;
            let v1 = (data[3] as u32) | (data[4] as u32) << 8;
            let v2 = (data[5] as u32)
                | (data[6] as u32) << 8
                | (data[7] as u32) << 16
                | (data[8] as u32) << 24;
            let v3 = (data[9] as u32) | (data[10] as u32) << 8 | (data[11] as u32) << 16;
            (v0, v1, v2, v3)
        }
        183 => {
            let v0 = (data[0] as u32)
                | (data[1] as u32) << 8
                | (data[2] as u32) << 16
                | (data[3] as u32) << 24;
            let v1 = (data[4] as u32) | (data[5] as u32) << 8;
            let v2 = (data[6] as u32)
                | (data[7] as u32) << 8
                | (data[8] as u32) << 16
                | (data[9] as u32) << 24;
            let v3 = (data[10] as u32) | (data[11] as u32) << 8 | (data[12] as u32) << 16;
            (v0, v1, v2, v3)
        }
        184 => {
            let v0 = data[0] as u32;
            let v1 = (data[1] as u32) | (data[2] as u32) << 8 | (data[3] as u32) << 16;
            let v2 = (data[4] as u32)
                | (data[5] as u32) << 8
                | (data[6] as u32) << 16
                | (data[7] as u32) << 24;
            let v3 = (data[8] as u32) | (data[9] as u32) << 8 | (data[10] as u32) << 16;
            (v0, v1, v2, v3)
        }
        185 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8;
            let v1 = (data[2] as u32) | (data[3] as u32) << 8 | (data[4] as u32) << 16;
            let v2 = (data[5] as u32)
                | (data[6] as u32) << 8
                | (data[7] as u32) << 16
                | (data[8] as u32) << 24;
            let v3 = (data[9] as u32) | (data[10] as u32) << 8 | (data[11] as u32) << 16;
            (v0, v1, v2, v3)
        }
        186 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8 | (data[2] as u32) << 16;
            let v1 = (data[3] as u32) | (data[4] as u32) << 8 | (data[5] as u32) << 16;
            let v2 = (data[6] as u32)
                | (data[7] as u32) << 8
                | (data[8] as u32) << 16
                | (data[9] as u32) << 24;
            let v3 = (data[10] as u32) | (data[11] as u32) << 8 | (data[12] as u32) << 16;
            (v0, v1, v2, v3)
        }
        187 => {
            let v0 = (data[0] as u32)
                | (data[1] as u32) << 8
                | (data[2] as u32) << 16
                | (data[3] as u32) << 24;
            let v1 = (data[4] as u32) | (data[5] as u32) << 8 | (data[6] as u32) << 16;
            let v2 = (data[7] as u32)
                | (data[8] as u32) << 8
                | (data[9] as u32) << 16
                | (data[10] as u32) << 24;
            let v3 = (data[11] as u32) | (data[12] as u32) << 8 | (data[13] as u32) << 16;
            (v0, v1, v2, v3)
        }
        188 => {
            let v0 = data[0] as u32;
            let v1 = (data[1] as u32)
                | (data[2] as u32) << 8
                | (data[3] as u32) << 16
                | (data[4] as u32) << 24;
            let v2 = (data[5] as u32)
                | (data[6] as u32) << 8
                | (data[7] as u32) << 16
                | (data[8] as u32) << 24;
            let v3 = (data[9] as u32) | (data[10] as u32) << 8 | (data[11] as u32) << 16;
            (v0, v1, v2, v3)
        }
        189 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8;
            let v1 = (data[2] as u32)
                | (data[3] as u32) << 8
                | (data[4] as u32) << 16
                | (data[5] as u32) << 24;
            let v2 = (data[6] as u32)
                | (data[7] as u32) << 8
                | (data[8] as u32) << 16
                | (data[9] as u32) << 24;
            let v3 = (data[10] as u32) | (data[11] as u32) << 8 | (data[12] as u32) << 16;
            (v0, v1, v2, v3)
        }
        190 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8 | (data[2] as u32) << 16;
            let v1 = (data[3] as u32)
                | (data[4] as u32) << 8
                | (data[5] as u32) << 16
                | (data[6] as u32) << 24;
            let v2 = (data[7] as u32)
                | (data[8] as u32) << 8
                | (data[9] as u32) << 16
                | (data[10] as u32) << 24;
            let v3 = (data[11] as u32) | (data[12] as u32) << 8 | (data[13] as u32) << 16;
            (v0, v1, v2, v3)
        }
        191 => {
            let v0 = (data[0] as u32)
                | (data[1] as u32) << 8
                | (data[2] as u32) << 16
                | (data[3] as u32) << 24;
            let v1 = (data[4] as u32)
                | (data[5] as u32) << 8
                | (data[6] as u32) << 16
                | (data[7] as u32) << 24;
            let v2 = (data[8] as u32)
                | (data[9] as u32) << 8
                | (data[10] as u32) << 16
                | (data[11] as u32) << 24;
            let v3 = (data[12] as u32) | (data[13] as u32) << 8 | (data[14] as u32) << 16;
            (v0, v1, v2, v3)
        }
        192 => {
            let v0 = data[0] as u32;
            let v1 = data[1] as u32;
            let v2 = data[2] as u32;
            let v3 = (data[3] as u32)
                | (data[4] as u32) << 8
                | (data[5] as u32) << 16
                | (data[6] as u32) << 24;
            (v0, v1, v2, v3)
        }
        193 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8;
            let v1 = data[2] as u32;
            let v2 = data[3] as u32;
            let v3 = (data[4] as u32)
                | (data[5] as u32) << 8
                | (data[6] as u32) << 16
                | (data[7] as u32) << 24;
            (v0, v1, v2, v3)
        }
        194 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8 | (data[2] as u32) << 16;
            let v1 = data[3] as u32;
            let v2 = data[4] as u32;
            let v3 = (data[5] as u32)
                | (data[6] as u32) << 8
                | (data[7] as u32) << 16
                | (data[8] as u32) << 24;
            (v0, v1, v2, v3)
        }
        195 => {
            let v0 = (data[0] as u32)
                | (data[1] as u32) << 8
                | (data[2] as u32) << 16
                | (data[3] as u32) << 24;
            let v1 = data[4] as u32;
            let v2 = data[5] as u32;
            let v3 = (data[6] as u32)
                | (data[7] as u32) << 8
                | (data[8] as u32) << 16
                | (data[9] as u32) << 24;
            (v0, v1, v2, v3)
        }
        196 => {
            let v0 = data[0] as u32;
            let v1 = (data[1] as u32) | (data[2] as u32) << 8;
            let v2 = data[3] as u32;
            let v3 = (data[4] as u32)
                | (data[5] as u32) << 8
                | (data[6] as u32) << 16
                | (data[7] as u32) << 24;
            (v0, v1, v2, v3)
        }
        197 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8;
            let v1 = (data[2] as u32) | (data[3] as u32) << 8;
            let v2 = data[4] as u32;
            let v3 = (data[5] as u32)
                | (data[6] as u32) << 8
                | (data[7] as u32) << 16
                | (data[8] as u32) << 24;
            (v0, v1, v2, v3)
        }
        198 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8 | (data[2] as u32) << 16;
            let v1 = (data[3] as u32) | (data[4] as u32) << 8;
            let v2 = data[5] as u32;
            let v3 = (data[6] as u32)
                | (data[7] as u32) << 8
                | (data[8] as u32) << 16
                | (data[9] as u32) << 24;
            (v0, v1, v2, v3)
        }
        199 => {
            let v0 = (data[0] as u32)
                | (data[1] as u32) << 8
                | (data[2] as u32) << 16
                | (data[3] as u32) << 24;
            let v1 = (data[4] as u32) | (data[5] as u32) << 8;
            let v2 = data[6] as u32;
            let v3 = (data[7] as u32)
                | (data[8] as u32) << 8
                | (data[9] as u32) << 16
                | (data[10] as u32) << 24;
            (v0, v1, v2, v3)
        }
        200 => {
            let v0 = data[0] as u32;
            let v1 = (data[1] as u32) | (data[2] as u32) << 8 | (data[3] as u32) << 16;
            let v2 = data[4] as u32;
            let v3 = (data[5] as u32)
                | (data[6] as u32) << 8
                | (data[7] as u32) << 16
                | (data[8] as u32) << 24;
            (v0, v1, v2, v3)
        }
        201 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8;
            let v1 = (data[2] as u32) | (data[3] as u32) << 8 | (data[4] as u32) << 16;
            let v2 = data[5] as u32;
            let v3 = (data[6] as u32)
                | (data[7] as u32) << 8
                | (data[8] as u32) << 16
                | (data[9] as u32) << 24;
            (v0, v1, v2, v3)
        }
        202 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8 | (data[2] as u32) << 16;
            let v1 = (data[3] as u32) | (data[4] as u32) << 8 | (data[5] as u32) << 16;
            let v2 = data[6] as u32;
            let v3 = (data[7] as u32)
                | (data[8] as u32) << 8
                | (data[9] as u32) << 16
                | (data[10] as u32) << 24;
            (v0, v1, v2, v3)
        }
        203 => {
            let v0 = (data[0] as u32)
                | (data[1] as u32) << 8
                | (data[2] as u32) << 16
                | (data[3] as u32) << 24;
            let v1 = (data[4] as u32) | (data[5] as u32) << 8 | (data[6] as u32) << 16;
            let v2 = data[7] as u32;
            let v3 = (data[8] as u32)
                | (data[9] as u32) << 8
                | (data[10] as u32) << 16
                | (data[11] as u32) << 24;
            (v0, v1, v2, v3)
        }
        204 => {
            let v0 = data[0] as u32;
            let v1 = (data[1] as u32)
                | (data[2] as u32) << 8
                | (data[3] as u32) << 16
                | (data[4] as u32) << 24;
            let v2 = data[5] as u32;
            let v3 = (data[6] as u32)
                | (data[7] as u32) << 8
                | (data[8] as u32) << 16
                | (data[9] as u32) << 24;
            (v0, v1, v2, v3)
        }
        205 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8;
            let v1 = (data[2] as u32)
                | (data[3] as u32) << 8
                | (data[4] as u32) << 16
                | (data[5] as u32) << 24;
            let v2 = data[6] as u32;
            let v3 = (data[7] as u32)
                | (data[8] as u32) << 8
                | (data[9] as u32) << 16
                | (data[10] as u32) << 24;
            (v0, v1, v2, v3)
        }
        206 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8 | (data[2] as u32) << 16;
            let v1 = (data[3] as u32)
                | (data[4] as u32) << 8
                | (data[5] as u32) << 16
                | (data[6] as u32) << 24;
            let v2 = data[7] as u32;
            let v3 = (data[8] as u32)
                | (data[9] as u32) << 8
                | (data[10] as u32) << 16
                | (data[11] as u32) << 24;
            (v0, v1, v2, v3)
        }
        207 => {
            let v0 = (data[0] as u32)
                | (data[1] as u32) << 8
                | (data[2] as u32) << 16
                | (data[3] as u32) << 24;
            let v1 = (data[4] as u32)
                | (data[5] as u32) << 8
                | (data[6] as u32) << 16
                | (data[7] as u32) << 24;
            let v2 = data[8] as u32;
            let v3 = (data[9] as u32)
                | (data[10] as u32) << 8
                | (data[11] as u32) << 16
                | (data[12] as u32) << 24;
            (v0, v1, v2, v3)
        }
        208 => {
            let v0 = data[0] as u32;
            let v1 = data[1] as u32;
            let v2 = (data[2] as u32) | (data[3] as u32) << 8;
            let v3 = (data[4] as u32)
                | (data[5] as u32) << 8
                | (data[6] as u32) << 16
                | (data[7] as u32) << 24;
            (v0, v1, v2, v3)
        }
        209 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8;
            let v1 = data[2] as u32;
            let v2 = (data[3] as u32) | (data[4] as u32) << 8;
            let v3 = (data[5] as u32)
                | (data[6] as u32) << 8
                | (data[7] as u32) << 16
                | (data[8] as u32) << 24;
            (v0, v1, v2, v3)
        }
        210 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8 | (data[2] as u32) << 16;
            let v1 = data[3] as u32;
            let v2 = (data[4] as u32) | (data[5] as u32) << 8;
            let v3 = (data[6] as u32)
                | (data[7] as u32) << 8
                | (data[8] as u32) << 16
                | (data[9] as u32) << 24;
            (v0, v1, v2, v3)
        }
        211 => {
            let v0 = (data[0] as u32)
                | (data[1] as u32) << 8
                | (data[2] as u32) << 16
                | (data[3] as u32) << 24;
            let v1 = data[4] as u32;
            let v2 = (data[5] as u32) | (data[6] as u32) << 8;
            let v3 = (data[7] as u32)
                | (data[8] as u32) << 8
                | (data[9] as u32) << 16
                | (data[10] as u32) << 24;
            (v0, v1, v2, v3)
        }
        212 => {
            let v0 = data[0] as u32;
            let v1 = (data[1] as u32) | (data[2] as u32) << 8;
            let v2 = (data[3] as u32) | (data[4] as u32) << 8;
            let v3 = (data[5] as u32)
                | (data[6] as u32) << 8
                | (data[7] as u32) << 16
                | (data[8] as u32) << 24;
            (v0, v1, v2, v3)
        }
        213 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8;
            let v1 = (data[2] as u32) | (data[3] as u32) << 8;
            let v2 = (data[4] as u32) | (data[5] as u32) << 8;
            let v3 = (data[6] as u32)
                | (data[7] as u32) << 8
                | (data[8] as u32) << 16
                | (data[9] as u32) << 24;
            (v0, v1, v2, v3)
        }
        214 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8 | (data[2] as u32) << 16;
            let v1 = (data[3] as u32) | (data[4] as u32) << 8;
            let v2 = (data[5] as u32) | (data[6] as u32) << 8;
            let v3 = (data[7] as u32)
                | (data[8] as u32) << 8
                | (data[9] as u32) << 16
                | (data[10] as u32) << 24;
            (v0, v1, v2, v3)
        }
        215 => {
            let v0 = (data[0] as u32)
                | (data[1] as u32) << 8
                | (data[2] as u32) << 16
                | (data[3] as u32) << 24;
            let v1 = (data[4] as u32) | (data[5] as u32) << 8;
            let v2 = (data[6] as u32) | (data[7] as u32) << 8;
            let v3 = (data[8] as u32)
                | (data[9] as u32) << 8
                | (data[10] as u32) << 16
                | (data[11] as u32) << 24;
            (v0, v1, v2, v3)
        }
        216 => {
            let v0 = data[0] as u32;
            let v1 = (data[1] as u32) | (data[2] as u32) << 8 | (data[3] as u32) << 16;
            let v2 = (data[4] as u32) | (data[5] as u32) << 8;
            let v3 = (data[6] as u32)
                | (data[7] as u32) << 8
                | (data[8] as u32) << 16
                | (data[9] as u32) << 24;
            (v0, v1, v2, v3)
        }
        217 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8;
            let v1 = (data[2] as u32) | (data[3] as u32) << 8 | (data[4] as u32) << 16;
            let v2 = (data[5] as u32) | (data[6] as u32) << 8;
            let v3 = (data[7] as u32)
                | (data[8] as u32) << 8
                | (data[9] as u32) << 16
                | (data[10] as u32) << 24;
            (v0, v1, v2, v3)
        }
        218 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8 | (data[2] as u32) << 16;
            let v1 = (data[3] as u32) | (data[4] as u32) << 8 | (data[5] as u32) << 16;
            let v2 = (data[6] as u32) | (data[7] as u32) << 8;
            let v3 = (data[8] as u32)
                | (data[9] as u32) << 8
                | (data[10] as u32) << 16
                | (data[11] as u32) << 24;
            (v0, v1, v2, v3)
        }
        219 => {
            let v0 = (data[0] as u32)
                | (data[1] as u32) << 8
                | (data[2] as u32) << 16
                | (data[3] as u32) << 24;
            let v1 = (data[4] as u32) | (data[5] as u32) << 8 | (data[6] as u32) << 16;
            let v2 = (data[7] as u32) | (data[8] as u32) << 8;
            let v3 = (data[9] as u32)
                | (data[10] as u32) << 8
                | (data[11] as u32) << 16
                | (data[12] as u32) << 24;
            (v0, v1, v2, v3)
        }
        220 => {
            let v0 = data[0] as u32;
            let v1 = (data[1] as u32)
                | (data[2] as u32) << 8
                | (data[3] as u32) << 16
                | (data[4] as u32) << 24;
            let v2 = (data[5] as u32) | (data[6] as u32) << 8;
            let v3 = (data[7] as u32)
                | (data[8] as u32) << 8
                | (data[9] as u32) << 16
                | (data[10] as u32) << 24;
            (v0, v1, v2, v3)
        }
        221 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8;
            let v1 = (data[2] as u32)
                | (data[3] as u32) << 8
                | (data[4] as u32) << 16
                | (data[5] as u32) << 24;
            let v2 = (data[6] as u32) | (data[7] as u32) << 8;
            let v3 = (data[8] as u32)
                | (data[9] as u32) << 8
                | (data[10] as u32) << 16
                | (data[11] as u32) << 24;
            (v0, v1, v2, v3)
        }
        222 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8 | (data[2] as u32) << 16;
            let v1 = (data[3] as u32)
                | (data[4] as u32) << 8
                | (data[5] as u32) << 16
                | (data[6] as u32) << 24;
            let v2 = (data[7] as u32) | (data[8] as u32) << 8;
            let v3 = (data[9] as u32)
                | (data[10] as u32) << 8
                | (data[11] as u32) << 16
                | (data[12] as u32) << 24;
            (v0, v1, v2, v3)
        }
        223 => {
            let v0 = (data[0] as u32)
                | (data[1] as u32) << 8
                | (data[2] as u32) << 16
                | (data[3] as u32) << 24;
            let v1 = (data[4] as u32)
                | (data[5] as u32) << 8
                | (data[6] as u32) << 16
                | (data[7] as u32) << 24;
            let v2 = (data[8] as u32) | (data[9] as u32) << 8;
            let v3 = (data[10] as u32)
                | (data[11] as u32) << 8
                | (data[12] as u32) << 16
                | (data[13] as u32) << 24;
            (v0, v1, v2, v3)
        }
        224 => {
            let v0 = data[0] as u32;
            let v1 = data[1] as u32;
            let v2 = (data[2] as u32) | (data[3] as u32) << 8 | (data[4] as u32) << 16;
            let v3 = (data[5] as u32)
                | (data[6] as u32) << 8
                | (data[7] as u32) << 16
                | (data[8] as u32) << 24;
            (v0, v1, v2, v3)
        }
        225 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8;
            let v1 = data[2] as u32;
            let v2 = (data[3] as u32) | (data[4] as u32) << 8 | (data[5] as u32) << 16;
            let v3 = (data[6] as u32)
                | (data[7] as u32) << 8
                | (data[8] as u32) << 16
                | (data[9] as u32) << 24;
            (v0, v1, v2, v3)
        }
        226 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8 | (data[2] as u32) << 16;
            let v1 = data[3] as u32;
            let v2 = (data[4] as u32) | (data[5] as u32) << 8 | (data[6] as u32) << 16;
            let v3 = (data[7] as u32)
                | (data[8] as u32) << 8
                | (data[9] as u32) << 16
                | (data[10] as u32) << 24;
            (v0, v1, v2, v3)
        }
        227 => {
            let v0 = (data[0] as u32)
                | (data[1] as u32) << 8
                | (data[2] as u32) << 16
                | (data[3] as u32) << 24;
            let v1 = data[4] as u32;
            let v2 = (data[5] as u32) | (data[6] as u32) << 8 | (data[7] as u32) << 16;
            let v3 = (data[8] as u32)
                | (data[9] as u32) << 8
                | (data[10] as u32) << 16
                | (data[11] as u32) << 24;
            (v0, v1, v2, v3)
        }
        228 => {
            let v0 = data[0] as u32;
            let v1 = (data[1] as u32) | (data[2] as u32) << 8;
            let v2 = (data[3] as u32) | (data[4] as u32) << 8 | (data[5] as u32) << 16;
            let v3 = (data[6] as u32)
                | (data[7] as u32) << 8
                | (data[8] as u32) << 16
                | (data[9] as u32) << 24;
            (v0, v1, v2, v3)
        }
        229 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8;
            let v1 = (data[2] as u32) | (data[3] as u32) << 8;
            let v2 = (data[4] as u32) | (data[5] as u32) << 8 | (data[6] as u32) << 16;
            let v3 = (data[7] as u32)
                | (data[8] as u32) << 8
                | (data[9] as u32) << 16
                | (data[10] as u32) << 24;
            (v0, v1, v2, v3)
        }
        230 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8 | (data[2] as u32) << 16;
            let v1 = (data[3] as u32) | (data[4] as u32) << 8;
            let v2 = (data[5] as u32) | (data[6] as u32) << 8 | (data[7] as u32) << 16;
            let v3 = (data[8] as u32)
                | (data[9] as u32) << 8
                | (data[10] as u32) << 16
                | (data[11] as u32) << 24;
            (v0, v1, v2, v3)
        }
        231 => {
            let v0 = (data[0] as u32)
                | (data[1] as u32) << 8
                | (data[2] as u32) << 16
                | (data[3] as u32) << 24;
            let v1 = (data[4] as u32) | (data[5] as u32) << 8;
            let v2 = (data[6] as u32) | (data[7] as u32) << 8 | (data[8] as u32) << 16;
            let v3 = (data[9] as u32)
                | (data[10] as u32) << 8
                | (data[11] as u32) << 16
                | (data[12] as u32) << 24;
            (v0, v1, v2, v3)
        }
        232 => {
            let v0 = data[0] as u32;
            let v1 = (data[1] as u32) | (data[2] as u32) << 8 | (data[3] as u32) << 16;
            let v2 = (data[4] as u32) | (data[5] as u32) << 8 | (data[6] as u32) << 16;
            let v3 = (data[7] as u32)
                | (data[8] as u32) << 8
                | (data[9] as u32) << 16
                | (data[10] as u32) << 24;
            (v0, v1, v2, v3)
        }
        233 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8;
            let v1 = (data[2] as u32) | (data[3] as u32) << 8 | (data[4] as u32) << 16;
            let v2 = (data[5] as u32) | (data[6] as u32) << 8 | (data[7] as u32) << 16;
            let v3 = (data[8] as u32)
                | (data[9] as u32) << 8
                | (data[10] as u32) << 16
                | (data[11] as u32) << 24;
            (v0, v1, v2, v3)
        }
        234 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8 | (data[2] as u32) << 16;
            let v1 = (data[3] as u32) | (data[4] as u32) << 8 | (data[5] as u32) << 16;
            let v2 = (data[6] as u32) | (data[7] as u32) << 8 | (data[8] as u32) << 16;
            let v3 = (data[9] as u32)
                | (data[10] as u32) << 8
                | (data[11] as u32) << 16
                | (data[12] as u32) << 24;
            (v0, v1, v2, v3)
        }
        235 => {
            let v0 = (data[0] as u32)
                | (data[1] as u32) << 8
                | (data[2] as u32) << 16
                | (data[3] as u32) << 24;
            let v1 = (data[4] as u32) | (data[5] as u32) << 8 | (data[6] as u32) << 16;
            let v2 = (data[7] as u32) | (data[8] as u32) << 8 | (data[9] as u32) << 16;
            let v3 = (data[10] as u32)
                | (data[11] as u32) << 8
                | (data[12] as u32) << 16
                | (data[13] as u32) << 24;
            (v0, v1, v2, v3)
        }
        236 => {
            let v0 = data[0] as u32;
            let v1 = (data[1] as u32)
                | (data[2] as u32) << 8
                | (data[3] as u32) << 16
                | (data[4] as u32) << 24;
            let v2 = (data[5] as u32) | (data[6] as u32) << 8 | (data[7] as u32) << 16;
            let v3 = (data[8] as u32)
                | (data[9] as u32) << 8
                | (data[10] as u32) << 16
                | (data[11] as u32) << 24;
            (v0, v1, v2, v3)
        }
        237 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8;
            let v1 = (data[2] as u32)
                | (data[3] as u32) << 8
                | (data[4] as u32) << 16
                | (data[5] as u32) << 24;
            let v2 = (data[6] as u32) | (data[7] as u32) << 8 | (data[8] as u32) << 16;
            let v3 = (data[9] as u32)
                | (data[10] as u32) << 8
                | (data[11] as u32) << 16
                | (data[12] as u32) << 24;
            (v0, v1, v2, v3)
        }
        238 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8 | (data[2] as u32) << 16;
            let v1 = (data[3] as u32)
                | (data[4] as u32) << 8
                | (data[5] as u32) << 16
                | (data[6] as u32) << 24;
            let v2 = (data[7] as u32) | (data[8] as u32) << 8 | (data[9] as u32) << 16;
            let v3 = (data[10] as u32)
                | (data[11] as u32) << 8
                | (data[12] as u32) << 16
                | (data[13] as u32) << 24;
            (v0, v1, v2, v3)
        }
        239 => {
            let v0 = (data[0] as u32)
                | (data[1] as u32) << 8
                | (data[2] as u32) << 16
                | (data[3] as u32) << 24;
            let v1 = (data[4] as u32)
                | (data[5] as u32) << 8
                | (data[6] as u32) << 16
                | (data[7] as u32) << 24;
            let v2 = (data[8] as u32) | (data[9] as u32) << 8 | (data[10] as u32) << 16;
            let v3 = (data[11] as u32)
                | (data[12] as u32) << 8
                | (data[13] as u32) << 16
                | (data[14] as u32) << 24;
            (v0, v1, v2, v3)
        }
        240 => {
            let v0 = data[0] as u32;
            let v1 = data[1] as u32;
            let v2 = (data[2] as u32)
                | (data[3] as u32) << 8
                | (data[4] as u32) << 16
                | (data[5] as u32) << 24;
            let v3 = (data[6] as u32)
                | (data[7] as u32) << 8
                | (data[8] as u32) << 16
                | (data[9] as u32) << 24;
            (v0, v1, v2, v3)
        }
        241 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8;
            let v1 = data[2] as u32;
            let v2 = (data[3] as u32)
                | (data[4] as u32) << 8
                | (data[5] as u32) << 16
                | (data[6] as u32) << 24;
            let v3 = (data[7] as u32)
                | (data[8] as u32) << 8
                | (data[9] as u32) << 16
                | (data[10] as u32) << 24;
            (v0, v1, v2, v3)
        }
        242 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8 | (data[2] as u32) << 16;
            let v1 = data[3] as u32;
            let v2 = (data[4] as u32)
                | (data[5] as u32) << 8
                | (data[6] as u32) << 16
                | (data[7] as u32) << 24;
            let v3 = (data[8] as u32)
                | (data[9] as u32) << 8
                | (data[10] as u32) << 16
                | (data[11] as u32) << 24;
            (v0, v1, v2, v3)
        }
        243 => {
            let v0 = (data[0] as u32)
                | (data[1] as u32) << 8
                | (data[2] as u32) << 16
                | (data[3] as u32) << 24;
            let v1 = data[4] as u32;
            let v2 = (data[5] as u32)
                | (data[6] as u32) << 8
                | (data[7] as u32) << 16
                | (data[8] as u32) << 24;
            let v3 = (data[9] as u32)
                | (data[10] as u32) << 8
                | (data[11] as u32) << 16
                | (data[12] as u32) << 24;
            (v0, v1, v2, v3)
        }
        244 => {
            let v0 = data[0] as u32;
            let v1 = (data[1] as u32) | (data[2] as u32) << 8;
            let v2 = (data[3] as u32)
                | (data[4] as u32) << 8
                | (data[5] as u32) << 16
                | (data[6] as u32) << 24;
            let v3 = (data[7] as u32)
                | (data[8] as u32) << 8
                | (data[9] as u32) << 16
                | (data[10] as u32) << 24;
            (v0, v1, v2, v3)
        }
        245 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8;
            let v1 = (data[2] as u32) | (data[3] as u32) << 8;
            let v2 = (data[4] as u32)
                | (data[5] as u32) << 8
                | (data[6] as u32) << 16
                | (data[7] as u32) << 24;
            let v3 = (data[8] as u32)
                | (data[9] as u32) << 8
                | (data[10] as u32) << 16
                | (data[11] as u32) << 24;
            (v0, v1, v2, v3)
        }
        246 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8 | (data[2] as u32) << 16;
            let v1 = (data[3] as u32) | (data[4] as u32) << 8;
            let v2 = (data[5] as u32)
                | (data[6] as u32) << 8
                | (data[7] as u32) << 16
                | (data[8] as u32) << 24;
            let v3 = (data[9] as u32)
                | (data[10] as u32) << 8
                | (data[11] as u32) << 16
                | (data[12] as u32) << 24;
            (v0, v1, v2, v3)
        }
        247 => {
            let v0 = (data[0] as u32)
                | (data[1] as u32) << 8
                | (data[2] as u32) << 16
                | (data[3] as u32) << 24;
            let v1 = (data[4] as u32) | (data[5] as u32) << 8;
            let v2 = (data[6] as u32)
                | (data[7] as u32) << 8
                | (data[8] as u32) << 16
                | (data[9] as u32) << 24;
            let v3 = (data[10] as u32)
                | (data[11] as u32) << 8
                | (data[12] as u32) << 16
                | (data[13] as u32) << 24;
            (v0, v1, v2, v3)
        }
        248 => {
            let v0 = data[0] as u32;
            let v1 = (data[1] as u32) | (data[2] as u32) << 8 | (data[3] as u32) << 16;
            let v2 = (data[4] as u32)
                | (data[5] as u32) << 8
                | (data[6] as u32) << 16
                | (data[7] as u32) << 24;
            let v3 = (data[8] as u32)
                | (data[9] as u32) << 8
                | (data[10] as u32) << 16
                | (data[11] as u32) << 24;
            (v0, v1, v2, v3)
        }
        249 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8;
            let v1 = (data[2] as u32) | (data[3] as u32) << 8 | (data[4] as u32) << 16;
            let v2 = (data[5] as u32)
                | (data[6] as u32) << 8
                | (data[7] as u32) << 16
                | (data[8] as u32) << 24;
            let v3 = (data[9] as u32)
                | (data[10] as u32) << 8
                | (data[11] as u32) << 16
                | (data[12] as u32) << 24;
            (v0, v1, v2, v3)
        }
        250 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8 | (data[2] as u32) << 16;
            let v1 = (data[3] as u32) | (data[4] as u32) << 8 | (data[5] as u32) << 16;
            let v2 = (data[6] as u32)
                | (data[7] as u32) << 8
                | (data[8] as u32) << 16
                | (data[9] as u32) << 24;
            let v3 = (data[10] as u32)
                | (data[11] as u32) << 8
                | (data[12] as u32) << 16
                | (data[13] as u32) << 24;
            (v0, v1, v2, v3)
        }
        251 => {
            let v0 = (data[0] as u32)
                | (data[1] as u32) << 8
                | (data[2] as u32) << 16
                | (data[3] as u32) << 24;
            let v1 = (data[4] as u32) | (data[5] as u32) << 8 | (data[6] as u32) << 16;
            let v2 = (data[7] as u32)
                | (data[8] as u32) << 8
                | (data[9] as u32) << 16
                | (data[10] as u32) << 24;
            let v3 = (data[11] as u32)
                | (data[12] as u32) << 8
                | (data[13] as u32) << 16
                | (data[14] as u32) << 24;
            (v0, v1, v2, v3)
        }
        252 => {
            let v0 = data[0] as u32;
            let v1 = (data[1] as u32)
                | (data[2] as u32) << 8
                | (data[3] as u32) << 16
                | (data[4] as u32) << 24;
            let v2 = (data[5] as u32)
                | (data[6] as u32) << 8
                | (data[7] as u32) << 16
                | (data[8] as u32) << 24;
            let v3 = (data[9] as u32)
                | (data[10] as u32) << 8
                | (data[11] as u32) << 16
                | (data[12] as u32) << 24;
            (v0, v1, v2, v3)
        }
        253 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8;
            let v1 = (data[2] as u32)
                | (data[3] as u32) << 8
                | (data[4] as u32) << 16
                | (data[5] as u32) << 24;
            let v2 = (data[6] as u32)
                | (data[7] as u32) << 8
                | (data[8] as u32) << 16
                | (data[9] as u32) << 24;
            let v3 = (data[10] as u32)
                | (data[11] as u32) << 8
                | (data[12] as u32) << 16
                | (data[13] as u32) << 24;
            (v0, v1, v2, v3)
        }
        254 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8 | (data[2] as u32) << 16;
            let v1 = (data[3] as u32)
                | (data[4] as u32) << 8
                | (data[5] as u32) << 16
                | (data[6] as u32) << 24;
            let v2 = (data[7] as u32)
                | (data[8] as u32) << 8
                | (data[9] as u32) << 16
                | (data[10] as u32) << 24;
            let v3 = (data[11] as u32)
                | (data[12] as u32) << 8
                | (data[13] as u32) << 16
                | (data[14] as u32) << 24;
            (v0, v1, v2, v3)
        }
        255 => {
            let v0 = (data[0] as u32)
                | (data[1] as u32) << 8
                | (data[2] as u32) << 16
                | (data[3] as u32) << 24;
            let v1 = (data[4] as u32)
                | (data[5] as u32) << 8
                | (data[6] as u32) << 16
                | (data[7] as u32) << 24;
            let v2 = (data[8] as u32)
                | (data[9] as u32) << 8
                | (data[10] as u32) << 16
                | (data[11] as u32) << 24;
            let v3 = (data[12] as u32)
                | (data[13] as u32) << 8
                | (data[14] as u32) << 16
                | (data[15] as u32) << 24;
            (v0, v1, v2, v3)
        } // END
    }
}

pub fn compress(iter: impl IntoIterator<Item = u32>) -> Vec<u8> {
    let mut buffer = Vec::new();
    let iter = iter.into_iter();
    for mut chunk in (Chunk { iter, n: 4 }) {
        while chunk.len() < 4 {
            chunk.push(0);
        }

        compress_block(&mut buffer, to_block(chunk));
    }

    buffer.shrink_to_fit();

    buffer
}

fn to_block(v: Vec<u32>) -> [u32; 4] {
    if v.len() != 4 {
        unreachable!("length of vector must be 4");
    }

    [v[0], v[1], v[2], v[3]]
}

fn compress_block(buffer: &mut Vec<u8>, chunk: [u32; 4]) {
    let mut mask = 0; //bits0 | bits1 << 2 | bits2 << 4 | bits3 << 6;
    let maskidx = buffer.len();
    buffer.push(0);

    // loop over every integer in the chunk
    for i in 0..4u8 {
        let elem = chunk[i as usize];

        let bits = var_bits(elem);
        mask |= bits << (i << 1);

        // the first byte uses less instructions to encode.
        buffer.push((elem & 0xff) as u8);
        for byte_index in 1..=bits {
            let byte_index = byte_index * 8;
            let byte = (elem >> byte_index) & 0xff;
            buffer.push(byte as u8);
        }
    }

    // apply mask
    buffer[maskidx] = mask;
}

fn var_bits(v: u32) -> u8 {
    if v <= 0xffff {
        if v <= 0xff {
            0
        } else {
            1
        }
    } else {
        if v <= 0xffffff {
            2
        } else {
            3
        }
    }
}

struct Chunk<I>
where
    I: Iterator<Item = u32>,
{
    iter: I,
    n: u32,
}

impl<I> Iterator for Chunk<I>
where
    I: Iterator<Item = u32>,
{
    type Item = Vec<u32>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut v = Vec::with_capacity(self.n as usize);
        while let Some(elem) = self.iter.next() {
            v.push(elem);

            if v.len() as u32 == self.n {
                break;
            }
        }

        if v.len() == 0 {
            return None;
        }

        Some(v)
    }
}
