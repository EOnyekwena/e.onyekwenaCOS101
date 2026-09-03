fn main() {
	let toshiba: f64 = 450_000.0;
	let mac: f64 = 1_500_000.0;
	let hp: f64 = 750_000.0;
	let dell: f64 = 2_850_000.0;
	let acer: f64 = 250_000.0;

	//sum
	let s = toshiba + mac + hp + dell + acer;
	println!("Sum is {}", s);

	//average
	let av = (toshiba + mac + hp + dell + acer) / 5.0;
	println!("Average is {}", av);
}