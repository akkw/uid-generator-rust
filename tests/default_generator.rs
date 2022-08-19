use chrono::{DateTime, Local, NaiveDateTime, TimeZone};
use log::info;
use uid_generator_rust::default_generator::{DefaultUidGenerator, InteriorDefaultUidGenerator, UidGeneratorBuild};
use uid_generator_rust::metadata_storage::Config;
use uid_generator_rust::UidGenerator;

#[test]
fn uid() -> Result<(),String>{

    let date = Local.ymd(2022, 8, 18).and_hms(0,0,0);
    let epoch_seconds = date.to_string().parse::<DateTime<Local>>().unwrap().timestamp();
    let config = Config::from(("127.0.0.1", 9706), "root", "root");
    let build = UidGeneratorBuild::form(Some(29), Some(21), Some(13), Some(date), Some(epoch_seconds), config);
    let mut generator = DefaultUidGenerator::from(build);

    let result = generator.get_uid()?;
    println!("{}",result);
    let result = generator.get_uid()?;
    println!("{}",result);
    let result = generator.get_uid()?;
    println!("{}",result);
    let result = generator.get_uid()?;
    println!("{}",result);
    Result::Ok(())
}