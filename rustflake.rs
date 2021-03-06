extern crate time;
use std::time::Duration;
 
struct Rustflake
{
	counter: u16,
	host_id: u16,
	last_rollover: u64,
	last_gen: u64,
}

impl Rustflake
{
	pub fn gen(&mut self) -> u64
	{
		// Per https://github.com/twitter/snowflake/
		//  * time - 41 bits (millisecond precision w/ a custom epoch gives us 69 years)
		//  * configured machine id - 10 bits - gives us up to 1024 machines
		//  * sequence number - 12 bits - rolls over every 4096 per machine (with protection to avoid rollover in the same ms)

		let ut = time::get_time();
		let ut64:u64 = ((ut.sec as u64) * std::num::pow(10u64,9)) +
			(ut.nsec as u64);// -
		//	1396646093292870342u64; // 2014-Apr-04 17:15 EST
		if self.last_gen > ut64
		{
			// May not be the best way to deal with this
			std::io::timer::sleep(Duration::seconds((self.last_gen - ut64) as i64));
			return self.gen();
		}
		self.last_gen = ut64;
		let trimed_ut = ut64 & 0xffffffffffc00000;
		
		let trimed_host_id = (self.host_id & 0x03ff) as u64;
		let trimed_counter = (self.counter & 0x0fff) as u64;

		let sfid = trimed_ut + (trimed_host_id << 12) + (trimed_counter);

		//print!("ut : {:t}\n", trimed_ut);
		//print!("hid: {:t}\n", trimed_host_id);
		//print!("cnt: {:t}\n", trimed_counter);
		//print!("sf : {:t}\n", sfid);

		let utu = ut64 / 1000;

		self.counter = (self.counter + 1) % 4096;
		if self.counter == 0
		{
			if self.last_rollover == utu
			{
				// May not be the best way to deal with this
				std::io::timer::sleep(Duration::seconds(1));
				return self.gen();
			};
			self.last_rollover = utu;
		}
		sfid
	}

	fn new(host_id: u16) -> Rustflake
	{
		if  host_id >= 1024  { panic!("Host must be < 1024"); }
		return Rustflake
		{
			counter: 0,
			host_id: host_id,
			last_rollover: 0,
			last_gen: 0,
		};
	}
}



fn main()
{
	let mut s = Rustflake::new(0);
	for _ in range(0, 20u)
	{
		print!("ID: {}\n", s.gen());
	}
}
