use std::env;
use std::error::Error;
use chrono::{Local, Timelike};
use chrono_tz::Asia::Kolkata;
use dotenv;

mod clist;
mod green_api;

// const data = await clistSerivce(startDate, endDate);
// let messages = data?.map((object) => {
//   let message = "";
//   const startDateStr = moment(object.start).tz(timeZone).format("HH:mm");
//   const endDateStr = moment(object.end).tz(timeZone).format("HH:mm");
//   const duration = Math.floor((object.duration || 0) / 60);
//   message += `
// start : ${startDateStr}
// end : ${endDateStr}
// host : ${object.host}
// contest : ${object.event}
// problems : ${object.n_problems}
// duration : ${duration} min
// `;
//   return message;
// });

// messages.unshift(todayDate);
// messages = messages.join("\n");
// const isSent = await greenAPIService(messages);
// console.log(`Message Status : ${isSent}, Date : ${startDate}`);
// console.log(messages);
// console.log("SERVER STARTED");

#[tokio::main]
async fn init<'a>() -> Result<&'a str, Box<dyn Error>> {
    dotenv::dotenv().ok();
    let start_time_str = env::var("START_TIME")?;
    let end_time_str = env::var("END_TIME")?;
    let trigger_time_str = env::var("TRIGGER_TIME")?;

    let time_now= Local::now().with_timezone(&Kolkata);
    let curr_time_str = time_now.format("%H:%M:%S").to_string();
    if curr_time_str == trigger_time_str {
        return Ok("Not a Trigger Time");
    }

    // constructing start and end dates
    let start_time_hour_minute: Vec<u32>= start_time_str.split(":")
        .map(|x| x.parse::<u32>().unwrap())
        .collect();
    let start_date = Local::now().with_timezone(&Kolkata)
        .with_hour(start_time_hour_minute[0]).unwrap()
        .with_minute(start_time_hour_minute[1]).unwrap();
    let end_time_hour_minute: Vec<u32> = end_time_str.splitn(2, ":")
        .map(|x| x.parse::<u32>().unwrap())
        .collect();
    let end_date = Local::now().with_timezone(&Kolkata)
        .with_hour(end_time_hour_minute[0]).unwrap()
        .with_minute(end_time_hour_minute[1]).unwrap();
    let today_date = String::from(
        start_time_str + 
        "-" + 
        &end_time_str + 
        " " + 
        &time_now.format("%d/%m/%Y").to_string()
    );
    
    let resp_data: Vec<clist::ResponseObject> = clist::main::get_contest_data(start_date, end_date).await?;
    println!("{:#?}", resp_data);
    return Ok("runned");
}

fn main() {
    println!("Server Started");
    match init() {
        Ok(res) => println!("OK RES: {}", res),
        Err(error) => println!("ERROR: {}", error),
    }
}
