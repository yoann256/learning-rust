use rand::Rng;

pub fn add(x: i32, y: i32) -> i32 {
	return x + y;
}

pub fn sub(x: i32, y: i32) -> i32 {
	return x - y;
}

pub fn multi(x: i32, y: i32) -> i32 {
	return x * y;
}

pub fn div(x: i32, y: i32) -> i32 {
	return x / y;
}

pub fn rand(min: i32, max: i32) -> i32 {
	return rand::thread_rng().gen_range(min..add(max,1));
}