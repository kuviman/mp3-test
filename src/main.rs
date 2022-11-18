use rodio::Source;

fn main() {
    let (_s, handle) = rodio::OutputStream::try_default().unwrap();
    let file = std::io::BufReader::new(std::fs::File::open("static.mp3").unwrap());
    let decoder = rodio::Decoder::new_looped(file).unwrap();
    handle.play_raw(decoder.convert_samples()).unwrap();
    std::thread::sleep(std::time::Duration::from_secs(5));
}
