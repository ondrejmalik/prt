pub mod simd {
    #[target_feature(enable = "avx2")]
    pub unsafe fn square(a: &Vec<i32>, b: &Vec<i32>, c: &mut Vec<i32>) {
        for (i, res) in c.iter_mut().enumerate() {
            *res = a[i] + b[i];
        }
    }

    pub unsafe fn square_basic(a: &Vec<i32>, b: &Vec<i32>, c: &mut Vec<i32>) {
        for (i, res) in c.iter_mut().enumerate() {
            *res = a[i] + b[i];
        }
    }
}
