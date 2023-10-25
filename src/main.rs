use std::sync::mpsc::channel;
use pv_recorder::PvRecorderBuilder;
use dtmf::{decoder::{Decoder, ToneChange}, enums::{State, Tone}};
use dasp::sample::conv;

struct Tones {
    tone: char,
    tones: String,
}
impl ToneChange for Tones {
    fn tone_change(&mut self, tone: Tone, state: State) {
        match state{
            State::On => {self.tone=tone.as_char(); self.tones.push(self.tone);}
            State::Off => {self.tone='n'}
        }
    }
}

struct Detector {
    samplerate: u32,
    decoder: Decoder<Tones>
}

impl Detector {
    fn new(sr: u32) -> Self {
        Self{samplerate: sr, decoder: Decoder::new(sr, Tones { tone: 'n', tones: "".to_string() })}
    }
    fn last_tone(&self) -> char {self.decoder.tone_change.tone}
    fn tones(&self) -> String {self.decoder.tone_change.tones.clone()}
    
    
    fn decode(&mut self, data: Vec<f32>) {
        self.decoder.process(&data);
    }
}




fn main() {
    println!("DTMF decoder, v1.0 by Deniz Sincar.");
    let (tx, rx)=channel();
    ctrlc::set_handler(move || tx.send(()).expect("Could not send signal on channel.")).unwrap();
    let frame_length = 512;
    let recorder = PvRecorderBuilder::new(frame_length).init().expect("can't init!");
    let mut det=Detector::new(recorder.sample_rate() as u32);
    recorder.start().expect("cannot start rec!");
    println!("Recorder started. Ready to decode DTMF.");
    let mut ot='n';
    while recorder.is_recording() {
        let frame = recorder.read().expect("cannot read!");
        let convframe: Vec<f32>=frame.iter().map(|s| {conv::i16::to_f32(*s)}).collect();
        det.decode(convframe);
        let t=det.last_tone();
        if t!=ot {
            if t!='n' {println!("({})", t);}
            ot=t;
        }
        if let Ok(_) = rx.try_recv() {
            recorder.stop().unwrap();
            println!("exitting");
        }
    }
    
    
}
