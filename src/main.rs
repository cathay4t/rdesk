mod idasen;

use crate::idasen::get_instance_by_mac;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut argv: Vec<String> = std::env::args().collect();

    let desk = get_instance_by_mac("E9:FC:8C:07:4C:E7").await?;

    if argv.len() >= 2 {
        desk.move_to_with_progress(argv.remove(1).as_str().parse::<u16>()?)
            .await?;
    } else {
        let position = desk.position().await?;
        println!("Current position: {} {}cm", position, position / 100);
    }
    Ok(())
}
