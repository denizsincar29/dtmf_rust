mod clipars;
mod dtmfdecoder;
use clap::Parser;
use clipars::CLI;
use dasp::sample::conv;
use dtmfdecoder::Detector;
use pv_recorder::PvRecorderBuilder;
use std::sync::mpsc::channel;

fn main() {
    let cli = CLI::parse();
    let frame_length = cli.frame_len;
    let mut bldr = PvRecorderBuilder::new(frame_length);
    #[cfg(windows)]
    bldr.library_path(std::path::Path::new("libpv_recorder.dll"));
    if cli.list_devices {
        for (i, dev) in bldr
            .get_available_devices()
            .expect("Cannot get devices!")
            .iter()
            .enumerate()
        {
            println!("#{}: {}", i, dev);
        }
        return;
    }
    println!("DTMF decoder, v1.0 by Deniz Sincar.");
    let (tx, rx) = channel::<()>();
    ctrlc::set_handler(move || {
        tx.send(()).expect("Could not send signal on channel.");
        println!("trying to exit!  ")
    })
    .unwrap();
    let recorder = bldr.init().expect("can't init!");
    recorder.set_debug_logging(true);
    println!("Recorder started. Ready to decode DTMF.");
    let sr = recorder.sample_rate();
    let mut det = Detector::new(sr as u32);
    let mut ot = 'n';
    recorder.start().expect("cannot start rec!");
    while recorder.is_recording() {
        let frame = recorder.read().expect("cannot read!");
        let convframe: Vec<f32> = frame.iter().map(|s| conv::i16::to_f32(*s)).collect();
        det.decode(convframe);
        let t = det.last_tone();
        if t != ot {
            if t != 'n' {
                println!("({})", t);
            }
            ot = t;
        }
        if let Ok(_) = rx.try_recv() {
            break;
        }
    }
    println!("exitting");
    recorder.stop().unwrap();
}
