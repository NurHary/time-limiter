use std::sync::{Arc, Mutex};
use std::{fs::File, io::BufReader, thread, time::Duration};
use std::{io, usize};

use rodio::Source;

fn main() {
    let (_stream, streamhandler) = rodio::OutputStream::try_default().unwrap();
    let mut sink = rodio::Sink::try_new(&streamhandler).unwrap();
    let lagu1 = File::open("assets/mus1.mp3").unwrap();
    play_music(&mut sink, lagu1);
    let lagu2 = File::open("assets/mus2.mp3").unwrap();
    play_music(&mut sink, lagu2);

    sink.set_volume(0.05);

    //sink.sleep_until_end();
    let sink_arc = Arc::new(Mutex::new(sink));
    let sink_cl = Arc::clone(&sink_arc);

    thread::spawn(move || {
        let mut masukan_saya = String::new();
        loop {
            masukan_saya.clear();
            if let Ok(_) = io::stdin().read_line(&mut masukan_saya) {
                if masukan_saya.trim() == "a" {
                    sink_cl.lock().unwrap().skip_one();
                    println!("len: {:?}", sink_cl.lock().unwrap().len());
                }
                thread::sleep(Duration::from_millis(100))
            }
        }
    });
    while !sink_arc.lock().unwrap().empty() {
        thread::sleep(Duration::from_secs(1));
    }
}

fn play_music(stream: &mut rodio::Sink, music: File) {
    let isi = rodio::Decoder::new(BufReader::new(music)).unwrap();
    stream.append(isi);
}
