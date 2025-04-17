
use enigo::{Enigo, Settings};
use xcap::Monitor;
use std::{thread, time::Duration};

fn main() {
    // 1. Initialisation du contrôle de souris
    let mut enigo = Enigo::new(&Settings::default());
    // 2. Initialisation du capture d'écran
    let monitor = Monitor::from_point(10, 10).expect("Erreur création monitor");
    let (video_recorder, sx) = monitor.video_recorder().unwrap();

    // 3. Boucle de capture et déplacement souris (gravité)
    let mut vel_x = 0.0_f64;
    let mut vel_y = 0.0_f64;
    let gravity = 0.5_f64;
    let tick = Duration::from_millis(16);
    video_recorder.start().unwrap();

    loop {
        match sx.recv() {
            Ok(frame) => {
                println!("Size: {:?}, {:?} {}", frame.width, frame.height, frame.raw.len());
                save_frame(&frame.raw, frame.width as usize, frame.height as usize);
                println!("frame: {:?}", frame.width);
            }
            Err(e) => {
                println!("Erreur: {:?}", e);
            }
        }
        thread::sleep(tick);
    }
}

#[allow(dead_code)]
fn save_frame(buf: &[u8], width: usize, height: usize) {
    use image::{ImageBuffer, RgbaImage};
    // Build the image directly from the raw buffer
    let imgbuf: RgbaImage = ImageBuffer::from_raw(width as u32, height as u32, buf.to_vec())
        .expect("Failed to build image buffer");

}
