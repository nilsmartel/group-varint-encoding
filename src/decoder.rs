pub(crate) fn decode_block(v: u8, data: &[u8]) -> (u32, u32, u32, u32, usize) {
    match v {
        // GEN TABLE HERE
        0 => {
            let v0 = data[0] as u32;
            let v1 = data[1] as u32;
            let v2 = data[2] as u32;
            let v3 = data[3] as u32;
            (v0, v1, v2, v3, 4)
        }
        1 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8;
            let v1 = data[2] as u32;
            let v2 = data[3] as u32;
            let v3 = data[4] as u32;
            (v0, v1, v2, v3, 5)
        }
        2 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8 | (data[2] as u32) << 16;
            let v1 = data[3] as u32;
            let v2 = data[4] as u32;
            let v3 = data[5] as u32;
            (v0, v1, v2, v3, 6)
        }
        3 => {
            let v0 = (data[0] as u32)
                | (data[1] as u32) << 8
                | (data[2] as u32) << 16
                | (data[3] as u32) << 24;
            let v1 = data[4] as u32;
            let v2 = data[5] as u32;
            let v3 = data[6] as u32;
            (v0, v1, v2, v3, 7)
        }
        4 => {
            let v0 = data[0] as u32;
            let v1 = (data[1] as u32) | (data[2] as u32) << 8;
            let v2 = data[3] as u32;
            let v3 = data[4] as u32;
            (v0, v1, v2, v3, 5)
        }
        5 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8;
            let v1 = (data[2] as u32) | (data[3] as u32) << 8;
            let v2 = data[4] as u32;
            let v3 = data[5] as u32;
            (v0, v1, v2, v3, 6)
        }
        6 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8 | (data[2] as u32) << 16;
            let v1 = (data[3] as u32) | (data[4] as u32) << 8;
            let v2 = data[5] as u32;
            let v3 = data[6] as u32;
            (v0, v1, v2, v3, 7)
        }
        7 => {
            let v0 = (data[0] as u32)
                | (data[1] as u32) << 8
                | (data[2] as u32) << 16
                | (data[3] as u32) << 24;
            let v1 = (data[4] as u32) | (data[5] as u32) << 8;
            let v2 = data[6] as u32;
            let v3 = data[7] as u32;
            (v0, v1, v2, v3, 8)
        }
        8 => {
            let v0 = data[0] as u32;
            let v1 = (data[1] as u32) | (data[2] as u32) << 8 | (data[3] as u32) << 16;
            let v2 = data[4] as u32;
            let v3 = data[5] as u32;
            (v0, v1, v2, v3, 6)
        }
        9 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8;
            let v1 = (data[2] as u32) | (data[3] as u32) << 8 | (data[4] as u32) << 16;
            let v2 = data[5] as u32;
            let v3 = data[6] as u32;
            (v0, v1, v2, v3, 7)
        }
        10 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8 | (data[2] as u32) << 16;
            let v1 = (data[3] as u32) | (data[4] as u32) << 8 | (data[5] as u32) << 16;
            let v2 = data[6] as u32;
            let v3 = data[7] as u32;
            (v0, v1, v2, v3, 8)
        }
        11 => {
            let v0 = (data[0] as u32)
                | (data[1] as u32) << 8
                | (data[2] as u32) << 16
                | (data[3] as u32) << 24;
            let v1 = (data[4] as u32) | (data[5] as u32) << 8 | (data[6] as u32) << 16;
            let v2 = data[7] as u32;
            let v3 = data[8] as u32;
            (v0, v1, v2, v3, 9)
        }
        12 => {
            let v0 = data[0] as u32;
            let v1 = (data[1] as u32)
                | (data[2] as u32) << 8
                | (data[3] as u32) << 16
                | (data[4] as u32) << 24;
            let v2 = data[5] as u32;
            let v3 = data[6] as u32;
            (v0, v1, v2, v3, 7)
        }
        13 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8;
            let v1 = (data[2] as u32)
                | (data[3] as u32) << 8
                | (data[4] as u32) << 16
                | (data[5] as u32) << 24;
            let v2 = data[6] as u32;
            let v3 = data[7] as u32;
            (v0, v1, v2, v3, 8)
        }
        14 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8 | (data[2] as u32) << 16;
            let v1 = (data[3] as u32)
                | (data[4] as u32) << 8
                | (data[5] as u32) << 16
                | (data[6] as u32) << 24;
            let v2 = data[7] as u32;
            let v3 = data[8] as u32;
            (v0, v1, v2, v3, 9)
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
            (v0, v1, v2, v3, 10)
        }
        16 => {
            let v0 = data[0] as u32;
            let v1 = data[1] as u32;
            let v2 = (data[2] as u32) | (data[3] as u32) << 8;
            let v3 = data[4] as u32;
            (v0, v1, v2, v3, 5)
        }
        17 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8;
            let v1 = data[2] as u32;
            let v2 = (data[3] as u32) | (data[4] as u32) << 8;
            let v3 = data[5] as u32;
            (v0, v1, v2, v3, 6)
        }
        18 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8 | (data[2] as u32) << 16;
            let v1 = data[3] as u32;
            let v2 = (data[4] as u32) | (data[5] as u32) << 8;
            let v3 = data[6] as u32;
            (v0, v1, v2, v3, 7)
        }
        19 => {
            let v0 = (data[0] as u32)
                | (data[1] as u32) << 8
                | (data[2] as u32) << 16
                | (data[3] as u32) << 24;
            let v1 = data[4] as u32;
            let v2 = (data[5] as u32) | (data[6] as u32) << 8;
            let v3 = data[7] as u32;
            (v0, v1, v2, v3, 8)
        }
        20 => {
            let v0 = data[0] as u32;
            let v1 = (data[1] as u32) | (data[2] as u32) << 8;
            let v2 = (data[3] as u32) | (data[4] as u32) << 8;
            let v3 = data[5] as u32;
            (v0, v1, v2, v3, 6)
        }
        21 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8;
            let v1 = (data[2] as u32) | (data[3] as u32) << 8;
            let v2 = (data[4] as u32) | (data[5] as u32) << 8;
            let v3 = data[6] as u32;
            (v0, v1, v2, v3, 7)
        }
        22 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8 | (data[2] as u32) << 16;
            let v1 = (data[3] as u32) | (data[4] as u32) << 8;
            let v2 = (data[5] as u32) | (data[6] as u32) << 8;
            let v3 = data[7] as u32;
            (v0, v1, v2, v3, 8)
        }
        23 => {
            let v0 = (data[0] as u32)
                | (data[1] as u32) << 8
                | (data[2] as u32) << 16
                | (data[3] as u32) << 24;
            let v1 = (data[4] as u32) | (data[5] as u32) << 8;
            let v2 = (data[6] as u32) | (data[7] as u32) << 8;
            let v3 = data[8] as u32;
            (v0, v1, v2, v3, 9)
        }
        24 => {
            let v0 = data[0] as u32;
            let v1 = (data[1] as u32) | (data[2] as u32) << 8 | (data[3] as u32) << 16;
            let v2 = (data[4] as u32) | (data[5] as u32) << 8;
            let v3 = data[6] as u32;
            (v0, v1, v2, v3, 7)
        }
        25 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8;
            let v1 = (data[2] as u32) | (data[3] as u32) << 8 | (data[4] as u32) << 16;
            let v2 = (data[5] as u32) | (data[6] as u32) << 8;
            let v3 = data[7] as u32;
            (v0, v1, v2, v3, 8)
        }
        26 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8 | (data[2] as u32) << 16;
            let v1 = (data[3] as u32) | (data[4] as u32) << 8 | (data[5] as u32) << 16;
            let v2 = (data[6] as u32) | (data[7] as u32) << 8;
            let v3 = data[8] as u32;
            (v0, v1, v2, v3, 9)
        }
        27 => {
            let v0 = (data[0] as u32)
                | (data[1] as u32) << 8
                | (data[2] as u32) << 16
                | (data[3] as u32) << 24;
            let v1 = (data[4] as u32) | (data[5] as u32) << 8 | (data[6] as u32) << 16;
            let v2 = (data[7] as u32) | (data[8] as u32) << 8;
            let v3 = data[9] as u32;
            (v0, v1, v2, v3, 10)
        }
        28 => {
            let v0 = data[0] as u32;
            let v1 = (data[1] as u32)
                | (data[2] as u32) << 8
                | (data[3] as u32) << 16
                | (data[4] as u32) << 24;
            let v2 = (data[5] as u32) | (data[6] as u32) << 8;
            let v3 = data[7] as u32;
            (v0, v1, v2, v3, 8)
        }
        29 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8;
            let v1 = (data[2] as u32)
                | (data[3] as u32) << 8
                | (data[4] as u32) << 16
                | (data[5] as u32) << 24;
            let v2 = (data[6] as u32) | (data[7] as u32) << 8;
            let v3 = data[8] as u32;
            (v0, v1, v2, v3, 9)
        }
        30 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8 | (data[2] as u32) << 16;
            let v1 = (data[3] as u32)
                | (data[4] as u32) << 8
                | (data[5] as u32) << 16
                | (data[6] as u32) << 24;
            let v2 = (data[7] as u32) | (data[8] as u32) << 8;
            let v3 = data[9] as u32;
            (v0, v1, v2, v3, 10)
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
            (v0, v1, v2, v3, 11)
        }
        32 => {
            let v0 = data[0] as u32;
            let v1 = data[1] as u32;
            let v2 = (data[2] as u32) | (data[3] as u32) << 8 | (data[4] as u32) << 16;
            let v3 = data[5] as u32;
            (v0, v1, v2, v3, 6)
        }
        33 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8;
            let v1 = data[2] as u32;
            let v2 = (data[3] as u32) | (data[4] as u32) << 8 | (data[5] as u32) << 16;
            let v3 = data[6] as u32;
            (v0, v1, v2, v3, 7)
        }
        34 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8 | (data[2] as u32) << 16;
            let v1 = data[3] as u32;
            let v2 = (data[4] as u32) | (data[5] as u32) << 8 | (data[6] as u32) << 16;
            let v3 = data[7] as u32;
            (v0, v1, v2, v3, 8)
        }
        35 => {
            let v0 = (data[0] as u32)
                | (data[1] as u32) << 8
                | (data[2] as u32) << 16
                | (data[3] as u32) << 24;
            let v1 = data[4] as u32;
            let v2 = (data[5] as u32) | (data[6] as u32) << 8 | (data[7] as u32) << 16;
            let v3 = data[8] as u32;
            (v0, v1, v2, v3, 9)
        }
        36 => {
            let v0 = data[0] as u32;
            let v1 = (data[1] as u32) | (data[2] as u32) << 8;
            let v2 = (data[3] as u32) | (data[4] as u32) << 8 | (data[5] as u32) << 16;
            let v3 = data[6] as u32;
            (v0, v1, v2, v3, 7)
        }
        37 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8;
            let v1 = (data[2] as u32) | (data[3] as u32) << 8;
            let v2 = (data[4] as u32) | (data[5] as u32) << 8 | (data[6] as u32) << 16;
            let v3 = data[7] as u32;
            (v0, v1, v2, v3, 8)
        }
        38 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8 | (data[2] as u32) << 16;
            let v1 = (data[3] as u32) | (data[4] as u32) << 8;
            let v2 = (data[5] as u32) | (data[6] as u32) << 8 | (data[7] as u32) << 16;
            let v3 = data[8] as u32;
            (v0, v1, v2, v3, 9)
        }
        39 => {
            let v0 = (data[0] as u32)
                | (data[1] as u32) << 8
                | (data[2] as u32) << 16
                | (data[3] as u32) << 24;
            let v1 = (data[4] as u32) | (data[5] as u32) << 8;
            let v2 = (data[6] as u32) | (data[7] as u32) << 8 | (data[8] as u32) << 16;
            let v3 = data[9] as u32;
            (v0, v1, v2, v3, 10)
        }
        40 => {
            let v0 = data[0] as u32;
            let v1 = (data[1] as u32) | (data[2] as u32) << 8 | (data[3] as u32) << 16;
            let v2 = (data[4] as u32) | (data[5] as u32) << 8 | (data[6] as u32) << 16;
            let v3 = data[7] as u32;
            (v0, v1, v2, v3, 8)
        }
        41 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8;
            let v1 = (data[2] as u32) | (data[3] as u32) << 8 | (data[4] as u32) << 16;
            let v2 = (data[5] as u32) | (data[6] as u32) << 8 | (data[7] as u32) << 16;
            let v3 = data[8] as u32;
            (v0, v1, v2, v3, 9)
        }
        42 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8 | (data[2] as u32) << 16;
            let v1 = (data[3] as u32) | (data[4] as u32) << 8 | (data[5] as u32) << 16;
            let v2 = (data[6] as u32) | (data[7] as u32) << 8 | (data[8] as u32) << 16;
            let v3 = data[9] as u32;
            (v0, v1, v2, v3, 10)
        }
        43 => {
            let v0 = (data[0] as u32)
                | (data[1] as u32) << 8
                | (data[2] as u32) << 16
                | (data[3] as u32) << 24;
            let v1 = (data[4] as u32) | (data[5] as u32) << 8 | (data[6] as u32) << 16;
            let v2 = (data[7] as u32) | (data[8] as u32) << 8 | (data[9] as u32) << 16;
            let v3 = data[10] as u32;
            (v0, v1, v2, v3, 11)
        }
        44 => {
            let v0 = data[0] as u32;
            let v1 = (data[1] as u32)
                | (data[2] as u32) << 8
                | (data[3] as u32) << 16
                | (data[4] as u32) << 24;
            let v2 = (data[5] as u32) | (data[6] as u32) << 8 | (data[7] as u32) << 16;
            let v3 = data[8] as u32;
            (v0, v1, v2, v3, 9)
        }
        45 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8;
            let v1 = (data[2] as u32)
                | (data[3] as u32) << 8
                | (data[4] as u32) << 16
                | (data[5] as u32) << 24;
            let v2 = (data[6] as u32) | (data[7] as u32) << 8 | (data[8] as u32) << 16;
            let v3 = data[9] as u32;
            (v0, v1, v2, v3, 10)
        }
        46 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8 | (data[2] as u32) << 16;
            let v1 = (data[3] as u32)
                | (data[4] as u32) << 8
                | (data[5] as u32) << 16
                | (data[6] as u32) << 24;
            let v2 = (data[7] as u32) | (data[8] as u32) << 8 | (data[9] as u32) << 16;
            let v3 = data[10] as u32;
            (v0, v1, v2, v3, 11)
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
            (v0, v1, v2, v3, 12)
        }
        48 => {
            let v0 = data[0] as u32;
            let v1 = data[1] as u32;
            let v2 = (data[2] as u32)
                | (data[3] as u32) << 8
                | (data[4] as u32) << 16
                | (data[5] as u32) << 24;
            let v3 = data[6] as u32;
            (v0, v1, v2, v3, 7)
        }
        49 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8;
            let v1 = data[2] as u32;
            let v2 = (data[3] as u32)
                | (data[4] as u32) << 8
                | (data[5] as u32) << 16
                | (data[6] as u32) << 24;
            let v3 = data[7] as u32;
            (v0, v1, v2, v3, 8)
        }
        50 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8 | (data[2] as u32) << 16;
            let v1 = data[3] as u32;
            let v2 = (data[4] as u32)
                | (data[5] as u32) << 8
                | (data[6] as u32) << 16
                | (data[7] as u32) << 24;
            let v3 = data[8] as u32;
            (v0, v1, v2, v3, 9)
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
            (v0, v1, v2, v3, 10)
        }
        52 => {
            let v0 = data[0] as u32;
            let v1 = (data[1] as u32) | (data[2] as u32) << 8;
            let v2 = (data[3] as u32)
                | (data[4] as u32) << 8
                | (data[5] as u32) << 16
                | (data[6] as u32) << 24;
            let v3 = data[7] as u32;
            (v0, v1, v2, v3, 8)
        }
        53 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8;
            let v1 = (data[2] as u32) | (data[3] as u32) << 8;
            let v2 = (data[4] as u32)
                | (data[5] as u32) << 8
                | (data[6] as u32) << 16
                | (data[7] as u32) << 24;
            let v3 = data[8] as u32;
            (v0, v1, v2, v3, 9)
        }
        54 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8 | (data[2] as u32) << 16;
            let v1 = (data[3] as u32) | (data[4] as u32) << 8;
            let v2 = (data[5] as u32)
                | (data[6] as u32) << 8
                | (data[7] as u32) << 16
                | (data[8] as u32) << 24;
            let v3 = data[9] as u32;
            (v0, v1, v2, v3, 10)
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
            (v0, v1, v2, v3, 11)
        }
        56 => {
            let v0 = data[0] as u32;
            let v1 = (data[1] as u32) | (data[2] as u32) << 8 | (data[3] as u32) << 16;
            let v2 = (data[4] as u32)
                | (data[5] as u32) << 8
                | (data[6] as u32) << 16
                | (data[7] as u32) << 24;
            let v3 = data[8] as u32;
            (v0, v1, v2, v3, 9)
        }
        57 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8;
            let v1 = (data[2] as u32) | (data[3] as u32) << 8 | (data[4] as u32) << 16;
            let v2 = (data[5] as u32)
                | (data[6] as u32) << 8
                | (data[7] as u32) << 16
                | (data[8] as u32) << 24;
            let v3 = data[9] as u32;
            (v0, v1, v2, v3, 10)
        }
        58 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8 | (data[2] as u32) << 16;
            let v1 = (data[3] as u32) | (data[4] as u32) << 8 | (data[5] as u32) << 16;
            let v2 = (data[6] as u32)
                | (data[7] as u32) << 8
                | (data[8] as u32) << 16
                | (data[9] as u32) << 24;
            let v3 = data[10] as u32;
            (v0, v1, v2, v3, 11)
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
            (v0, v1, v2, v3, 12)
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
            (v0, v1, v2, v3, 10)
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
            (v0, v1, v2, v3, 11)
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
            (v0, v1, v2, v3, 12)
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
            (v0, v1, v2, v3, 13)
        }
        64 => {
            let v0 = data[0] as u32;
            let v1 = data[1] as u32;
            let v2 = data[2] as u32;
            let v3 = (data[3] as u32) | (data[4] as u32) << 8;
            (v0, v1, v2, v3, 5)
        }
        65 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8;
            let v1 = data[2] as u32;
            let v2 = data[3] as u32;
            let v3 = (data[4] as u32) | (data[5] as u32) << 8;
            (v0, v1, v2, v3, 6)
        }
        66 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8 | (data[2] as u32) << 16;
            let v1 = data[3] as u32;
            let v2 = data[4] as u32;
            let v3 = (data[5] as u32) | (data[6] as u32) << 8;
            (v0, v1, v2, v3, 7)
        }
        67 => {
            let v0 = (data[0] as u32)
                | (data[1] as u32) << 8
                | (data[2] as u32) << 16
                | (data[3] as u32) << 24;
            let v1 = data[4] as u32;
            let v2 = data[5] as u32;
            let v3 = (data[6] as u32) | (data[7] as u32) << 8;
            (v0, v1, v2, v3, 8)
        }
        68 => {
            let v0 = data[0] as u32;
            let v1 = (data[1] as u32) | (data[2] as u32) << 8;
            let v2 = data[3] as u32;
            let v3 = (data[4] as u32) | (data[5] as u32) << 8;
            (v0, v1, v2, v3, 6)
        }
        69 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8;
            let v1 = (data[2] as u32) | (data[3] as u32) << 8;
            let v2 = data[4] as u32;
            let v3 = (data[5] as u32) | (data[6] as u32) << 8;
            (v0, v1, v2, v3, 7)
        }
        70 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8 | (data[2] as u32) << 16;
            let v1 = (data[3] as u32) | (data[4] as u32) << 8;
            let v2 = data[5] as u32;
            let v3 = (data[6] as u32) | (data[7] as u32) << 8;
            (v0, v1, v2, v3, 8)
        }
        71 => {
            let v0 = (data[0] as u32)
                | (data[1] as u32) << 8
                | (data[2] as u32) << 16
                | (data[3] as u32) << 24;
            let v1 = (data[4] as u32) | (data[5] as u32) << 8;
            let v2 = data[6] as u32;
            let v3 = (data[7] as u32) | (data[8] as u32) << 8;
            (v0, v1, v2, v3, 9)
        }
        72 => {
            let v0 = data[0] as u32;
            let v1 = (data[1] as u32) | (data[2] as u32) << 8 | (data[3] as u32) << 16;
            let v2 = data[4] as u32;
            let v3 = (data[5] as u32) | (data[6] as u32) << 8;
            (v0, v1, v2, v3, 7)
        }
        73 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8;
            let v1 = (data[2] as u32) | (data[3] as u32) << 8 | (data[4] as u32) << 16;
            let v2 = data[5] as u32;
            let v3 = (data[6] as u32) | (data[7] as u32) << 8;
            (v0, v1, v2, v3, 8)
        }
        74 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8 | (data[2] as u32) << 16;
            let v1 = (data[3] as u32) | (data[4] as u32) << 8 | (data[5] as u32) << 16;
            let v2 = data[6] as u32;
            let v3 = (data[7] as u32) | (data[8] as u32) << 8;
            (v0, v1, v2, v3, 9)
        }
        75 => {
            let v0 = (data[0] as u32)
                | (data[1] as u32) << 8
                | (data[2] as u32) << 16
                | (data[3] as u32) << 24;
            let v1 = (data[4] as u32) | (data[5] as u32) << 8 | (data[6] as u32) << 16;
            let v2 = data[7] as u32;
            let v3 = (data[8] as u32) | (data[9] as u32) << 8;
            (v0, v1, v2, v3, 10)
        }
        76 => {
            let v0 = data[0] as u32;
            let v1 = (data[1] as u32)
                | (data[2] as u32) << 8
                | (data[3] as u32) << 16
                | (data[4] as u32) << 24;
            let v2 = data[5] as u32;
            let v3 = (data[6] as u32) | (data[7] as u32) << 8;
            (v0, v1, v2, v3, 8)
        }
        77 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8;
            let v1 = (data[2] as u32)
                | (data[3] as u32) << 8
                | (data[4] as u32) << 16
                | (data[5] as u32) << 24;
            let v2 = data[6] as u32;
            let v3 = (data[7] as u32) | (data[8] as u32) << 8;
            (v0, v1, v2, v3, 9)
        }
        78 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8 | (data[2] as u32) << 16;
            let v1 = (data[3] as u32)
                | (data[4] as u32) << 8
                | (data[5] as u32) << 16
                | (data[6] as u32) << 24;
            let v2 = data[7] as u32;
            let v3 = (data[8] as u32) | (data[9] as u32) << 8;
            (v0, v1, v2, v3, 10)
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
            (v0, v1, v2, v3, 11)
        }
        80 => {
            let v0 = data[0] as u32;
            let v1 = data[1] as u32;
            let v2 = (data[2] as u32) | (data[3] as u32) << 8;
            let v3 = (data[4] as u32) | (data[5] as u32) << 8;
            (v0, v1, v2, v3, 6)
        }
        81 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8;
            let v1 = data[2] as u32;
            let v2 = (data[3] as u32) | (data[4] as u32) << 8;
            let v3 = (data[5] as u32) | (data[6] as u32) << 8;
            (v0, v1, v2, v3, 7)
        }
        82 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8 | (data[2] as u32) << 16;
            let v1 = data[3] as u32;
            let v2 = (data[4] as u32) | (data[5] as u32) << 8;
            let v3 = (data[6] as u32) | (data[7] as u32) << 8;
            (v0, v1, v2, v3, 8)
        }
        83 => {
            let v0 = (data[0] as u32)
                | (data[1] as u32) << 8
                | (data[2] as u32) << 16
                | (data[3] as u32) << 24;
            let v1 = data[4] as u32;
            let v2 = (data[5] as u32) | (data[6] as u32) << 8;
            let v3 = (data[7] as u32) | (data[8] as u32) << 8;
            (v0, v1, v2, v3, 9)
        }
        84 => {
            let v0 = data[0] as u32;
            let v1 = (data[1] as u32) | (data[2] as u32) << 8;
            let v2 = (data[3] as u32) | (data[4] as u32) << 8;
            let v3 = (data[5] as u32) | (data[6] as u32) << 8;
            (v0, v1, v2, v3, 7)
        }
        85 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8;
            let v1 = (data[2] as u32) | (data[3] as u32) << 8;
            let v2 = (data[4] as u32) | (data[5] as u32) << 8;
            let v3 = (data[6] as u32) | (data[7] as u32) << 8;
            (v0, v1, v2, v3, 8)
        }
        86 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8 | (data[2] as u32) << 16;
            let v1 = (data[3] as u32) | (data[4] as u32) << 8;
            let v2 = (data[5] as u32) | (data[6] as u32) << 8;
            let v3 = (data[7] as u32) | (data[8] as u32) << 8;
            (v0, v1, v2, v3, 9)
        }
        87 => {
            let v0 = (data[0] as u32)
                | (data[1] as u32) << 8
                | (data[2] as u32) << 16
                | (data[3] as u32) << 24;
            let v1 = (data[4] as u32) | (data[5] as u32) << 8;
            let v2 = (data[6] as u32) | (data[7] as u32) << 8;
            let v3 = (data[8] as u32) | (data[9] as u32) << 8;
            (v0, v1, v2, v3, 10)
        }
        88 => {
            let v0 = data[0] as u32;
            let v1 = (data[1] as u32) | (data[2] as u32) << 8 | (data[3] as u32) << 16;
            let v2 = (data[4] as u32) | (data[5] as u32) << 8;
            let v3 = (data[6] as u32) | (data[7] as u32) << 8;
            (v0, v1, v2, v3, 8)
        }
        89 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8;
            let v1 = (data[2] as u32) | (data[3] as u32) << 8 | (data[4] as u32) << 16;
            let v2 = (data[5] as u32) | (data[6] as u32) << 8;
            let v3 = (data[7] as u32) | (data[8] as u32) << 8;
            (v0, v1, v2, v3, 9)
        }
        90 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8 | (data[2] as u32) << 16;
            let v1 = (data[3] as u32) | (data[4] as u32) << 8 | (data[5] as u32) << 16;
            let v2 = (data[6] as u32) | (data[7] as u32) << 8;
            let v3 = (data[8] as u32) | (data[9] as u32) << 8;
            (v0, v1, v2, v3, 10)
        }
        91 => {
            let v0 = (data[0] as u32)
                | (data[1] as u32) << 8
                | (data[2] as u32) << 16
                | (data[3] as u32) << 24;
            let v1 = (data[4] as u32) | (data[5] as u32) << 8 | (data[6] as u32) << 16;
            let v2 = (data[7] as u32) | (data[8] as u32) << 8;
            let v3 = (data[9] as u32) | (data[10] as u32) << 8;
            (v0, v1, v2, v3, 11)
        }
        92 => {
            let v0 = data[0] as u32;
            let v1 = (data[1] as u32)
                | (data[2] as u32) << 8
                | (data[3] as u32) << 16
                | (data[4] as u32) << 24;
            let v2 = (data[5] as u32) | (data[6] as u32) << 8;
            let v3 = (data[7] as u32) | (data[8] as u32) << 8;
            (v0, v1, v2, v3, 9)
        }
        93 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8;
            let v1 = (data[2] as u32)
                | (data[3] as u32) << 8
                | (data[4] as u32) << 16
                | (data[5] as u32) << 24;
            let v2 = (data[6] as u32) | (data[7] as u32) << 8;
            let v3 = (data[8] as u32) | (data[9] as u32) << 8;
            (v0, v1, v2, v3, 10)
        }
        94 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8 | (data[2] as u32) << 16;
            let v1 = (data[3] as u32)
                | (data[4] as u32) << 8
                | (data[5] as u32) << 16
                | (data[6] as u32) << 24;
            let v2 = (data[7] as u32) | (data[8] as u32) << 8;
            let v3 = (data[9] as u32) | (data[10] as u32) << 8;
            (v0, v1, v2, v3, 11)
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
            (v0, v1, v2, v3, 12)
        }
        96 => {
            let v0 = data[0] as u32;
            let v1 = data[1] as u32;
            let v2 = (data[2] as u32) | (data[3] as u32) << 8 | (data[4] as u32) << 16;
            let v3 = (data[5] as u32) | (data[6] as u32) << 8;
            (v0, v1, v2, v3, 7)
        }
        97 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8;
            let v1 = data[2] as u32;
            let v2 = (data[3] as u32) | (data[4] as u32) << 8 | (data[5] as u32) << 16;
            let v3 = (data[6] as u32) | (data[7] as u32) << 8;
            (v0, v1, v2, v3, 8)
        }
        98 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8 | (data[2] as u32) << 16;
            let v1 = data[3] as u32;
            let v2 = (data[4] as u32) | (data[5] as u32) << 8 | (data[6] as u32) << 16;
            let v3 = (data[7] as u32) | (data[8] as u32) << 8;
            (v0, v1, v2, v3, 9)
        }
        99 => {
            let v0 = (data[0] as u32)
                | (data[1] as u32) << 8
                | (data[2] as u32) << 16
                | (data[3] as u32) << 24;
            let v1 = data[4] as u32;
            let v2 = (data[5] as u32) | (data[6] as u32) << 8 | (data[7] as u32) << 16;
            let v3 = (data[8] as u32) | (data[9] as u32) << 8;
            (v0, v1, v2, v3, 10)
        }
        100 => {
            let v0 = data[0] as u32;
            let v1 = (data[1] as u32) | (data[2] as u32) << 8;
            let v2 = (data[3] as u32) | (data[4] as u32) << 8 | (data[5] as u32) << 16;
            let v3 = (data[6] as u32) | (data[7] as u32) << 8;
            (v0, v1, v2, v3, 8)
        }
        101 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8;
            let v1 = (data[2] as u32) | (data[3] as u32) << 8;
            let v2 = (data[4] as u32) | (data[5] as u32) << 8 | (data[6] as u32) << 16;
            let v3 = (data[7] as u32) | (data[8] as u32) << 8;
            (v0, v1, v2, v3, 9)
        }
        102 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8 | (data[2] as u32) << 16;
            let v1 = (data[3] as u32) | (data[4] as u32) << 8;
            let v2 = (data[5] as u32) | (data[6] as u32) << 8 | (data[7] as u32) << 16;
            let v3 = (data[8] as u32) | (data[9] as u32) << 8;
            (v0, v1, v2, v3, 10)
        }
        103 => {
            let v0 = (data[0] as u32)
                | (data[1] as u32) << 8
                | (data[2] as u32) << 16
                | (data[3] as u32) << 24;
            let v1 = (data[4] as u32) | (data[5] as u32) << 8;
            let v2 = (data[6] as u32) | (data[7] as u32) << 8 | (data[8] as u32) << 16;
            let v3 = (data[9] as u32) | (data[10] as u32) << 8;
            (v0, v1, v2, v3, 11)
        }
        104 => {
            let v0 = data[0] as u32;
            let v1 = (data[1] as u32) | (data[2] as u32) << 8 | (data[3] as u32) << 16;
            let v2 = (data[4] as u32) | (data[5] as u32) << 8 | (data[6] as u32) << 16;
            let v3 = (data[7] as u32) | (data[8] as u32) << 8;
            (v0, v1, v2, v3, 9)
        }
        105 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8;
            let v1 = (data[2] as u32) | (data[3] as u32) << 8 | (data[4] as u32) << 16;
            let v2 = (data[5] as u32) | (data[6] as u32) << 8 | (data[7] as u32) << 16;
            let v3 = (data[8] as u32) | (data[9] as u32) << 8;
            (v0, v1, v2, v3, 10)
        }
        106 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8 | (data[2] as u32) << 16;
            let v1 = (data[3] as u32) | (data[4] as u32) << 8 | (data[5] as u32) << 16;
            let v2 = (data[6] as u32) | (data[7] as u32) << 8 | (data[8] as u32) << 16;
            let v3 = (data[9] as u32) | (data[10] as u32) << 8;
            (v0, v1, v2, v3, 11)
        }
        107 => {
            let v0 = (data[0] as u32)
                | (data[1] as u32) << 8
                | (data[2] as u32) << 16
                | (data[3] as u32) << 24;
            let v1 = (data[4] as u32) | (data[5] as u32) << 8 | (data[6] as u32) << 16;
            let v2 = (data[7] as u32) | (data[8] as u32) << 8 | (data[9] as u32) << 16;
            let v3 = (data[10] as u32) | (data[11] as u32) << 8;
            (v0, v1, v2, v3, 12)
        }
        108 => {
            let v0 = data[0] as u32;
            let v1 = (data[1] as u32)
                | (data[2] as u32) << 8
                | (data[3] as u32) << 16
                | (data[4] as u32) << 24;
            let v2 = (data[5] as u32) | (data[6] as u32) << 8 | (data[7] as u32) << 16;
            let v3 = (data[8] as u32) | (data[9] as u32) << 8;
            (v0, v1, v2, v3, 10)
        }
        109 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8;
            let v1 = (data[2] as u32)
                | (data[3] as u32) << 8
                | (data[4] as u32) << 16
                | (data[5] as u32) << 24;
            let v2 = (data[6] as u32) | (data[7] as u32) << 8 | (data[8] as u32) << 16;
            let v3 = (data[9] as u32) | (data[10] as u32) << 8;
            (v0, v1, v2, v3, 11)
        }
        110 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8 | (data[2] as u32) << 16;
            let v1 = (data[3] as u32)
                | (data[4] as u32) << 8
                | (data[5] as u32) << 16
                | (data[6] as u32) << 24;
            let v2 = (data[7] as u32) | (data[8] as u32) << 8 | (data[9] as u32) << 16;
            let v3 = (data[10] as u32) | (data[11] as u32) << 8;
            (v0, v1, v2, v3, 12)
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
            (v0, v1, v2, v3, 13)
        }
        112 => {
            let v0 = data[0] as u32;
            let v1 = data[1] as u32;
            let v2 = (data[2] as u32)
                | (data[3] as u32) << 8
                | (data[4] as u32) << 16
                | (data[5] as u32) << 24;
            let v3 = (data[6] as u32) | (data[7] as u32) << 8;
            (v0, v1, v2, v3, 8)
        }
        113 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8;
            let v1 = data[2] as u32;
            let v2 = (data[3] as u32)
                | (data[4] as u32) << 8
                | (data[5] as u32) << 16
                | (data[6] as u32) << 24;
            let v3 = (data[7] as u32) | (data[8] as u32) << 8;
            (v0, v1, v2, v3, 9)
        }
        114 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8 | (data[2] as u32) << 16;
            let v1 = data[3] as u32;
            let v2 = (data[4] as u32)
                | (data[5] as u32) << 8
                | (data[6] as u32) << 16
                | (data[7] as u32) << 24;
            let v3 = (data[8] as u32) | (data[9] as u32) << 8;
            (v0, v1, v2, v3, 10)
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
            (v0, v1, v2, v3, 11)
        }
        116 => {
            let v0 = data[0] as u32;
            let v1 = (data[1] as u32) | (data[2] as u32) << 8;
            let v2 = (data[3] as u32)
                | (data[4] as u32) << 8
                | (data[5] as u32) << 16
                | (data[6] as u32) << 24;
            let v3 = (data[7] as u32) | (data[8] as u32) << 8;
            (v0, v1, v2, v3, 9)
        }
        117 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8;
            let v1 = (data[2] as u32) | (data[3] as u32) << 8;
            let v2 = (data[4] as u32)
                | (data[5] as u32) << 8
                | (data[6] as u32) << 16
                | (data[7] as u32) << 24;
            let v3 = (data[8] as u32) | (data[9] as u32) << 8;
            (v0, v1, v2, v3, 10)
        }
        118 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8 | (data[2] as u32) << 16;
            let v1 = (data[3] as u32) | (data[4] as u32) << 8;
            let v2 = (data[5] as u32)
                | (data[6] as u32) << 8
                | (data[7] as u32) << 16
                | (data[8] as u32) << 24;
            let v3 = (data[9] as u32) | (data[10] as u32) << 8;
            (v0, v1, v2, v3, 11)
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
            (v0, v1, v2, v3, 12)
        }
        120 => {
            let v0 = data[0] as u32;
            let v1 = (data[1] as u32) | (data[2] as u32) << 8 | (data[3] as u32) << 16;
            let v2 = (data[4] as u32)
                | (data[5] as u32) << 8
                | (data[6] as u32) << 16
                | (data[7] as u32) << 24;
            let v3 = (data[8] as u32) | (data[9] as u32) << 8;
            (v0, v1, v2, v3, 10)
        }
        121 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8;
            let v1 = (data[2] as u32) | (data[3] as u32) << 8 | (data[4] as u32) << 16;
            let v2 = (data[5] as u32)
                | (data[6] as u32) << 8
                | (data[7] as u32) << 16
                | (data[8] as u32) << 24;
            let v3 = (data[9] as u32) | (data[10] as u32) << 8;
            (v0, v1, v2, v3, 11)
        }
        122 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8 | (data[2] as u32) << 16;
            let v1 = (data[3] as u32) | (data[4] as u32) << 8 | (data[5] as u32) << 16;
            let v2 = (data[6] as u32)
                | (data[7] as u32) << 8
                | (data[8] as u32) << 16
                | (data[9] as u32) << 24;
            let v3 = (data[10] as u32) | (data[11] as u32) << 8;
            (v0, v1, v2, v3, 12)
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
            (v0, v1, v2, v3, 13)
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
            (v0, v1, v2, v3, 11)
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
            (v0, v1, v2, v3, 12)
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
            (v0, v1, v2, v3, 13)
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
            (v0, v1, v2, v3, 14)
        }
        128 => {
            let v0 = data[0] as u32;
            let v1 = data[1] as u32;
            let v2 = data[2] as u32;
            let v3 = (data[3] as u32) | (data[4] as u32) << 8 | (data[5] as u32) << 16;
            (v0, v1, v2, v3, 6)
        }
        129 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8;
            let v1 = data[2] as u32;
            let v2 = data[3] as u32;
            let v3 = (data[4] as u32) | (data[5] as u32) << 8 | (data[6] as u32) << 16;
            (v0, v1, v2, v3, 7)
        }
        130 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8 | (data[2] as u32) << 16;
            let v1 = data[3] as u32;
            let v2 = data[4] as u32;
            let v3 = (data[5] as u32) | (data[6] as u32) << 8 | (data[7] as u32) << 16;
            (v0, v1, v2, v3, 8)
        }
        131 => {
            let v0 = (data[0] as u32)
                | (data[1] as u32) << 8
                | (data[2] as u32) << 16
                | (data[3] as u32) << 24;
            let v1 = data[4] as u32;
            let v2 = data[5] as u32;
            let v3 = (data[6] as u32) | (data[7] as u32) << 8 | (data[8] as u32) << 16;
            (v0, v1, v2, v3, 9)
        }
        132 => {
            let v0 = data[0] as u32;
            let v1 = (data[1] as u32) | (data[2] as u32) << 8;
            let v2 = data[3] as u32;
            let v3 = (data[4] as u32) | (data[5] as u32) << 8 | (data[6] as u32) << 16;
            (v0, v1, v2, v3, 7)
        }
        133 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8;
            let v1 = (data[2] as u32) | (data[3] as u32) << 8;
            let v2 = data[4] as u32;
            let v3 = (data[5] as u32) | (data[6] as u32) << 8 | (data[7] as u32) << 16;
            (v0, v1, v2, v3, 8)
        }
        134 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8 | (data[2] as u32) << 16;
            let v1 = (data[3] as u32) | (data[4] as u32) << 8;
            let v2 = data[5] as u32;
            let v3 = (data[6] as u32) | (data[7] as u32) << 8 | (data[8] as u32) << 16;
            (v0, v1, v2, v3, 9)
        }
        135 => {
            let v0 = (data[0] as u32)
                | (data[1] as u32) << 8
                | (data[2] as u32) << 16
                | (data[3] as u32) << 24;
            let v1 = (data[4] as u32) | (data[5] as u32) << 8;
            let v2 = data[6] as u32;
            let v3 = (data[7] as u32) | (data[8] as u32) << 8 | (data[9] as u32) << 16;
            (v0, v1, v2, v3, 10)
        }
        136 => {
            let v0 = data[0] as u32;
            let v1 = (data[1] as u32) | (data[2] as u32) << 8 | (data[3] as u32) << 16;
            let v2 = data[4] as u32;
            let v3 = (data[5] as u32) | (data[6] as u32) << 8 | (data[7] as u32) << 16;
            (v0, v1, v2, v3, 8)
        }
        137 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8;
            let v1 = (data[2] as u32) | (data[3] as u32) << 8 | (data[4] as u32) << 16;
            let v2 = data[5] as u32;
            let v3 = (data[6] as u32) | (data[7] as u32) << 8 | (data[8] as u32) << 16;
            (v0, v1, v2, v3, 9)
        }
        138 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8 | (data[2] as u32) << 16;
            let v1 = (data[3] as u32) | (data[4] as u32) << 8 | (data[5] as u32) << 16;
            let v2 = data[6] as u32;
            let v3 = (data[7] as u32) | (data[8] as u32) << 8 | (data[9] as u32) << 16;
            (v0, v1, v2, v3, 10)
        }
        139 => {
            let v0 = (data[0] as u32)
                | (data[1] as u32) << 8
                | (data[2] as u32) << 16
                | (data[3] as u32) << 24;
            let v1 = (data[4] as u32) | (data[5] as u32) << 8 | (data[6] as u32) << 16;
            let v2 = data[7] as u32;
            let v3 = (data[8] as u32) | (data[9] as u32) << 8 | (data[10] as u32) << 16;
            (v0, v1, v2, v3, 11)
        }
        140 => {
            let v0 = data[0] as u32;
            let v1 = (data[1] as u32)
                | (data[2] as u32) << 8
                | (data[3] as u32) << 16
                | (data[4] as u32) << 24;
            let v2 = data[5] as u32;
            let v3 = (data[6] as u32) | (data[7] as u32) << 8 | (data[8] as u32) << 16;
            (v0, v1, v2, v3, 9)
        }
        141 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8;
            let v1 = (data[2] as u32)
                | (data[3] as u32) << 8
                | (data[4] as u32) << 16
                | (data[5] as u32) << 24;
            let v2 = data[6] as u32;
            let v3 = (data[7] as u32) | (data[8] as u32) << 8 | (data[9] as u32) << 16;
            (v0, v1, v2, v3, 10)
        }
        142 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8 | (data[2] as u32) << 16;
            let v1 = (data[3] as u32)
                | (data[4] as u32) << 8
                | (data[5] as u32) << 16
                | (data[6] as u32) << 24;
            let v2 = data[7] as u32;
            let v3 = (data[8] as u32) | (data[9] as u32) << 8 | (data[10] as u32) << 16;
            (v0, v1, v2, v3, 11)
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
            (v0, v1, v2, v3, 12)
        }
        144 => {
            let v0 = data[0] as u32;
            let v1 = data[1] as u32;
            let v2 = (data[2] as u32) | (data[3] as u32) << 8;
            let v3 = (data[4] as u32) | (data[5] as u32) << 8 | (data[6] as u32) << 16;
            (v0, v1, v2, v3, 7)
        }
        145 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8;
            let v1 = data[2] as u32;
            let v2 = (data[3] as u32) | (data[4] as u32) << 8;
            let v3 = (data[5] as u32) | (data[6] as u32) << 8 | (data[7] as u32) << 16;
            (v0, v1, v2, v3, 8)
        }
        146 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8 | (data[2] as u32) << 16;
            let v1 = data[3] as u32;
            let v2 = (data[4] as u32) | (data[5] as u32) << 8;
            let v3 = (data[6] as u32) | (data[7] as u32) << 8 | (data[8] as u32) << 16;
            (v0, v1, v2, v3, 9)
        }
        147 => {
            let v0 = (data[0] as u32)
                | (data[1] as u32) << 8
                | (data[2] as u32) << 16
                | (data[3] as u32) << 24;
            let v1 = data[4] as u32;
            let v2 = (data[5] as u32) | (data[6] as u32) << 8;
            let v3 = (data[7] as u32) | (data[8] as u32) << 8 | (data[9] as u32) << 16;
            (v0, v1, v2, v3, 10)
        }
        148 => {
            let v0 = data[0] as u32;
            let v1 = (data[1] as u32) | (data[2] as u32) << 8;
            let v2 = (data[3] as u32) | (data[4] as u32) << 8;
            let v3 = (data[5] as u32) | (data[6] as u32) << 8 | (data[7] as u32) << 16;
            (v0, v1, v2, v3, 8)
        }
        149 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8;
            let v1 = (data[2] as u32) | (data[3] as u32) << 8;
            let v2 = (data[4] as u32) | (data[5] as u32) << 8;
            let v3 = (data[6] as u32) | (data[7] as u32) << 8 | (data[8] as u32) << 16;
            (v0, v1, v2, v3, 9)
        }
        150 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8 | (data[2] as u32) << 16;
            let v1 = (data[3] as u32) | (data[4] as u32) << 8;
            let v2 = (data[5] as u32) | (data[6] as u32) << 8;
            let v3 = (data[7] as u32) | (data[8] as u32) << 8 | (data[9] as u32) << 16;
            (v0, v1, v2, v3, 10)
        }
        151 => {
            let v0 = (data[0] as u32)
                | (data[1] as u32) << 8
                | (data[2] as u32) << 16
                | (data[3] as u32) << 24;
            let v1 = (data[4] as u32) | (data[5] as u32) << 8;
            let v2 = (data[6] as u32) | (data[7] as u32) << 8;
            let v3 = (data[8] as u32) | (data[9] as u32) << 8 | (data[10] as u32) << 16;
            (v0, v1, v2, v3, 11)
        }
        152 => {
            let v0 = data[0] as u32;
            let v1 = (data[1] as u32) | (data[2] as u32) << 8 | (data[3] as u32) << 16;
            let v2 = (data[4] as u32) | (data[5] as u32) << 8;
            let v3 = (data[6] as u32) | (data[7] as u32) << 8 | (data[8] as u32) << 16;
            (v0, v1, v2, v3, 9)
        }
        153 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8;
            let v1 = (data[2] as u32) | (data[3] as u32) << 8 | (data[4] as u32) << 16;
            let v2 = (data[5] as u32) | (data[6] as u32) << 8;
            let v3 = (data[7] as u32) | (data[8] as u32) << 8 | (data[9] as u32) << 16;
            (v0, v1, v2, v3, 10)
        }
        154 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8 | (data[2] as u32) << 16;
            let v1 = (data[3] as u32) | (data[4] as u32) << 8 | (data[5] as u32) << 16;
            let v2 = (data[6] as u32) | (data[7] as u32) << 8;
            let v3 = (data[8] as u32) | (data[9] as u32) << 8 | (data[10] as u32) << 16;
            (v0, v1, v2, v3, 11)
        }
        155 => {
            let v0 = (data[0] as u32)
                | (data[1] as u32) << 8
                | (data[2] as u32) << 16
                | (data[3] as u32) << 24;
            let v1 = (data[4] as u32) | (data[5] as u32) << 8 | (data[6] as u32) << 16;
            let v2 = (data[7] as u32) | (data[8] as u32) << 8;
            let v3 = (data[9] as u32) | (data[10] as u32) << 8 | (data[11] as u32) << 16;
            (v0, v1, v2, v3, 12)
        }
        156 => {
            let v0 = data[0] as u32;
            let v1 = (data[1] as u32)
                | (data[2] as u32) << 8
                | (data[3] as u32) << 16
                | (data[4] as u32) << 24;
            let v2 = (data[5] as u32) | (data[6] as u32) << 8;
            let v3 = (data[7] as u32) | (data[8] as u32) << 8 | (data[9] as u32) << 16;
            (v0, v1, v2, v3, 10)
        }
        157 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8;
            let v1 = (data[2] as u32)
                | (data[3] as u32) << 8
                | (data[4] as u32) << 16
                | (data[5] as u32) << 24;
            let v2 = (data[6] as u32) | (data[7] as u32) << 8;
            let v3 = (data[8] as u32) | (data[9] as u32) << 8 | (data[10] as u32) << 16;
            (v0, v1, v2, v3, 11)
        }
        158 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8 | (data[2] as u32) << 16;
            let v1 = (data[3] as u32)
                | (data[4] as u32) << 8
                | (data[5] as u32) << 16
                | (data[6] as u32) << 24;
            let v2 = (data[7] as u32) | (data[8] as u32) << 8;
            let v3 = (data[9] as u32) | (data[10] as u32) << 8 | (data[11] as u32) << 16;
            (v0, v1, v2, v3, 12)
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
            (v0, v1, v2, v3, 13)
        }
        160 => {
            let v0 = data[0] as u32;
            let v1 = data[1] as u32;
            let v2 = (data[2] as u32) | (data[3] as u32) << 8 | (data[4] as u32) << 16;
            let v3 = (data[5] as u32) | (data[6] as u32) << 8 | (data[7] as u32) << 16;
            (v0, v1, v2, v3, 8)
        }
        161 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8;
            let v1 = data[2] as u32;
            let v2 = (data[3] as u32) | (data[4] as u32) << 8 | (data[5] as u32) << 16;
            let v3 = (data[6] as u32) | (data[7] as u32) << 8 | (data[8] as u32) << 16;
            (v0, v1, v2, v3, 9)
        }
        162 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8 | (data[2] as u32) << 16;
            let v1 = data[3] as u32;
            let v2 = (data[4] as u32) | (data[5] as u32) << 8 | (data[6] as u32) << 16;
            let v3 = (data[7] as u32) | (data[8] as u32) << 8 | (data[9] as u32) << 16;
            (v0, v1, v2, v3, 10)
        }
        163 => {
            let v0 = (data[0] as u32)
                | (data[1] as u32) << 8
                | (data[2] as u32) << 16
                | (data[3] as u32) << 24;
            let v1 = data[4] as u32;
            let v2 = (data[5] as u32) | (data[6] as u32) << 8 | (data[7] as u32) << 16;
            let v3 = (data[8] as u32) | (data[9] as u32) << 8 | (data[10] as u32) << 16;
            (v0, v1, v2, v3, 11)
        }
        164 => {
            let v0 = data[0] as u32;
            let v1 = (data[1] as u32) | (data[2] as u32) << 8;
            let v2 = (data[3] as u32) | (data[4] as u32) << 8 | (data[5] as u32) << 16;
            let v3 = (data[6] as u32) | (data[7] as u32) << 8 | (data[8] as u32) << 16;
            (v0, v1, v2, v3, 9)
        }
        165 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8;
            let v1 = (data[2] as u32) | (data[3] as u32) << 8;
            let v2 = (data[4] as u32) | (data[5] as u32) << 8 | (data[6] as u32) << 16;
            let v3 = (data[7] as u32) | (data[8] as u32) << 8 | (data[9] as u32) << 16;
            (v0, v1, v2, v3, 10)
        }
        166 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8 | (data[2] as u32) << 16;
            let v1 = (data[3] as u32) | (data[4] as u32) << 8;
            let v2 = (data[5] as u32) | (data[6] as u32) << 8 | (data[7] as u32) << 16;
            let v3 = (data[8] as u32) | (data[9] as u32) << 8 | (data[10] as u32) << 16;
            (v0, v1, v2, v3, 11)
        }
        167 => {
            let v0 = (data[0] as u32)
                | (data[1] as u32) << 8
                | (data[2] as u32) << 16
                | (data[3] as u32) << 24;
            let v1 = (data[4] as u32) | (data[5] as u32) << 8;
            let v2 = (data[6] as u32) | (data[7] as u32) << 8 | (data[8] as u32) << 16;
            let v3 = (data[9] as u32) | (data[10] as u32) << 8 | (data[11] as u32) << 16;
            (v0, v1, v2, v3, 12)
        }
        168 => {
            let v0 = data[0] as u32;
            let v1 = (data[1] as u32) | (data[2] as u32) << 8 | (data[3] as u32) << 16;
            let v2 = (data[4] as u32) | (data[5] as u32) << 8 | (data[6] as u32) << 16;
            let v3 = (data[7] as u32) | (data[8] as u32) << 8 | (data[9] as u32) << 16;
            (v0, v1, v2, v3, 10)
        }
        169 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8;
            let v1 = (data[2] as u32) | (data[3] as u32) << 8 | (data[4] as u32) << 16;
            let v2 = (data[5] as u32) | (data[6] as u32) << 8 | (data[7] as u32) << 16;
            let v3 = (data[8] as u32) | (data[9] as u32) << 8 | (data[10] as u32) << 16;
            (v0, v1, v2, v3, 11)
        }
        170 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8 | (data[2] as u32) << 16;
            let v1 = (data[3] as u32) | (data[4] as u32) << 8 | (data[5] as u32) << 16;
            let v2 = (data[6] as u32) | (data[7] as u32) << 8 | (data[8] as u32) << 16;
            let v3 = (data[9] as u32) | (data[10] as u32) << 8 | (data[11] as u32) << 16;
            (v0, v1, v2, v3, 12)
        }
        171 => {
            let v0 = (data[0] as u32)
                | (data[1] as u32) << 8
                | (data[2] as u32) << 16
                | (data[3] as u32) << 24;
            let v1 = (data[4] as u32) | (data[5] as u32) << 8 | (data[6] as u32) << 16;
            let v2 = (data[7] as u32) | (data[8] as u32) << 8 | (data[9] as u32) << 16;
            let v3 = (data[10] as u32) | (data[11] as u32) << 8 | (data[12] as u32) << 16;
            (v0, v1, v2, v3, 13)
        }
        172 => {
            let v0 = data[0] as u32;
            let v1 = (data[1] as u32)
                | (data[2] as u32) << 8
                | (data[3] as u32) << 16
                | (data[4] as u32) << 24;
            let v2 = (data[5] as u32) | (data[6] as u32) << 8 | (data[7] as u32) << 16;
            let v3 = (data[8] as u32) | (data[9] as u32) << 8 | (data[10] as u32) << 16;
            (v0, v1, v2, v3, 11)
        }
        173 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8;
            let v1 = (data[2] as u32)
                | (data[3] as u32) << 8
                | (data[4] as u32) << 16
                | (data[5] as u32) << 24;
            let v2 = (data[6] as u32) | (data[7] as u32) << 8 | (data[8] as u32) << 16;
            let v3 = (data[9] as u32) | (data[10] as u32) << 8 | (data[11] as u32) << 16;
            (v0, v1, v2, v3, 12)
        }
        174 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8 | (data[2] as u32) << 16;
            let v1 = (data[3] as u32)
                | (data[4] as u32) << 8
                | (data[5] as u32) << 16
                | (data[6] as u32) << 24;
            let v2 = (data[7] as u32) | (data[8] as u32) << 8 | (data[9] as u32) << 16;
            let v3 = (data[10] as u32) | (data[11] as u32) << 8 | (data[12] as u32) << 16;
            (v0, v1, v2, v3, 13)
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
            (v0, v1, v2, v3, 14)
        }
        176 => {
            let v0 = data[0] as u32;
            let v1 = data[1] as u32;
            let v2 = (data[2] as u32)
                | (data[3] as u32) << 8
                | (data[4] as u32) << 16
                | (data[5] as u32) << 24;
            let v3 = (data[6] as u32) | (data[7] as u32) << 8 | (data[8] as u32) << 16;
            (v0, v1, v2, v3, 9)
        }
        177 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8;
            let v1 = data[2] as u32;
            let v2 = (data[3] as u32)
                | (data[4] as u32) << 8
                | (data[5] as u32) << 16
                | (data[6] as u32) << 24;
            let v3 = (data[7] as u32) | (data[8] as u32) << 8 | (data[9] as u32) << 16;
            (v0, v1, v2, v3, 10)
        }
        178 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8 | (data[2] as u32) << 16;
            let v1 = data[3] as u32;
            let v2 = (data[4] as u32)
                | (data[5] as u32) << 8
                | (data[6] as u32) << 16
                | (data[7] as u32) << 24;
            let v3 = (data[8] as u32) | (data[9] as u32) << 8 | (data[10] as u32) << 16;
            (v0, v1, v2, v3, 11)
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
            (v0, v1, v2, v3, 12)
        }
        180 => {
            let v0 = data[0] as u32;
            let v1 = (data[1] as u32) | (data[2] as u32) << 8;
            let v2 = (data[3] as u32)
                | (data[4] as u32) << 8
                | (data[5] as u32) << 16
                | (data[6] as u32) << 24;
            let v3 = (data[7] as u32) | (data[8] as u32) << 8 | (data[9] as u32) << 16;
            (v0, v1, v2, v3, 10)
        }
        181 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8;
            let v1 = (data[2] as u32) | (data[3] as u32) << 8;
            let v2 = (data[4] as u32)
                | (data[5] as u32) << 8
                | (data[6] as u32) << 16
                | (data[7] as u32) << 24;
            let v3 = (data[8] as u32) | (data[9] as u32) << 8 | (data[10] as u32) << 16;
            (v0, v1, v2, v3, 11)
        }
        182 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8 | (data[2] as u32) << 16;
            let v1 = (data[3] as u32) | (data[4] as u32) << 8;
            let v2 = (data[5] as u32)
                | (data[6] as u32) << 8
                | (data[7] as u32) << 16
                | (data[8] as u32) << 24;
            let v3 = (data[9] as u32) | (data[10] as u32) << 8 | (data[11] as u32) << 16;
            (v0, v1, v2, v3, 12)
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
            (v0, v1, v2, v3, 13)
        }
        184 => {
            let v0 = data[0] as u32;
            let v1 = (data[1] as u32) | (data[2] as u32) << 8 | (data[3] as u32) << 16;
            let v2 = (data[4] as u32)
                | (data[5] as u32) << 8
                | (data[6] as u32) << 16
                | (data[7] as u32) << 24;
            let v3 = (data[8] as u32) | (data[9] as u32) << 8 | (data[10] as u32) << 16;
            (v0, v1, v2, v3, 11)
        }
        185 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8;
            let v1 = (data[2] as u32) | (data[3] as u32) << 8 | (data[4] as u32) << 16;
            let v2 = (data[5] as u32)
                | (data[6] as u32) << 8
                | (data[7] as u32) << 16
                | (data[8] as u32) << 24;
            let v3 = (data[9] as u32) | (data[10] as u32) << 8 | (data[11] as u32) << 16;
            (v0, v1, v2, v3, 12)
        }
        186 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8 | (data[2] as u32) << 16;
            let v1 = (data[3] as u32) | (data[4] as u32) << 8 | (data[5] as u32) << 16;
            let v2 = (data[6] as u32)
                | (data[7] as u32) << 8
                | (data[8] as u32) << 16
                | (data[9] as u32) << 24;
            let v3 = (data[10] as u32) | (data[11] as u32) << 8 | (data[12] as u32) << 16;
            (v0, v1, v2, v3, 13)
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
            (v0, v1, v2, v3, 14)
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
            (v0, v1, v2, v3, 12)
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
            (v0, v1, v2, v3, 13)
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
            (v0, v1, v2, v3, 14)
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
            (v0, v1, v2, v3, 15)
        }
        192 => {
            let v0 = data[0] as u32;
            let v1 = data[1] as u32;
            let v2 = data[2] as u32;
            let v3 = (data[3] as u32)
                | (data[4] as u32) << 8
                | (data[5] as u32) << 16
                | (data[6] as u32) << 24;
            (v0, v1, v2, v3, 7)
        }
        193 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8;
            let v1 = data[2] as u32;
            let v2 = data[3] as u32;
            let v3 = (data[4] as u32)
                | (data[5] as u32) << 8
                | (data[6] as u32) << 16
                | (data[7] as u32) << 24;
            (v0, v1, v2, v3, 8)
        }
        194 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8 | (data[2] as u32) << 16;
            let v1 = data[3] as u32;
            let v2 = data[4] as u32;
            let v3 = (data[5] as u32)
                | (data[6] as u32) << 8
                | (data[7] as u32) << 16
                | (data[8] as u32) << 24;
            (v0, v1, v2, v3, 9)
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
            (v0, v1, v2, v3, 10)
        }
        196 => {
            let v0 = data[0] as u32;
            let v1 = (data[1] as u32) | (data[2] as u32) << 8;
            let v2 = data[3] as u32;
            let v3 = (data[4] as u32)
                | (data[5] as u32) << 8
                | (data[6] as u32) << 16
                | (data[7] as u32) << 24;
            (v0, v1, v2, v3, 8)
        }
        197 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8;
            let v1 = (data[2] as u32) | (data[3] as u32) << 8;
            let v2 = data[4] as u32;
            let v3 = (data[5] as u32)
                | (data[6] as u32) << 8
                | (data[7] as u32) << 16
                | (data[8] as u32) << 24;
            (v0, v1, v2, v3, 9)
        }
        198 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8 | (data[2] as u32) << 16;
            let v1 = (data[3] as u32) | (data[4] as u32) << 8;
            let v2 = data[5] as u32;
            let v3 = (data[6] as u32)
                | (data[7] as u32) << 8
                | (data[8] as u32) << 16
                | (data[9] as u32) << 24;
            (v0, v1, v2, v3, 10)
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
            (v0, v1, v2, v3, 11)
        }
        200 => {
            let v0 = data[0] as u32;
            let v1 = (data[1] as u32) | (data[2] as u32) << 8 | (data[3] as u32) << 16;
            let v2 = data[4] as u32;
            let v3 = (data[5] as u32)
                | (data[6] as u32) << 8
                | (data[7] as u32) << 16
                | (data[8] as u32) << 24;
            (v0, v1, v2, v3, 9)
        }
        201 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8;
            let v1 = (data[2] as u32) | (data[3] as u32) << 8 | (data[4] as u32) << 16;
            let v2 = data[5] as u32;
            let v3 = (data[6] as u32)
                | (data[7] as u32) << 8
                | (data[8] as u32) << 16
                | (data[9] as u32) << 24;
            (v0, v1, v2, v3, 10)
        }
        202 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8 | (data[2] as u32) << 16;
            let v1 = (data[3] as u32) | (data[4] as u32) << 8 | (data[5] as u32) << 16;
            let v2 = data[6] as u32;
            let v3 = (data[7] as u32)
                | (data[8] as u32) << 8
                | (data[9] as u32) << 16
                | (data[10] as u32) << 24;
            (v0, v1, v2, v3, 11)
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
            (v0, v1, v2, v3, 12)
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
            (v0, v1, v2, v3, 10)
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
            (v0, v1, v2, v3, 11)
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
            (v0, v1, v2, v3, 12)
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
            (v0, v1, v2, v3, 13)
        }
        208 => {
            let v0 = data[0] as u32;
            let v1 = data[1] as u32;
            let v2 = (data[2] as u32) | (data[3] as u32) << 8;
            let v3 = (data[4] as u32)
                | (data[5] as u32) << 8
                | (data[6] as u32) << 16
                | (data[7] as u32) << 24;
            (v0, v1, v2, v3, 8)
        }
        209 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8;
            let v1 = data[2] as u32;
            let v2 = (data[3] as u32) | (data[4] as u32) << 8;
            let v3 = (data[5] as u32)
                | (data[6] as u32) << 8
                | (data[7] as u32) << 16
                | (data[8] as u32) << 24;
            (v0, v1, v2, v3, 9)
        }
        210 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8 | (data[2] as u32) << 16;
            let v1 = data[3] as u32;
            let v2 = (data[4] as u32) | (data[5] as u32) << 8;
            let v3 = (data[6] as u32)
                | (data[7] as u32) << 8
                | (data[8] as u32) << 16
                | (data[9] as u32) << 24;
            (v0, v1, v2, v3, 10)
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
            (v0, v1, v2, v3, 11)
        }
        212 => {
            let v0 = data[0] as u32;
            let v1 = (data[1] as u32) | (data[2] as u32) << 8;
            let v2 = (data[3] as u32) | (data[4] as u32) << 8;
            let v3 = (data[5] as u32)
                | (data[6] as u32) << 8
                | (data[7] as u32) << 16
                | (data[8] as u32) << 24;
            (v0, v1, v2, v3, 9)
        }
        213 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8;
            let v1 = (data[2] as u32) | (data[3] as u32) << 8;
            let v2 = (data[4] as u32) | (data[5] as u32) << 8;
            let v3 = (data[6] as u32)
                | (data[7] as u32) << 8
                | (data[8] as u32) << 16
                | (data[9] as u32) << 24;
            (v0, v1, v2, v3, 10)
        }
        214 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8 | (data[2] as u32) << 16;
            let v1 = (data[3] as u32) | (data[4] as u32) << 8;
            let v2 = (data[5] as u32) | (data[6] as u32) << 8;
            let v3 = (data[7] as u32)
                | (data[8] as u32) << 8
                | (data[9] as u32) << 16
                | (data[10] as u32) << 24;
            (v0, v1, v2, v3, 11)
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
            (v0, v1, v2, v3, 12)
        }
        216 => {
            let v0 = data[0] as u32;
            let v1 = (data[1] as u32) | (data[2] as u32) << 8 | (data[3] as u32) << 16;
            let v2 = (data[4] as u32) | (data[5] as u32) << 8;
            let v3 = (data[6] as u32)
                | (data[7] as u32) << 8
                | (data[8] as u32) << 16
                | (data[9] as u32) << 24;
            (v0, v1, v2, v3, 10)
        }
        217 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8;
            let v1 = (data[2] as u32) | (data[3] as u32) << 8 | (data[4] as u32) << 16;
            let v2 = (data[5] as u32) | (data[6] as u32) << 8;
            let v3 = (data[7] as u32)
                | (data[8] as u32) << 8
                | (data[9] as u32) << 16
                | (data[10] as u32) << 24;
            (v0, v1, v2, v3, 11)
        }
        218 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8 | (data[2] as u32) << 16;
            let v1 = (data[3] as u32) | (data[4] as u32) << 8 | (data[5] as u32) << 16;
            let v2 = (data[6] as u32) | (data[7] as u32) << 8;
            let v3 = (data[8] as u32)
                | (data[9] as u32) << 8
                | (data[10] as u32) << 16
                | (data[11] as u32) << 24;
            (v0, v1, v2, v3, 12)
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
            (v0, v1, v2, v3, 13)
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
            (v0, v1, v2, v3, 11)
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
            (v0, v1, v2, v3, 12)
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
            (v0, v1, v2, v3, 13)
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
            (v0, v1, v2, v3, 14)
        }
        224 => {
            let v0 = data[0] as u32;
            let v1 = data[1] as u32;
            let v2 = (data[2] as u32) | (data[3] as u32) << 8 | (data[4] as u32) << 16;
            let v3 = (data[5] as u32)
                | (data[6] as u32) << 8
                | (data[7] as u32) << 16
                | (data[8] as u32) << 24;
            (v0, v1, v2, v3, 9)
        }
        225 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8;
            let v1 = data[2] as u32;
            let v2 = (data[3] as u32) | (data[4] as u32) << 8 | (data[5] as u32) << 16;
            let v3 = (data[6] as u32)
                | (data[7] as u32) << 8
                | (data[8] as u32) << 16
                | (data[9] as u32) << 24;
            (v0, v1, v2, v3, 10)
        }
        226 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8 | (data[2] as u32) << 16;
            let v1 = data[3] as u32;
            let v2 = (data[4] as u32) | (data[5] as u32) << 8 | (data[6] as u32) << 16;
            let v3 = (data[7] as u32)
                | (data[8] as u32) << 8
                | (data[9] as u32) << 16
                | (data[10] as u32) << 24;
            (v0, v1, v2, v3, 11)
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
            (v0, v1, v2, v3, 12)
        }
        228 => {
            let v0 = data[0] as u32;
            let v1 = (data[1] as u32) | (data[2] as u32) << 8;
            let v2 = (data[3] as u32) | (data[4] as u32) << 8 | (data[5] as u32) << 16;
            let v3 = (data[6] as u32)
                | (data[7] as u32) << 8
                | (data[8] as u32) << 16
                | (data[9] as u32) << 24;
            (v0, v1, v2, v3, 10)
        }
        229 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8;
            let v1 = (data[2] as u32) | (data[3] as u32) << 8;
            let v2 = (data[4] as u32) | (data[5] as u32) << 8 | (data[6] as u32) << 16;
            let v3 = (data[7] as u32)
                | (data[8] as u32) << 8
                | (data[9] as u32) << 16
                | (data[10] as u32) << 24;
            (v0, v1, v2, v3, 11)
        }
        230 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8 | (data[2] as u32) << 16;
            let v1 = (data[3] as u32) | (data[4] as u32) << 8;
            let v2 = (data[5] as u32) | (data[6] as u32) << 8 | (data[7] as u32) << 16;
            let v3 = (data[8] as u32)
                | (data[9] as u32) << 8
                | (data[10] as u32) << 16
                | (data[11] as u32) << 24;
            (v0, v1, v2, v3, 12)
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
            (v0, v1, v2, v3, 13)
        }
        232 => {
            let v0 = data[0] as u32;
            let v1 = (data[1] as u32) | (data[2] as u32) << 8 | (data[3] as u32) << 16;
            let v2 = (data[4] as u32) | (data[5] as u32) << 8 | (data[6] as u32) << 16;
            let v3 = (data[7] as u32)
                | (data[8] as u32) << 8
                | (data[9] as u32) << 16
                | (data[10] as u32) << 24;
            (v0, v1, v2, v3, 11)
        }
        233 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8;
            let v1 = (data[2] as u32) | (data[3] as u32) << 8 | (data[4] as u32) << 16;
            let v2 = (data[5] as u32) | (data[6] as u32) << 8 | (data[7] as u32) << 16;
            let v3 = (data[8] as u32)
                | (data[9] as u32) << 8
                | (data[10] as u32) << 16
                | (data[11] as u32) << 24;
            (v0, v1, v2, v3, 12)
        }
        234 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8 | (data[2] as u32) << 16;
            let v1 = (data[3] as u32) | (data[4] as u32) << 8 | (data[5] as u32) << 16;
            let v2 = (data[6] as u32) | (data[7] as u32) << 8 | (data[8] as u32) << 16;
            let v3 = (data[9] as u32)
                | (data[10] as u32) << 8
                | (data[11] as u32) << 16
                | (data[12] as u32) << 24;
            (v0, v1, v2, v3, 13)
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
            (v0, v1, v2, v3, 14)
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
            (v0, v1, v2, v3, 12)
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
            (v0, v1, v2, v3, 13)
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
            (v0, v1, v2, v3, 14)
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
            (v0, v1, v2, v3, 15)
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
            (v0, v1, v2, v3, 10)
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
            (v0, v1, v2, v3, 11)
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
            (v0, v1, v2, v3, 12)
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
            (v0, v1, v2, v3, 13)
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
            (v0, v1, v2, v3, 11)
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
            (v0, v1, v2, v3, 12)
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
            (v0, v1, v2, v3, 13)
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
            (v0, v1, v2, v3, 14)
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
            (v0, v1, v2, v3, 12)
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
            (v0, v1, v2, v3, 13)
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
            (v0, v1, v2, v3, 14)
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
            (v0, v1, v2, v3, 15)
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
            (v0, v1, v2, v3, 13)
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
            (v0, v1, v2, v3, 14)
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
            (v0, v1, v2, v3, 15)
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
            (v0, v1, v2, v3, 16)
        } // END
    }
}
