struct HeEx {
    hot_num: i32,
    cold_num: i32,
    f: f32,
    k: f32,
    q: f32,
    w1: f32,
    w2: f32,
}

impl HeEx {
    fn new(hot_num: i32, cold_num: i32, f: f32, k: f32, q: f32, w1: f32, w2: f32) -> Self {
        Self {
            hot_num,
            cold_num,
            f, k, q,
            w1, w2,
        }
    }
}

fn balance(hot_begin: &f32, cold_end: &f32, he_ex: &mut HeEx) -> (f32, f32) {
    let hot_end = hot_begin - he_ex.w2 * he_ex.q / (he_ex.k * he_ex.f);
    let cold_begin = cold_end - he_ex.w1 * he_ex.q / (he_ex.k * he_ex.f);

    let delta_t = ((hot_begin - hot_end) - (cold_end - cold_begin)) / (log((hot_begin - hot_end) / (cold_end - cold_begin)));

    he_ex.q = he_ex.f * he_ex.k * delta_t;

    (hot_end, cold_begin)
}

fn find_temperature(cold_begin: Vec<f32>, hot_begin: Vec<f32>, he_exes: Vec<&mut HeEx>) -> (Vec<f32>, Vec<f32>) {
    let n = cold_begin.len();
    let m = hot_begin.len();

    let cold_end = vec![0f32; n];
    let hot_end = vec![0f32; m];
    let delta_cold = vec![0f32; n];
    let cold_now = vec![0f32; n];

    loop {
        for i in 0..n {
            cold_end[i] = cold_begin[i] + delta_cold[i];
            cold_now[i] = cold_end[i];
        }

        for j in 0..m {
            hot_end[j] = hot_begin[j];
        }

        for he_ex in he_exes {
            let h_num = he_ex.hot_num;
            let c_num = he_ex.cold_num;

            (hot_end[h_num], cold_now[c_num]) = balance(hot_end[h_num], cold_now[c_num], he_ex);
        }
    }


}

fn main() {
    println!("Hello, world!");
}
