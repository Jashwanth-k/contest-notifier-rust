use chrono::{Local, NaiveDateTime, Timelike};
use chrono_tz::Asia::Kolkata;
use dotenv;
use std::error::Error;
use std::time::Duration;
use std::{env, thread};

mod clist;
mod green_api;

#[tokio::main]
async fn init() -> Result<String, Box<dyn Error>> {
    dotenv::dotenv().ok();
    let start_time_str = env::var("START_TIME")?;
    let end_time_str = env::var("END_TIME")?;
    let trigger_time_str = env::var("TRIGGER_TIME")?;

    let time_now = Local::now().with_timezone(&Kolkata);
    let curr_time_str = time_now.format("%H:%M:%S").to_string();
    if curr_time_str != trigger_time_str {
        let resp: String = format!(
            "Curr Time: {}, Trigger Time: {}",
            curr_time_str, trigger_time_str
        );
        return Ok(resp);
        // return Ok(&format!("Curr Time: {}, Trigger Time: {}", curr_time_str, trigger_time_str).as_str());
    }

    // constructing start and end dates
    let start_time_hour_minute: Vec<u32> = start_time_str
        .split(":")
        .map(|x| x.parse::<u32>().unwrap())
        .collect();
    let start_date = Local::now()
        .with_timezone(&Kolkata)
        .with_hour(start_time_hour_minute[0])
        .unwrap()
        .with_minute(start_time_hour_minute[1])
        .unwrap();
    let end_time_hour_minute: Vec<u32> = end_time_str
        .splitn(2, ":")
        .map(|x| x.parse::<u32>().unwrap())
        .collect();
    let end_date = Local::now()
        .with_timezone(&Kolkata)
        .with_hour(end_time_hour_minute[0])
        .unwrap()
        .with_minute(end_time_hour_minute[1])
        .unwrap();
    let today_date = String::from(
        start_time_str + "-" + &end_time_str + " " + &time_now.format("%d/%m/%Y").to_string(),
    );

    let objects: Vec<clist::ResponseObject> =
        clist::main::get_contest_data(start_date, end_date).await?;
    let mut objects: Vec<String> = objects
        .iter()
        .map(|obj| {
            let start_date_str =
                NaiveDateTime::parse_from_str(obj.start.as_ref().unwrap(), "%Y-%m-%dT%H:%M:%S")
                    .unwrap()
                    .format("%H:%M")
                    .to_string();
            let end_date_str =
                NaiveDateTime::parse_from_str(obj.end.as_ref().unwrap(), "%Y-%m-%dT%H:%M:%S")
                    .unwrap()
                    .format("%H:%M")
                    .to_string();
            let duration: u32 = (obj.duration.unwrap() | 0) / 60;
            let message: String = format!(
                "start : {}\nend : {}\nhost : {}\ncontest : {}\nproblems : {}\nduration : {}",
                start_date_str,
                end_date_str,
                obj.host.as_ref().unwrap_or(&"".to_string()),
                obj.event.as_deref().unwrap_or(&"".to_string()),
                obj.n_problems.unwrap_or(0),
                duration,
            );
            return message;
        })
        .collect();
    objects.insert(0, today_date);
    let message_payload: String = objects.join("\n\n");
    let green_api_resp: green_api::ResponseBody =
        green_api::main::send_message(&message_payload).await?;
    println!("green api resp: {:?}", green_api_resp);
    return Ok("Success".to_string());
}

fn main() {
    println!("Server Started");
    loop {
        match init() {
            Ok(res) => println!("OK RES: {}", res),
            Err(error) => println!("ERROR: {}", error),
        }
        thread::sleep(Duration::from_secs(1));
    }
}
