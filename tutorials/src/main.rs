mod lessons;

use lessons::hello_world::hello;
use lessons::reverse_string::reverse;

fn main() {
    println!("{}", hello());
    println!("{}", reverse("cat"));
}
