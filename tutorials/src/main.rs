mod lessons;

use time::{PrimitiveDateTime as DateTime, Date,Time};
use lessons::{*, reverse_string::reverse,hello_world::hello, health_statistics::User, clock::Clock};

fn main() {
    println!("{}", hello());

    println!("{}", reverse("cat"));

    println!("{}", lasagna::expected_minutes_in_oven());
    println!("{}", lasagna::remaining_minutes_in_oven(30));
    println!("{}", lasagna::preparation_time_in_minutes(2));
    println!("{}", lasagna::elapsed_time_in_minutes(2, 30));

    println!("{}", car_assembly::production_rate_per_hour(6));
    println!("{}", car_assembly::working_items_per_minute(6));

    println!("{}", logs::info("info"));
    println!("{}", logs::error("error"));
    println!("{}", logs::warn("warning"));

    println!("{:?}", comparison::sublist(&['a'], &[]));
    println!("{:?}", comparison::sublist(&[], &['a', 's', 'd', 'f']));

    println!("{:?}", comparison::sublist(&['a'], &['a']));
    println!("{:?}", comparison::sublist(&['a'], &['a', 'b']));
    println!("{:?}", comparison::sublist(&['a'], &['v']));

    // println!("{:?}", poker::winning_hands(&["3H 4H 5C 6C JD"]));

    println!("{:?}", gigasecond::after(
        DateTime::new(
            Date::from_calendar_date(2022, 5.try_into().unwrap(), 15).unwrap(),
            Time::from_hms(10, 30, 15).unwrap(),
        )
    ));

    let mut user = User::new(String::from("Bob"), 15, 150.0);

    println!("name: {} age: {} weight: {}",  user.name(), user.age(), user.weight());

    User::set_age(&mut user, 19);
    User::set_weight(&mut user, 170.0);

    println!("name: {} age: {} weight: {}",  user.name(), user.age(), user.weight());

    println!("{:?}",  fibonacci::create_empty());
    println!("{:?}",  fibonacci::create_buffer(10));
    println!("{:?}",  fibonacci::fibonacci());

    println!("sum: {}",  sum_of_multiples::sum_of_multiples(20, &[3,5,6,9,10,12,15,18]));
    println!("sum: {}",  sum_of_multiples::sum_of_multiples(15, &[4,6]));

    println!("1st prime: {}",  prime::nth(1));
    println!("10 th prime: {}",  prime::nth(10));
    println!("100 th prime: {}",  prime::nth(100));

    let div = embedded_game::divmod(30, 5);
    let pos = embedded_game::Position(5, 6);
    let mut iter = embedded_game::evens(2..);

    println!("divisors: ({}, {})", div.0, div.1);
    println!("manhattan distance: {}", pos.manhattan());
    println!("iter next: {}", iter.next().unwrap());
    println!("iter next: {}", iter.next().unwrap());

    let clock = Clock::new(12, 30);
    let clock1 = Clock::new(8, 0);
    println!("clock: {}", clock);
    println!("clock 1: {}", clock1);

    let clock2 = clock.add_minutes(45);

    println!("clock 2: {}", clock2);

    println!("clock 24 hour: {}", Clock::new(24, 0).to_string());
    println!("clock 25 hour: {}", Clock::new(25, 0).to_string());
    println!("clock 2 hour rollover: {}", Clock::new(1, 60).to_string());
    println!("clock minute rollover: {}", Clock::new(0, 160).to_string());  
    println!("clock midnight: {}", Clock::new(72, 8640).to_string());
    println!("clock negative time: {}", Clock::new(-1, 15).to_string());

    let map = hashmap!('a' => 1, 'b' => 2);
    println!("key: 'a' => value: {}", map.get(&'a').unwrap());
}
