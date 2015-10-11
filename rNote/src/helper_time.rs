use time;

///Converts a unix timestamp to a Tm structure
/// # Arguments
/// * timestamp - Seconds since 1970 as i64
pub fn to_tm(timestamp: i64) -> time::Tm{
	let mut tm: time::Tm =  time::empty_tm();
	tm.tm_year = 70;
	return tm + time::Duration::seconds(timestamp);
}
///Creates a unix timestamp based on UTC
pub fn unix_utc_now() -> i64{
	time::now_utc().to_timespec().sec
}
