mod lessons;

use lessons::hello_world::hello;
use lessons::reverse_string::reverse;
use lessons::lasagna;
use lessons::car_assembly;
use lessons::logs;
use lessons::logs::LogLevel;

fn main() {
    println!("{}", hello());

    println!("{}", reverse("cat"));

    println!("{}", lasagna::expected_minutes_in_oven());
    println!("{}", lasagna::remaining_minutes_in_oven(30));
    println!("{}", lasagna::preparation_time_in_minutes(2));
    println!("{}", lasagna::elapsed_time_in_minutes(2, 30));

    println!("{}", car_assembly::production_rate_per_hour(6));
    println!("{}", car_assembly::working_items_per_minute(6));

    println!("{}", logs::log(LogLevel::Info, "info"));
    println!("{}", logs::log(LogLevel::Error, "error"));
    println!("{}", logs::log(LogLevel::Warning, "warning"));
}
