macro_rules! recurrence {
    ($a:ident[$n:ident] $($t:ty)? = $($ini:expr),+ ; $eval:expr ) => {
        {
            struct ArrayRingBufferWithFakeIndices<T: Copy, const N: usize> {
                vals: [T; N],
                fake_size: usize,
            }
            impl<T: Copy, const N: usize> ArrayRingBufferWithFakeIndices<T, N> {
                fn new(vals: [T; N]) -> Self {
                    Self { vals, fake_size: N }
                }
                fn push(&mut self, val: T) -> T {
                    self.vals[self.fake_size % N] = val;
                    self.fake_size += 1;
                    val
                }
            }
            impl<T: Copy, const N: usize> ::std::ops::Index<usize> for ArrayRingBufferWithFakeIndices<T, N> {
                type Output = T;
                fn index(&self, index: usize) -> &T {
                    // TODO: check the validity of the index
                    &self.vals[index % N]
                }
            }
            impl<T: Copy, const N: usize> ::std::ops::IndexMut<usize> for ArrayRingBufferWithFakeIndices<T, N> {
                fn index_mut(&mut self, index: usize) -> &mut T {
                    // TODO: check the validity of the index
                    &mut self.vals[index % N]
                }
            }

            let mut $a =
                ArrayRingBufferWithFakeIndices$(::<$t, _>)?::new([$($ini),*]);

            let ini_size = $a.fake_size; // TODO: make this const?

            let mut $n = 0;
            ::std::iter::from_fn(move || {
                let result = if $n < ini_size {
                    Some($a[$n])
                } else {
                    Some($a.push($eval))
                };
                $n += 1;
                result
            })
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let fib = recurrence![a[n] = 0, 1; a[n-1] + a[n-2]];

        for n in fib.take(10) { print!("{} ", n); }
        // 0 1 1 2 3 5 8 13 21 34

        let other = recurrence![f[i] = 1.0; f[i-1] * i as f64];
        for n in other.take(10) { print!("{} ", n); }
        //1 1 2 6 24 120 720 5040 40320 362880
    }
}
