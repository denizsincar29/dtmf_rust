mod dtmfdecoder;
use clap::Parser;
use dasp::sample::conv;
use dtmfdecoder::Detector;
use pv_recorder::PvRecorderBuilder;
use std::{sync::mpsc::channel, thread};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct CLI {
    /// Length of the frame in samples
    #[arg(short, long, default_value_t = 512)]
    frame_len: i32,
    /// Output device index. For default one, leave as blank.
    #[arg(short, long, default_value_t = 0)]
    device: i32,
}

fn main() {
    let cli = CLI::parse();
    println!("DTMF decoder, v1.0 by Deniz Sincar.");
    let (tx, rx) = channel();
    let (frametx, framerx) = channel::<Vec<i16>>();
    let (tonetx, tonerx) = channel::<char>();
    ctrlc::set_handler(move || tx.send(()).expect("Could not send signal on channel.")).unwrap();
    let frame_length = cli.frame_len;
    let mut bldr = PvRecorderBuilder::new(frame_length);
    #[cfg(windows)]
    bldr.library_path(std::path::Path::new("libpv_recorder.dll"));
    let recorder = bldr.init().expect("can't init!");
    recorder.set_debug_logging(true);
    println!("Recorder started. Ready to decode DTMF.");
    let sr = recorder.sample_rate();
    let thr = thread::spawn(move || {
        let mut det = Detector::new(sr as u32);
        let mut ot = 'n';
        for frame in framerx {
            let convframe: Vec<f32> = frame.iter().map(|s| conv::i16::to_f32(*s)).collect();
            det.decode(convframe);
            let t = det.last_tone();
            if t != ot {
                if t != 'n' {
                    tonetx.send(t).expect("Can't send to tonetx");
                }
                ot = t;
            }
        }
    }); // thread finished! Wow!
    recorder.start().expect("cannot start rec!");
    while let (Err(_), true) = (rx.try_recv(), recorder.is_recording()) {
        let frame = recorder.read().expect("cannot read!");
        continue;
        frametx.send(frame).expect("cannot send frame!");
        if let Ok(ton) = tonerx.recv() {
            println!("({})", ton);
        }
    }
    drop(frametx); // tell our thread that we're breaking the channel and it must die.
    println!("exitting");
    thr.join().unwrap();
    recorder.stop().unwrap();
}
