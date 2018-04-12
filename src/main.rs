use exmo_api::ExmoAPI;

mod exmo_api;

fn main() {
    let exmo_api = ExmoAPI::load_from_file();
    exmo_api.print();
}