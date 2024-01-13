extern crate embed_resource;
use std::env;
/*use embed_resource::compile;*/

fn main() {
    let target = env::var("TARGET").unwrap();
    if target.contains("windows") {
        // on windows we will set our game icon as icon for the executable
       /* compile("icon.rc", Vec::<&str>::new());*/
    }
}