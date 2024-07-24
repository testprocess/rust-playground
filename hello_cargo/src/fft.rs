
pub mod fftMod {
    use crate::complex::{self, complex::ComplexType};

    pub fn fft(mut array: &Vec<ComplexType>) -> Vec<ComplexType> {
        let n: usize = array.len();

        println!("N: {}", n);


        if n == 1 {
            return array.clone(); 
        }

        println!("FALSEN: {}", n);


        let half_n: usize = n/2;

        let mut even: Vec<ComplexType> =  Vec::new();
        let mut odd: Vec<ComplexType> =  Vec::new();
        let mut output: Vec<ComplexType> = vec![ComplexType {
            real: 0.0,
            imag: 0.0
        }; n];

        let mut index = 0;
 
        for value in array {
            if index % 2 == 0 {
                even.push(ComplexType {
                    real: value.real,
                    imag: 0.0
                });
            } else {
                odd.push(ComplexType {
                    real: value.real,
                    imag: 0.0
                });
            }

            index += 1;
        }

        let even_result = fft(&even);
        let odd_result = fft(&odd);

        let mut k: usize = 0;

        while k < half_n {
            let Wk = ComplexType {
                real: 1.0,
                imag: -2.0 * std::f64::consts::PI * (k as f64) / (n as f64)
            };

            let addWkOdd = complex::complex::mul(&Wk, &odd_result[k]);
            let evenPlus: ComplexType = complex::complex::add(&even_result[k], &addWkOdd);
            let evenMinus: ComplexType = complex::complex::sub(&even_result[k], &addWkOdd);

            output[k] = evenPlus;
            output[k+n/2] = evenMinus;
            k += 1;
        }


        return output;
    }
  }


