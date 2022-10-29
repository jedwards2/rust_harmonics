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
}

fn main() {
    let overtones: Vec<f64> = harmonics::calc_overtones(55.0);
    // for i in overtones.into_iter() {
    //     println!("{}", i);
    // }
    let undertones: Vec<f64> = harmonics::calc_undertones(220.0);
    // for i in undertones.into_iter() {
    //     println!("{}", i);
    // }
}
