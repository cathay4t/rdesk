mod idasen;

use crate::idasen::get_instance_by_mac;

const UP_POSITION: u16 = 10510;
const DOWN_POSITION: u16 = 8490;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut argv: Vec<String> = std::env::args().collect();

    // instantiate the struct, this will attempt to connect to the desk
    // and discover its characteristics
    let desk = get_instance_by_mac("E9:FC:8C:07:4C:E7").await?;

    // alternatively, if there's more than one desk you can get the
    // correct one by it's mac address
    // for some reason, using MAC seems to be more reliable when it
    // comes to device discovering:
    // let desk = get_instance_by_mac("EC:86:F6:44:D3:31")?;

    // move desk to desired position:
    //  minimum: 6200 (62cm), maximum: 12700 (1.27m)

    if argv.len() >= 2 {
        desk.move_to_with_progress(match argv.remove(1).as_str() {
            "up" => UP_POSITION,
            "down" => DOWN_POSITION,
            s => s.parse::<u16>()?,
        })
        .await?;
    } else {
        let position = desk.position().await?;
        println!("Current position: {} {}cm", position, position / 100);
    }
    Ok(())
}
