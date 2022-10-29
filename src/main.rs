mod harmonics {
    pub fn calc_overtones(fund: f64) -> Vec<f64> {
        let mut vec: Vec<f64> = vec![];
        let mut count = 2.0;
        let mut current_partial: f64 = fund;
        while current_partial < 2000.0 {
            current_partial = fund * count;
            vec.push(current_partial);
            count += 1.0;
        }
        return vec;
    }

    pub fn calc_undertones(fund: f64) -> Vec<f64> {
        let mut vec: Vec<f64> = vec![];
        let mut count = 2.0;
        let mut current_partial: f64 = fund;
        while current_partial > 20.0 {
            current_partial = fund / count;
            vec.push(current_partial);
            count += 1.0;
        }
        return vec;
    }

    pub fn calc_octaves(mut current_octave: f64) -> Vec<f64> {
        let mut vec: Vec<f64> = vec![];
        while current_octave < 20000.0 {
            current_octave *= 2.0;
            vec.push(current_octave);
        }
        return vec;
    }
}

mod scales {
    pub fn calc_et_scale(fund: f64) -> Vec<f64> {
        let mut degrees: Vec<f64> = vec![];
        degrees.push(fund);
        for i in 1..12 {
            degrees.push(fund * 2.0f64.powf(i as f64 / 12.0));
        }
        return degrees;
    }

    pub fn calc_et_major_scale(fund: f64) -> Vec<f64> {
        let mut degrees: Vec<f64> = vec![];
        let full_scale = calc_et_scale(fund);
        degrees.push(full_scale[0]);
        degrees.push(full_scale[2]);
        degrees.push(full_scale[4]);
        degrees.push(full_scale[5]);
        degrees.push(full_scale[7]);
        degrees.push(full_scale[9]);
        degrees.push(full_scale[11]);
        return degrees;
    }

    pub fn calc_et_natminor_scale(fund: f64) -> Vec<f64> {
        let mut degrees: Vec<f64> = vec![];
        let full_scale = calc_et_scale(fund);
        degrees.push(full_scale[0]);
        degrees.push(full_scale[2]);
        degrees.push(full_scale[3]);
        degrees.push(full_scale[5]);
        degrees.push(full_scale[7]);
        degrees.push(full_scale[8]);
        degrees.push(full_scale[10]);
        return degrees;
    }
}

fn main() {
    // let overtones: Vec<f64> = harmonics::calc_overtones(55.0);
    // for i in overtones.into_iter() {
    //     println!("{}", i);
    // }
    // let undertones: Vec<f64> = harmonics::calc_undertones(220.0);
    // for i in undertones.into_iter() {
    //     println!("{}", i);
    // }
    // let octaves: Vec<f64> = harmonics::calc_octaves(55.0);
    // for i in octaves.into_iter() {
    //     println!("{}", i);
    // }
    let scale: Vec<f64> = scales::calc_et_natminor_scale(65.41);
    for i in scale.into_iter() {
        println!("{}", i);
    }
}
