mod decoder;
use decoder::decode_block;

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
        assert_eq!(data, [0, 0xff, 0xff, 0xff, 0xff]);
    }

    #[test]
    fn tst3() {
        let data = compress([0xffff, 2, 3, 4]);
        assert_eq!(data, [1, 0xff, 0xff, 2, 3, 4]);

        let data = compress([0xff, 0xff01, 2, 3]);
        assert_eq!(data, [1 << 2, 0xff, 1, 0xff, 2, 3]);

        let data = compress([0xff, 0xff, 0xff01, 0]);
        assert_eq!(data, [1 << 4, 0xff, 0xff, 1, 0xff, 0]);

        let data = compress([0xff, 0xff, 0xff, 0xff03]);
        assert_eq!(data, [1 << 6, 0xff, 0xff, 0xff, 3, 0xff]);
    }

    #[test]
    fn tst4() {
        let data = compress([1, 2, 3, 4, 5, 6, 7, 8]);
        assert_eq!(data, [0, 1, 2, 3, 4, 0, 5, 6, 7, 8]);
    }

    #[test]
    fn tst5() {
        let data = compress([1, 2, 3, 4, 0xff05, 6, 7, 8]);
        assert_eq!(data, [0, 1, 2, 3, 4, 1, 5, 0xff, 6, 7, 8]);
    }


    #[test]
    fn tst6() {
        let data = compress([1, 2, 3, 4, 0xaabbccdd, 6, 7, 8]);
        assert_eq!(data, [
            // block 1
            0b00, 1, 2, 3, 4, 
            // block 2
            0b11, 0xdd, 0xcc, 0xbb, 0xaa, 6, 7, 8]);
    }

    #[test]
    fn tst10() {
        let data = [
            732734234u32,
            213213213,
            32,
            2314324,
            3243,
            12,
            32432,
            5435,
            4356,
            57,
            657,
            6546,
            32,
            4,
            3245,
            67,
            65,
            432,
            465,
            7,
            643,
            542,
            5424,
            2432,
            4,
            324,
            324,
            326,
            765,
            7534,
            646546546,
            45654,
            6456,
            546,
            546,
            546,
            546,
            546,
            5462,
            22222222,
            5637426,
            5356790,
            98765432,
            34567,
            6544567,
            6543245,
            6543,
            45678,
            76543,
            45678,
            765,
            3467890,
            9876,
            5432,
            345,
            0,
        ];

        let compressed = compress(data.iter().cloned());

        let dec = DataBlockIter { data: &compressed };

        let newdata = dec.collect();

        assert_eq!(newdata, data);
    }
}

pub struct DataBlockIter<'a> {
    data: &'a [u8],
}

impl<'a> DataBlockIter<'a> {
    pub fn collect(self) -> Vec<u32> {
        let mut v = Vec::new();

        for [a, b, c, d] in self {
            v.push(a);
            v.push(b);
            v.push(c);
            v.push(d);
        }

        v
    }
}

impl<'a> Iterator for DataBlockIter<'a> {
    type Item = [u32; 4];

    fn next(&mut self) -> Option<Self::Item> {
        if self.data.len() < 5 {
            return None;
        }

        let v = self.data[0];
        let data = &self.data[1..];

        let (a, b, c, d, offset) = decode_block(v, data);

        self.data = &self.data[offset..];

        Some([a, b, c, d])
    }
}

pub fn decompress<'a>(data: &'a [u8]) -> DataBlockIter<'a> {
    DataBlockIter { data }
}

pub fn compress(iter: impl IntoIterator<Item = u32>) -> Vec<u8> {
    let mut buffer = Vec::new();
    let iter = iter.into_iter();
    for mut chunk in (Chunk { iter }) {
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
    } else if v <= 0xffffff {
        2
    } else {
        3
    }
}

struct Chunk<I>
where
    I: Iterator<Item = u32>,
{
    iter: I,
}

impl<I> Iterator for Chunk<I>
where
    I: Iterator<Item = u32>,
{
    type Item = Vec<u32>;

    fn next(&mut self) -> Option<Self::Item> {
        let a = self.iter.next();
        let b = self.iter.next();
        let c = self.iter.next();
        let d = self.iter.next();

        match (a, b, c, d) {
            (None, _, _, _) => None,
            (Some(a), None, _, _) => Some(vec![a, 0, 0, 0]),
            (Some(a), Some(b), None, _) => Some(vec![a, b, 0, 0]),
            (Some(a), Some(b), Some(c), None) => Some(vec![a, b, c, 0]),
            (Some(a), Some(b), Some(c), Some(d)) => Some(vec![a, b, c, d]),
        }
    }
}
