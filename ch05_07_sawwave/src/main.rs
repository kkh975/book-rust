use hound;
use rand::prelude::*;

const SAMPLE_RATE: f32 = 44100.0;

fn main() {
    let bpm = 120;

    // 톱니파 멜로디 생성
    let mut wav: Vec<f32> = vec![];
    wav.extend(sawtooth_wave(60, calc_len(bpm, 4), 0.5)); // 도
    wav.extend(sawtooth_wave(64, calc_len(bpm, 4), 0.5)); // 미
    wav.extend(sawtooth_wave(67, calc_len(bpm, 4), 0.5)); // 솔
    write_file("saw.wav", wav);

    // 방형파 멜로디 생성
    let mut wav: Vec<f32> = vec![];
    [60,64,67,64, 60,64,67,72].iter().for_each(|no| {
        wav.extend(square_wave(*no, calc_len(bpm, 8), 0.5));
    });
    write_file("sq.wav", wav);

    // 삼각형 멜로디 생성
    let mut wav: Vec<f32> = vec![];
    [60,64,67,64, 60,64,67,72].iter().for_each(|no| {
        wav.extend(tri_wave(*no, calc_len(bpm, 8), 0.5));
    });
    write_file("tri.wav", wav);

    // 노이즈 생성
    let mut wav: Vec<f32> = vec![];
    wav.extend(noise_wave(2.0, -1.0, calc_len(bpm, 2), 0.5));
    wav.extend(noise_wave(0.2, 0.8, calc_len(bpm, 2), 0.5));
    wav.extend(noise_wave(0.8, -1.0, calc_len(bpm, 2), 0.5));
    write_file("noise.wav", wav);

    // 펄스파 생성
    let mut wav: Vec<f32> = vec![];
    [0.3, 0.1, 0.7, 0.5].iter().for_each(|duty| {
        [60,64,67,72].iter().for_each(|no| {
            wav.extend(pulse_wave(*no, calc_len(bpm, 4), 0.5, *duty));
        });
    });
    write_file("pulse.wav", wav);
}

fn write_file(filename: &str, wav: Vec<f32>) {
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: SAMPLE_RATE as u32,
        bits_per_sample: 32,
        sample_format: hound::SampleFormat::Float,
    };
    let mut fw = hound::WavWriter::create(filename, spec).unwrap();

    for v in wav.into_iter() {
        fw.write_sample(v).unwrap();
        // println!("{}", v);
    }
}

// 노트번호를 주파수로
fn noteno_to_hz(no: i32) -> f32 {
    440.0 * 2.0f32.powf((no - 69) as f32 / 12.0)
}

// n 음표 샘플 수를 계산
fn calc_len(bpm: usize, n: usize) -> usize {
    let base_len = (60.0 / bpm as f32) * SAMPLE_RATE;
    ((4.0 / n as f32) * base_len) as usize
}

////////////////////////////////////////////////////

// 톱니파 생성
fn sawtooth_wave(noteno: i32, len: usize, gain: f32) -> Vec<f32> {
    let tone = noteno_to_hz(noteno); // 주파수 얻기
    let form_samples = SAMPLE_RATE / tone; // 주기 얻기
    let mut wav = vec![0.0; len];

    for i in 0..len {
        let pif = (i as f32 / form_samples) % 1.0;
        wav[i] = pif * 2.0 - 1.0;
    }

    // 음량 조절
    wav.into_iter().map(|v| (v * gain) as f32).collect()
}

// 방형파 생성
fn square_wave(noteno: i32, len: usize, gain: f32) -> Vec<f32> {
    let tone = noteno_to_hz(noteno); // 주파수 얻기
    let form_samples = SAMPLE_RATE / tone; // 주기 얻기
    let mut wav = vec![0.0; len];

    let half_fs = (form_samples / 2.0) as usize;
    for i in 0..len {
        let hl = (i / half_fs) % 2;
        wav[i] = if hl == 0 { -1.0 } else { 1.0 };
    }

    // 음량 조절
    wav.into_iter().map(|v| (v * gain) as f32).collect()
}

// 삼각파 생성
fn tri_wave(noteno: i32, len: usize, gain: f32) -> Vec<f32> {
    let tone = noteno_to_hz(noteno); // 주파수 얻기
    let form_samples = SAMPLE_RATE / tone; // 주기 얻기
    let mut wav = vec![0.0; len];

    let half_fs = form_samples / 2.0; //주기를 절반으로 나눔
    for i in 0..len {
        let hi = i as f32 / half_fs;
        let mut v: f32 = 2.0 * (hi % 1.0) - 1.0;
        let is_climbing = hi.floor() as usize % 2 == 0;
        v = if is_climbing { v } else { -v };
        wav[i] = v;
    }

    // 음량 조절
    wav.into_iter().map(|v| (v * gain) as f32).collect()
}

// 펄스파 생성
fn noise_wave(range: f32, shift: f32, len: usize, gain: f32) -> Vec<f32> {
    let mut wav = vec![0.0; len];
    let mut rng = rand::thread_rng();

    for i in 0..len {
        wav[i] = rng.gen::<f32>() * range + shift;
    }

    // 음량 조절
    wav.into_iter().map(|v| (v * gain) as f32).collect()
}

// 펄스파 생성
fn pulse_wave(noteno: i32, len: usize, gain: f32, duty: f32) -> Vec<f32> {
    let tone = noteno_to_hz(noteno); // 주파수 얻기
    let form_samples = SAMPLE_RATE / tone; // 주기 얻기
    let mut wav = vec![0.0; len];

    for i in 0..len {
        let saw = (i as f32 / form_samples) % 1.0;
        wav[i] = if saw > duty { -1.0 } else { 1.0 };
    }

    // 음량 조절
    wav.into_iter().map(|v| (v * gain) as f32).collect()
}