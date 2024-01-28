fn main() {
    let signal = vec![0.0, 1.0, 0.0, 1.0, 7.0 ,9.0]; // Input signal
    let spectrum = algori::math::dft(&signal);
    
    for (k, value) in spectrum.iter().enumerate() {
            println!("Frequency {}: {:?}", k, value);
	}	
}
