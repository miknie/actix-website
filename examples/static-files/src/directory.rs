// <directory>
use actix_files as fs;
use actix_web::App;

pub fn main() {
    App::new().service(fs::Files::new("/static", ".").show_files_listing());
}
// </directory>
