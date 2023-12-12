use core::ops::{Index, IndexMut};

#[derive(Debug)]
pub struct RingBuffer<T, const N: usize> {
    buffer: [Option<T>; N],
    current: usize,
}

impl<T, const N: usize> RingBuffer<T, N> {
    pub fn wrapping_add(a: usize, b: usize) -> usize {
        match a.overflowing_add(b % N) {
            (i, true) => i.wrapping_sub(N) % N,
            (i, false) => i % N,
        }
    }

    pub fn wrapping_add_isize(a: usize, b: isize) -> usize {
        match a.overflowing_add(b.rem_euclid(N as isize) as usize) {
            (i, true) => i % N,
            (i, false) => i.wrapping_add(N) % N,
        }
    }

    pub fn get_isize(&self, index: isize) -> Option<&T> {
        self.buffer[Self::wrapping_add_isize(self.current, index)].as_ref()
    }

    pub fn get_mut_isize(&mut self, index: isize) -> Option<&mut T> {
        self.buffer[Self::wrapping_add_isize(self.current, index)].as_mut()
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        self.buffer[Self::wrapping_add(self.current, index)].as_ref()
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        self.buffer[Self::wrapping_add(self.current, index)].as_mut()
    }

    pub fn push(&mut self, value: Option<T>) -> Option<T> {
        self.current = self.current.wrapping_add(1) % N;

        std::mem::replace(&mut self.buffer[self.current], value)
    }
}

impl<T, const N: usize> Default for RingBuffer<T, N> {
    fn default() -> Self {
        Self {
            buffer: std::array::from_fn(|_| Default::default()),
            current: N - 1,
        }
    }
}

impl<T, const N: usize> Index<usize> for RingBuffer<T, N> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        self.get(index).unwrap()
    }
}

impl<T, const N: usize> IndexMut<usize> for RingBuffer<T, N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.get_mut(index).unwrap()
    }
}

impl<T, const N: usize> Index<isize> for RingBuffer<T, N> {
    type Output = T;

    fn index(&self, index: isize) -> &Self::Output {
        self.get_isize(index).unwrap()
    }
}

impl<T, const N: usize> IndexMut<isize> for RingBuffer<T, N> {
    fn index_mut(&mut self, index: isize) -> &mut Self::Output {
        self.get_mut_isize(index).unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_index() {
        let mut buffer = RingBuffer::<_, 3>::default();

        for i in 0..3 {
            buffer.push(i);
        }

        assert_eq!(buffer[-1_isize], 1);
        assert_eq!(buffer[-2_isize], 0);

        for index in 0_usize..=100 {
            assert_eq!(buffer[index], (index + 2) % 3, "Failed on index {index}");
        }

        for index in -100_isize..=100 {
            assert_eq!(buffer[index], (index + 2).rem_euclid(3) as usize, "Failed on index {index}");
        }

        assert_eq!(buffer[usize::MAX], (usize::MAX % 3) + 2);

        buffer.push(0);

        for index in 0_usize..=100 {
            assert_eq!(buffer[index], index % 3, "Failed on index {index}");
        }

        for index in -4_isize..=100 {
            assert_eq!(buffer[index], index.rem_euclid(3) as usize, "Failed on index {index}");
        }

        assert_eq!(buffer[usize::MAX], usize::MAX % 3)
    }
}
