use complex::complex::ComplexType;
mod complex;
mod fft;

fn main() {
    let mut data = Vec::new();
    let mut i: i64 = 1;


    while i <= 64 {
        data.push(ComplexType {
            real: f64::sin(i as f64),
            imag: 0.0
        });
        i += 1;
    }

    println!("LEN: {}", data.len());

    let mut fft_data = fft::fftMod::fft(&data);

    for x in fft_data {
        println!("{}", x.real.to_string());
        
    }
    
}