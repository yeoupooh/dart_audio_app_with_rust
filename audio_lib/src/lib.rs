use rodio::{Decoder, OutputStream, Source};
use std::ffi::CStr;
use std::os::raw::c_char;
use std::{fs::File, io::BufReader};

#[no_mangle]
pub extern "C" fn play_for_ffi(ptr: *const c_char) {
    let cstr_path = unsafe { CStr::from_ptr(ptr) };
    play(cstr_path.to_str().unwrap());
}

pub fn play(path: &str) {
    // Get a output stream handle to the default physical sound device
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    // Load a sound from a file, using a path relative to Cargo.toml
    let file = BufReader::new(File::open(path).unwrap());
    // Decode that sound file into a source
    let source = Decoder::new(file).unwrap();
    // Play the sound directly on the device
    stream_handle
        .play_raw(source.convert_samples())
        .expect("can't play audio file");

    // The sound plays in a separate audio thread,
    // so we need to keep the main thread alive while it's playing.
    std::thread::sleep(std::time::Duration::from_secs(1));
    println!("play sound in rust");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_play_audio() {
        play("../data/beep-01a.wav");
    }
}
