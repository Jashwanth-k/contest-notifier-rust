use std::env;
use std::error::Error;

mod clist;
mod green_api;

// console.log("SERVER STARTED");

// const currTimeStr = date.tz(timeZone).format("HH:mm:ss");
//     const areEqual = currTimeStr === triggerTimeStr;
//     if (!areEqual) return;
//     const startDate = moment().tz(timeZone);
//     startDate.hour(startTimeStr.split(":")[0]);
//     startDate.minute(startTimeStr.split(":")[1]);
//     startDate.second(0);
//     const endDate = moment().tz(timeZone);
//     endDate.hour(endTimeStr.split(":")[0]);
//     endDate.minute(endTimeStr.split(":")[1]);
//     endDate.second(0);
//     const todayDate = `${startTimeStr}-${endTimeStr} ${moment(startDate)
//       .tz(timeZone)
//       .format("DD/MM/YYYY")}`;

fn init(date: &String) -> Result<String, Box<dyn Error>> {
    let time_zone = env::var("TIMEZONE")?;
    let start_time_str = env::var("START_TIME")?;
    let end_time_str = env::var("END_TIME")?;
    let trigger_time_str = env::var("TRIGGER_TIME")?;
}

fn main() {
    println!("Hello, world!");
}
