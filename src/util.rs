
pub(crate) struct Chunk<I>
where
    I: Iterator<Item = u32>,
{
    pub iter: I,
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


pub(crate) fn var_bits(v: u32) -> u8 {
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
