use super::{random::Rng, sanity_assert, MUSIC_LENGTH};
use core::arch::asm;
use core::mem;
use windows_sys::Win32::Media::{
    Audio::{
        waveOutOpen, waveOutPrepareHeader, waveOutWrite, CALLBACK_NULL, HWAVEOUT, WAVEFORMATEX,
        WAVEHDR, WAVE_MAPPER,
    },
    Multimedia::WAVE_FORMAT_IEEE_FLOAT,
    MMSYSERR_NOERROR,
};

/// Number of samples per second
const SAMPLE_RATE: usize = 44100;

// Static to avoid having to care about lifetimes
static mut music: [f32; SAMPLE_RATE * MUSIC_LENGTH] = [0.; SAMPLE_RATE * MUSIC_LENGTH];

static wav_format: WAVEFORMATEX = WAVEFORMATEX {
    wFormatTag: WAVE_FORMAT_IEEE_FLOAT as u16, // suspicious...
    nChannels: 1,
    nSamplesPerSec: SAMPLE_RATE as u32,
    nAvgBytesPerSec: (SAMPLE_RATE * mem::size_of::<f32>()) as u32,
    nBlockAlign: 4,
    wBitsPerSample: mem::size_of::<f32>() as u16 * 8,
    cbSize: 0,
};

static mut wav_header: WAVEHDR = WAVEHDR {
    lpData: unsafe { music.as_mut_ptr() } as *mut u8,
    dwBufferLength: (SAMPLE_RATE * MUSIC_LENGTH * mem::size_of::<f32>()) as u32,
    dwBytesRecorded: 0,
    dwUser: 0,
    dwFlags: 0,
    dwLoops: 0,
    lpNext: 0 as *mut WAVEHDR,
    reserved: 0,
};

const FREQUENCIES: [f32; 4] = [
    frequency(A, 3),
    frequency(H, 3),
    frequency(C, 3),
    frequency(D, 3),
];

// Static mut since this is too large for the stack - but we also don't want to
// const-initialize it since that would bloat the binary.
// And due to no-std, a dynamic vector is not an option
static mut instruments: [[f32; SOUND_DURATION * SAMPLE_RATE]; FREQUENCIES.len()] =
    [[0.; SOUND_DURATION * SAMPLE_RATE]; FREQUENCIES.len()];

const SOUND_DURATION: usize = 6;
const N_HARMONICS: usize = 3;

// https://pages.mtu.edu/~suits/notefreqs.html
// This table assumes a speed of sound of 345m/s,
// adjust this accordingly when playing underwater or
// on venus.
const BASE_FREQUENCIES: [f32; 7] = [16.35, 18.35, 20.60, 21.83, 25.50, 27.50, 30.87];
const C: usize = 0;
const D: usize = 1;
const E: usize = 2;
const F: usize = 3;
const G: usize = 4;
const A: usize = 5;
const H: usize = 6;

#[must_use]
const fn frequency(base: usize, n: usize) -> f32 {
    BASE_FREQUENCIES[base] * (1 << n) as f32
}

pub(crate) fn play() {
    // Generate the instrument samples that will be
    // composed together

    for i in 0..FREQUENCIES.len() {
        let instrument = unsafe { &mut instruments[i] };
        let frequency = FREQUENCIES[i];

        // The music guys tell me that this sounds better if i impose harmonic
        // frequencies on top (:
        // let mut harmonic = 1;
        for harmonic in 1..N_HARMONICS + 1 {
            let harmonic_frequency = frequency * ((harmonic as f32 + 1.) / harmonic as f32);

            let mut position: f32 = 0.;
            let sample_duration = harmonic_frequency / SAMPLE_RATE as f32;

            for sample in 0..SOUND_DURATION * SAMPLE_RATE {
                // Create a triangle curve
                if position > 0.5 {
                    position -= 1.;
                }

                let val = unsafe { core::intrinsics::fabsf32(position) } * 4. - 1.;
                instrument[sample] += val / (50. * harmonic as f32);

                position += sample_duration;
            }
        }
    }

    let compose = |second, instrument_index| {
        // Queue the sound
        let instrument: &[f32; SAMPLE_RATE * SOUND_DURATION] =
            unsafe { &instruments[instrument_index] };

        // We multiply the signal with a filter function to fade in/out.
        // The function used is y = (x/4 - 2)(x/4)
        let mut x = 0.;
        for i in 0..instrument.len() {
            let sample = instrument[i];

            let filter = (x - 2.) * (x - 2.) * x;
            unsafe {
                music[second * SAMPLE_RATE + i] += sample * filter;
            }
            x += (SAMPLE_RATE as f32).recip() / 4.;
        }
    };

    // Compose the samples collected
    let mut rng = Rng::default();
    for s in 0..MUSIC_LENGTH - 10 {
        let mut i = 0;
        loop {
            // Handcrafted constant to adjust the number of notes being played
            if rng.next() < 0b00110000000000000000000000000000 {
                compose(s, i);
            }

            i += 1;
            if i == FREQUENCIES.len() {
                break;
            }
        }
    }

    let mut audio_device = 0 as HWAVEOUT;
    unsafe {
        let status = waveOutOpen(
            &mut audio_device,
            WAVE_MAPPER,
            &wav_format,
            0,
            0,
            CALLBACK_NULL,
        );
        sanity_assert!(status == MMSYSERR_NOERROR);

        let status = waveOutPrepareHeader(
            audio_device,
            &mut wav_header,
            mem::size_of::<WAVEHDR>() as u32,
        );
        sanity_assert!(status == MMSYSERR_NOERROR);

        let status = waveOutWrite(
            audio_device,
            &mut wav_header,
            mem::size_of::<WAVEHDR>() as u32,
        );
        sanity_assert!(status == MMSYSERR_NOERROR);
    }
}
