use ndarray::Array1;

pub mod rmvpe;
 
pub fn get_f0_post(f0: ndarray::Array1<f32>, f0_mel_min: f32, f0_mel_max: f32) -> (Array1<i64>, Array1<f32>) {
    let mut f0_mel = f0.mapv(|x| (x / 700.0 + 1.).ln() * 1127.);
    f0_mel.mapv_inplace(|x| if x <= 0. { x } else { (x - f0_mel_min) * 254. / (f0_mel_max - f0_mel_min) + 1. });
    f0_mel.mapv_inplace(|x| x.clamp(1., 255.));
    let f0_coarse = f0_mel.mapv(|x| x.round() as i64);
    (f0_coarse, f0)
}
