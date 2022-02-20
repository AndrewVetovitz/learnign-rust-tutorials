mod lessons;

use time::{PrimitiveDateTime as DateTime, Date,Time};
use lessons::{*, reverse_string::reverse,hello_world::hello, health_statistics::User};

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
}
