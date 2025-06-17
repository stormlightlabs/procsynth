//! # Procedural Ambient Music Synthesizer
//!
//! This module implements a proof of concept for a procedural ambient music generator
//! inspired by Brian Eno's ambient music philosophy. The synthesizer creates evolving,
//! atmospheric soundscapes using multiple synthetic voices, modulation, and environmental
//! effects.
//!
//! ## Core Concepts
//!
//! ### Generative Music
//! The synthesizer follows generative music principles where the system creates music
//! algorithmically rather than through pre-composed sequences. Each generation produces
//! unique output based on randomized parameters within controlled ranges.
//!
//! ### Voice Architecture
//! The system uses multiple synthetic voices (oscillators) that:
//! - Generate sine wave tones at slightly different frequencies
//! - Apply Low-Frequency Oscillation (LFO) modulation for movement
//! - Pan across the stereo field using slow oscillations
//! - Contribute to a rich, evolving harmonic texture
//!
//! ### Signal Processing Chain
//! 1. **Voice Synthesis**: Multiple sine wave oscillators with frequency modulation
//! 2. **Modulation**: LFO-based amplitude and frequency modulation for organic movement
//! 3. **Noise Generation**: Multiple layers of filtered noise for texture
//! 4. **Envelope Shaping**: Attack and release phases for smooth transitions
//! 5. **Reverb**: Simple delay-based reverb for spatial depth
//! 6. **Stereo Processing**: Panning and stereo effects for width
//!
//! ### Noise Layers
//! The synthesizer incorporates several types of noise:
//! - **Base Noise**: Continuous low-level noise for subtle texture
//! - **Granular Noise**: Burst-shaped noise modulated by a 10Hz LFO
//! - **Filtered Noise**: Low-pass filtered noise for warmth
//!
//! ## CLI Parameters
//!
//! ### Basic Parameters
//! - `--output (-o)`: Output WAV filename (auto-generated if not specified)
//! - `--duration (-d)`: Length of generated audio in seconds (default: 60.0)
//! - `--sample-rate (-r)`: Audio sample rate in Hz (default: 44100)
//!
//! ### Voice Configuration
//! - `--voices (-v)`: Number of synthetic voices (default: 4)
//! - `--base-freq`: Fundamental frequency in Hz for voice generation (default: 330.0)
//!   - Other voices are derived by random ratios (0.8-1.2x) from this base
//!
//! ### Modulation Parameters
//! - `--lfo-rate-range`: LFO frequency range as "min:max" in Hz (default: "0.05:0.2")
//!   - Controls how fast the amplitude modulation oscillates
//!   - Lower values create slower, more meditative changes
//! - `--mod-depth-range`: Modulation depth range as "min:max" (0.0-1.0) (default: "0.5:1.0")
//!   - Controls how intense the LFO modulation effect is
//!   - Higher values create more dramatic volume changes
//!
//! ### Texture and Dynamics
//! - `--noise-level`: Base noise level (0.0-1.0) (default: 0.005)
//!   - Adds subtle texture and prevents pure digital silence
//! - `--attack`: Fade-in time in seconds (default: 5.0)
//!   - How long it takes for the sound to reach full volume
//! - `--release`: Fade-out time in seconds (default: 10.0)
//!   - How long the ending fade-out lasts
//! - `--reverb-mix`: Dry/wet reverb balance (0.0-1.0) (default: 0.3)
//!   - 0.0 = completely dry, 1.0 = completely wet
//!
//! ## Usage Examples
//!
//! ```bash
//! # Generate a 2-minute ambient piece with default settings
//! cargo run -- --duration 120
//!
//! # Create a minimal drone with fewer voices and less modulation
//! cargo run -- --voices 2 --mod-depth-range "0.1:0.3" --lfo-rate-range "0.01:0.05"
//!
//! # Generate a more textured piece with higher noise levels
//! cargo run -- --noise-level 0.02 --reverb-mix 0.5 --duration 300
//!
//! # Quick test generation (10 seconds)
//! cargo run -- --duration 10 --output test.wav
//! ```
//!
//! ## Technical Implementation
//!
//! ### Audio Generation Process
//! 1. **Parameter Setup**: Parse CLI arguments and convert to internal parameters
//! 2. **Voice Creation**: Generate N voices with randomized frequencies and modulation rates
//! 3. **Sample Generation**: For each sample in the output:
//!    - Calculate each voice's contribution (sine wave + LFO modulation + panning)
//!    - Add multiple noise layers for texture
//!    - Apply envelope shaping (attack/sustain/release)
//!    - Accumulate into stereo output
//! 4. **Post-Processing**: Apply reverb using a simple delay line with feedback
//! 5. **File Output**: Write 16-bit stereo WAV file
//!
//! ### Mathematical Foundations
//! - **Sine Wave Generation**: `sin(2π * frequency * time)`
//! - **LFO Modulation**: `sin(2π * lfo_rate * time) * 0.5 + 0.5` (unipolar)
//! - **Panning**: `pan = sin(2π * pan_rate * time)`, `left = (1-pan)*0.5`, `right = (1+pan)*0.5`
//! - **Envelope**: Linear attack/release with rectangular sustain
//! - **Reverb**: `output = dry * (1-mix) + delayed_feedback * mix`
//!
//! ## References and Inspiration
//!
//! ### Musical References
//! - **Brian Eno**: "Music for Airports" (1978) - foundational ambient music concepts
//! - **Harold Budd & Brian Eno**: "The Plateaux of Mirror" (1980) - textural approaches
//! - **Stars of the Lid**: Masters of drone and ambient minimalism
//! - **Tim Hecker**: Modern ambient music with digital processing techniques
//!
//! ### Technical References
//! - **Curtis Roads**: "Microsound" - granular synthesis and texture concepts
//! - **Miller Puckette**: "The Theory and Technique of Electronic Music" - DSP fundamentals
//! - **Julius O. Smith III**: "Physical Audio Signal Processing" - digital audio theory
//!
//! ### Algorithmic Composition
//! - **Iannis Xenakis**: Stochastic music composition techniques
//! - **John Cage**: Chance operations and indeterminacy in music
//! - **Karlheinz Stockhausen**: Electronic music and spatial audio concepts
//!
//! ### Implementation Techniques
//! - **Low-Frequency Oscillators (LFOs)**: Used for slow modulation of amplitude and frequency
//! - **Additive Synthesis**: Multiple sine waves combined to create complex timbres
//! - **Envelope Generators**: ADSR-style (here simplified to attack/sustain/release)
//! - **Digital Reverb**: Delay lines with feedback for spatial effects
//! - **Procedural Generation**: Algorithmic parameter selection within aesthetic constraints

use clap::{Parser, ValueEnum};
use hound::{WavSpec, WavWriter};
use rand::{rngs::ThreadRng, Rng};
use serde::{Deserialize, Serialize};
use std::{f32::consts::PI, fs::File, io::BufWriter};

/// Ambient WAV generator inspired by Brian Eno
#[derive(Parser, Debug)]
#[command(author, version, about)]
struct CLI {
    /// Load configuration from JSON file (overrides other parameters)
    #[arg(short, long)]
    config: Option<String>,

    /// Output WAV file
    #[arg(short, long)]
    output: Option<String>,

    /// Duration in seconds
    #[arg(short, long, default_value_t = 60.0)]
    duration: f32,

    /// Sample rate (Hz)
    #[arg(short = 'r', long, default_value_t = 44100)]
    sample_rate: u32,

    /// Number of voices
    #[arg(short, long, default_value_t = 4)]
    voices: usize,

    /// Base frequency (Hz) for voices; others derived by random ratio
    #[arg(long, default_value_t = 330.0)]
    base_freq: f32,

    /// LFO modulation rate range (Hz), as min:max
    #[arg(long, default_value = "0.05:0.2")]
    lfo_rate_range: String,

    /// Noise level (0.0 to 1.0)
    #[arg(long, default_value_t = 0.005)]
    noise_level: f32,

    /// LFO modulation depth range (0.0 to 1.0), as min:max
    #[arg(long, default_value = "0.5:1.0")]
    mod_depth_range: String,

    /// Attack time (seconds) for fade in
    #[arg(long, default_value_t = 5.0)]
    attack: f32,

    /// Release time (seconds) for fade out
    #[arg(long, default_value_t = 10.0)]
    release: f32,

    #[arg(long, default_value_t = 0.3)]
    reverb_mix: f32,
}

impl CLI {
    fn to_params(self) -> Result<GeneratorParams, ConfigError> {
        // If config file is specified, load from JSON
        if let Some(config_path) = &self.config {
            return Ok(JsonConfig::from_file(config_path)?.into());
        }

        // Otherwise use CLI parameters
        Ok(self.into())
    }
}

#[derive(Debug, Clone, Copy, ValueEnum)]
enum OutputType {
    MIDI,
    WAV,
}

impl OutputType {
    fn as_str(&self) -> &'static str {
        match self {
            Self::MIDI => "midi",
            Self::WAV => "wav",
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Hound WAV Error")]
    HoundError(#[from] hound::Error),
    #[error("Configuration Error")]
    ConfigError(#[from] ConfigError),
}

/// JSON configuration for ambient synthesis parameters
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct JsonConfig {
    /// Output WAV file
    pub output: Option<String>,
    /// Duration in seconds
    pub duration: f32,
    /// Sample rate (Hz)
    pub sample_rate: u32,
    /// Number of voices
    pub voices: usize,
    /// Base frequency (Hz) for voices
    pub base_freq: f32,
    /// LFO modulation rate range (Hz), as [min, max]
    pub lfo_rate_range: [f32; 2],
    /// Noise level (0.0 to 1.0)
    pub noise_level: f32,
    /// LFO modulation depth range (0.0 to 1.0), as [min, max]
    pub mod_depth_range: [f32; 2],
    /// Attack time (seconds) for fade in
    pub attack: f32,
    /// Release time (seconds) for fade out
    pub release: f32,
    /// Reverb mix level (0.0 to 1.0)
    pub reverb_mix: f32,
}

impl Default for JsonConfig {
    fn default() -> Self {
        JsonConfig {
            output: None,
            duration: 60.0,
            sample_rate: 44100,
            voices: 4,
            base_freq: 330.0,
            lfo_rate_range: [0.05, 0.2],
            noise_level: 0.005,
            mod_depth_range: [0.5, 1.0],
            attack: 5.0,
            release: 10.0,
            reverb_mix: 0.3,
        }
    }
}

impl JsonConfig {
    /// Load configuration from a JSON file
    pub fn from_file<P: AsRef<std::path::Path>>(path: P) -> Result<Self, ConfigError> {
        let content = std::fs::read_to_string(path)?;
        let config: JsonConfig = serde_json::from_str(&content)?;
        Ok(config)
    }

    /// Save configuration to a JSON file
    pub fn to_file<P: AsRef<std::path::Path>>(&self, path: P) -> Result<(), ConfigError> {
        let content = serde_json::to_string_pretty(self)?;
        std::fs::write(path, content)?;
        Ok(())
    }

    /// Create a default configuration file
    pub fn create_default_file<P: AsRef<std::path::Path>>(path: P) -> Result<(), ConfigError> {
        Self::default().to_file(path)
    }

    pub fn to_params(self) -> GeneratorParams {
        self.into()
    }
}

impl Into<GeneratorParams> for JsonConfig {
    fn into(self) -> GeneratorParams {
        GeneratorParams {
            filename: match &self.output {
                Some(output) => format!("{}_{}", v4_uuid(), output.to_string()),
                None => generate_filename("ambient", OutputType::WAV),
            },
            sample_rate: self.sample_rate,
            duration: self.duration,
            lfo_range: format!("{}:{}", self.lfo_rate_range[0], self.lfo_rate_range[1]),
            mod_depth_range: format!("{}:{}", self.mod_depth_range[0], self.mod_depth_range[1]),
            voices: self.voices,
            base_freq: self.base_freq,
            noise_level: self.noise_level,
            attack: self.attack,
            release: self.release,
            reverb_mix: self.reverb_mix,
        }
    }
}

/// Configuration loading errors
#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("IO Error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("JSON Parse Error: {0}")]
    JsonError(#[from] serde_json::Error),
}

pub struct GeneratorParams {
    filename: String,
    sample_rate: u32,
    duration: f32,
    lfo_range: String,
    mod_depth_range: String,
    voices: usize,
    base_freq: f32,
    noise_level: f32,
    attack: f32,
    release: f32,
    reverb_mix: f32,
}

impl GeneratorParams {
    fn num_samples(&self) -> u32 {
        (self.duration * self.sample_rate as f32) as u32
    }
}

impl Into<GeneratorParams> for CLI {
    fn into(self) -> GeneratorParams {
        GeneratorParams {
            filename: match &self.output {
                Some(output) => output.to_string(),
                None => generate_filename("ambient", OutputType::WAV),
            },
            sample_rate: self.sample_rate,
            duration: self.duration,
            lfo_range: self.lfo_rate_range,
            mod_depth_range: self.mod_depth_range,
            voices: self.voices,
            base_freq: self.base_freq,
            noise_level: self.noise_level,
            attack: self.attack,
            release: self.release,
            reverb_mix: self.reverb_mix,
        }
    }
}

struct Generator {
    params: GeneratorParams,
    voices: Vec<Voice>,
    num_samples: u32,
    samples: Vec<(f32, f32)>,
    /// Filter state for noise filtering
    /// TODO: change to 2-tuple
    filter_prev_l: f32,
    filter_prev_r: f32,
}

impl Generator {
    fn parse_range(s: &String, default_min: f32, default_max: f32) -> (f32, f32) {
        let parts: Vec<&str> = s.split(':').collect();
        let min: f32 = parts
            .get(0)
            .and_then(|v| v.parse().ok())
            .unwrap_or(default_min);
        let max: f32 = parts
            .get(1)
            .and_then(|v| v.parse().ok())
            .unwrap_or(default_max);
        (min, max)
    }

    fn spec(sample_rate: u32) -> WavSpec {
        WavSpec {
            channels: 2,
            sample_rate: sample_rate,
            bits_per_sample: 16,
            sample_format: hound::SampleFormat::Int,
        }
    }

    fn generate_voices(params: &GeneratorParams) -> Vec<Voice> {
        let mut rng = rand::rng();
        let (lfo_min, lfo_max) = Self::parse_range(&params.lfo_range, 0.05, 0.2);
        let (depth_min, depth_max) = Self::parse_range(&params.mod_depth_range, 0.5, 1.0);
        (0..params.voices)
            .map(|_| {
                let freq = params.base_freq * rng.random_range(0.8..1.2);
                let lfo_rate = rng.random_range(lfo_min..lfo_max);
                let mod_depth = rng.random_range(depth_min..depth_max);
                let pan_rate = rng.random_range(0.01..0.05);

                Voice {
                    freq,
                    lfo_rate,
                    mod_depth,
                    pan_rate,
                }
            })
            .collect()
    }

    fn writer(params: &GeneratorParams) -> Result<WavWriter<BufWriter<File>>, Error> {
        WavWriter::create(&params.filename, Self::spec(params.sample_rate))
            .map_err(|err| err.into())
    }

    fn new(params: GeneratorParams) -> Result<Generator, Error> {
        let num_samples = params.num_samples();
        let voices = Self::generate_voices(&params);
        Ok(Generator {
            params,
            voices,
            num_samples,
            samples: Vec::new(),
            filter_prev_l: 0.0,
            filter_prev_r: 0.0,
        })
    }

    fn envelope(&self, time: f32) -> f32 {
        if time < self.params.attack {
            time / self.params.attack
        } else if time > self.params.duration - self.params.release {
            (self.params.duration - time) / self.params.release
        } else {
            1.0
        }
        .clamp(0.0, 1.0)
    }

    fn write_wav(&self) -> Result<(), Error> {
        let mut writer = Self::writer(&self.params)?;
        for (l, r) in &self.samples {
            let amp = i16::MAX as f32;
            let ls = (l * amp).clamp(i16::MIN as f32, i16::MAX as f32) as i16;
            let rs = (r * amp).clamp(i16::MIN as f32, i16::MAX as f32) as i16;
            writer.write_sample(ls)?;
            writer.write_sample(rs)?;
        }
        writer.finalize()?;
        Ok(())
    }

    fn noise(&self, rng: &mut ThreadRng) -> f32 {
        rng.random_range(-1.0..1.0) * self.params.noise_level
    }

    fn burst_env(t: f32) -> f32 {
        (2.0 * PI * 10.0 * t).sin() * 0.5 + 0.5
    }

    /// granular texture: occasional bursts of noise shaped by a 10Hz LFO
    fn granular_noise(&self, time: f32, rng: &mut impl Rng) -> f32 {
        rng.random_range(-1.0..1.0) * self.params.noise_level * 0.5 * Self::burst_env(time)
    }

    fn filtered_noise(&mut self, _time: f32, rng: &mut impl Rng) -> (f32, f32) {
        const A: f32 = 0.1;
        let w = rng.random_range(-1.0..1.0) * self.params.noise_level * 0.3;

        let fl = A * w + (1.0 - A) * self.filter_prev_l;
        let fr = A * w + (1.0 - A) * self.filter_prev_r;

        self.filter_prev_l = fl;
        self.filter_prev_r = fr;

        (fl, fr)
    }

    fn apply_reverb(&mut self) {
        let sr = self.params.sample_rate as usize;
        let delay = (0.05 * sr as f32) as usize;
        let fb = 0.7;
        let mut buf_l = vec![0.0; delay];
        let mut buf_r = vec![0.0; delay];
        let mut idx = 0;
        for sample in &mut self.samples {
            let dry_l = sample.0;
            let dry_r = sample.1;
            let wet_l = buf_l[idx];
            let wet_r = buf_r[idx];
            let out_l = dry_l * (1.0 - self.params.reverb_mix) + wet_l * self.params.reverb_mix;
            let out_r = dry_r * (1.0 - self.params.reverb_mix) + wet_r * self.params.reverb_mix;
            buf_l[idx] = dry_l + wet_l * fb;
            buf_r[idx] = dry_r + wet_r * fb;
            *sample = (out_l, out_r);
            idx = (idx + 1) % delay;
        }
    }

    fn mutate(left: &mut f32, right: &mut f32, value: &f32) -> (f32, f32) {
        *left += value;
        *right += value;
        (*left, *right)
    }

    fn generate(&mut self) -> Result<(), Error> {
        let mut rng = rand::rng();

        self.samples.reserve(self.num_samples as usize);

        for i in 0..self.num_samples {
            let t = i as f32 / self.params.sample_rate as f32;
            let mut left = 0.0;
            let mut right = 0.0;

            for voice in &self.voices {
                let (l_gain, r_gain) = voice.synthesize(t);
                left += l_gain;
                right += r_gain;
            }

            let noise = self.noise(&mut rng);
            (left, right) = Self::mutate(&mut left, &mut right, &noise);

            let gran_texture = self.granular_noise(t, &mut rng);
            (left, right) = Self::mutate(&mut left, &mut right, &gran_texture);

            let (l_filt, r_filt) = self.filtered_noise(t, &mut rng);
            left += l_filt;
            right += r_filt;

            let env = self.envelope(t);
            (left, right) = Self::mutate(&mut left, &mut right, &env);
            self.samples.push((left, right));
        }

        println!(
            "Generated '{}' ({}s) with {} voices.",
            self.params.filename, self.params.duration, self.params.voices
        );

        Ok(())
    }

    fn run(&mut self) -> Result<(), Error> {
        self.generate()?;
        self.apply_reverb();
        self.write_wav()?;
        println!(
            "Generated '{}' with {} samples.",
            self.params.filename,
            self.samples.len()
        );
        Ok(())
    }
}

struct Voice {
    freq: f32,
    lfo_rate: f32,
    mod_depth: f32,
    pan_rate: f32,
}

impl Voice {
    fn synthesize(&self, t: f32) -> (f32, f32) {
        let mod_env = (2.0 * PI * self.lfo_rate * t).sin() * 0.5 + 0.5;
        let sample = (2.0 * PI * self.freq * t).sin() * (mod_env * self.mod_depth);
        let pan = (2.0 * PI * self.pan_rate * t).sin();
        let l_gain = (1.0 - pan) * 0.5;
        let r_gain = (1.0 + pan) * 0.5;

        (sample * l_gain, sample * r_gain)
    }
}

fn v4_uuid() -> String {
    uuid::Uuid::new_v4().to_string()
}

fn generate_filename(root: &'static str, ext: OutputType) -> String {
    let prefix = &root.replace(" ", "_").to_lowercase();
    let extension = ext.as_str();

    format!("{}_{}.{}", v4_uuid(), prefix, extension)
}

pub fn run() -> Result<(), Error> {
    let params = CLI::parse().to_params()?;
    Generator::new(params).and_then(|mut r| r.run())?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Parser;
    fn cli() -> CLI {
        CLI {
            config: None,
            output: Some("test.wav".to_string()),
            duration: 30.0,
            sample_rate: 48000,
            voices: 6,
            base_freq: 440.0,
            lfo_rate_range: "0.1:0.3".to_string(),
            noise_level: 0.01,
            mod_depth_range: "0.3:0.8".to_string(),
            attack: 3.0,
            release: 5.0,
            reverb_mix: 0.4,
        }
    }

    fn params() -> GeneratorParams {
        GeneratorParams {
            filename: "test.wav".to_string(),
            sample_rate: 44100,
            duration: 20.0,
            lfo_range: "0.05:0.2".to_string(),
            mod_depth_range: "0.5:1.0".to_string(),
            voices: 4,
            base_freq: 330.0,
            noise_level: 0.005,
            attack: 5.0,
            release: 10.0,
            reverb_mix: 0.3,
        }
    }

    fn generator_for_envelope() -> Generator {
        let params = params();

        Generator {
            params,
            voices: vec![],
            num_samples: 0,
            samples: vec![],
            filter_prev_l: 0.0,
            filter_prev_r: 0.0,
        }
    }

    #[test]
    fn test_output_type_as_str() {
        assert_eq!(OutputType::MIDI.as_str(), "midi");
        assert_eq!(OutputType::WAV.as_str(), "wav");
    }

    #[test]
    fn test_generate_filename_helper() {
        let filename = generate_filename("test ambient", OutputType::WAV);
        assert!(filename.contains("test_ambient"));
        assert!(filename.ends_with(".wav"));
        assert!(filename.len() > 20); // UUID + prefix + extension
    }

    #[test]
    fn test_cli_default_values() {
        let cli = CLI::parse_from(&["procsynth"]);
        assert_eq!(cli.duration, 60.0);
        assert_eq!(cli.sample_rate, 44100);
        assert_eq!(cli.voices, 4);
        assert_eq!(cli.base_freq, 330.0);
        assert_eq!(cli.lfo_rate_range, "0.05:0.2");
        assert_eq!(cli.noise_level, 0.005);
        assert_eq!(cli.mod_depth_range, "0.5:1.0");
        assert_eq!(cli.attack, 5.0);
        assert_eq!(cli.release, 10.0);
        assert_eq!(cli.reverb_mix, 0.3);
        assert!(cli.output.is_none());
    }

    #[test]
    fn test_cli_custom_values() {
        let cli = CLI::parse_from(&[
            "procsynth",
            "--output",
            "test.wav",
            "--duration",
            "30.0",
            "--sample-rate",
            "48000",
            "--voices",
            "8",
            "--base-freq",
            "440.0",
            "--lfo-rate-range",
            "0.1:0.3",
            "--noise-level",
            "0.01",
            "--mod-depth-range",
            "0.3:0.8",
            "--attack",
            "3.0",
            "--release",
            "5.0",
            "--reverb-mix",
            "0.5",
        ]);

        assert_eq!(cli.output, Some("test.wav".to_string()));
        assert_eq!(cli.duration, 30.0);
        assert_eq!(cli.sample_rate, 48000);
        assert_eq!(cli.voices, 8);
        assert_eq!(cli.base_freq, 440.0);
        assert_eq!(cli.lfo_rate_range, "0.1:0.3");
        assert_eq!(cli.noise_level, 0.01);
        assert_eq!(cli.mod_depth_range, "0.3:0.8");
        assert_eq!(cli.attack, 3.0);
        assert_eq!(cli.release, 5.0);
        assert_eq!(cli.reverb_mix, 0.5);
    }

    #[test]
    fn test_generator_params_conversion() {
        let cli = cli();
        let params = cli.to_params().unwrap();

        assert_eq!(params.filename, "test.wav");
        assert_eq!(params.duration, 30.0);
        assert_eq!(params.sample_rate, 48000);
        assert_eq!(params.voices, 6);
        assert_eq!(params.base_freq, 440.0);
        assert_eq!(params.lfo_range, "0.1:0.3");
        assert_eq!(params.noise_level, 0.01);
        assert_eq!(params.mod_depth_range, "0.3:0.8");
        assert_eq!(params.attack, 3.0);
        assert_eq!(params.release, 5.0);
        assert_eq!(params.reverb_mix, 0.4);
    }

    #[test]
    fn test_generator_params_num_samples() {
        assert_eq!(params().num_samples(), 882000); // 44100 * 20.0
    }

    #[test]
    fn test_generator_parse_range() {
        assert_eq!(
            Generator::parse_range(&"0.1:0.3".to_string(), 0.0, 1.0),
            (0.1, 0.3)
        );
        assert_eq!(
            Generator::parse_range(&"0.5".to_string(), 0.0, 1.0),
            (0.5, 1.0)
        );
        assert_eq!(
            Generator::parse_range(&"invalid".to_string(), 0.2, 0.8),
            (0.2, 0.8)
        );
        assert_eq!(
            Generator::parse_range(&"".to_string(), 0.2, 0.8),
            (0.2, 0.8)
        );
    }

    #[test]
    fn test_generator_spec() {
        let spec = Generator::spec(44100);
        assert_eq!(spec.channels, 2);
        assert_eq!(spec.sample_rate, 44100);
        assert_eq!(spec.bits_per_sample, 16);
        assert_eq!(spec.sample_format, hound::SampleFormat::Int);
    }

    #[test]
    fn test_voice_synthesis() {
        let voice = Voice {
            freq: 440.0,
            lfo_rate: 0.1,
            mod_depth: 0.5,
            pan_rate: 0.02,
        };

        let (left, right) = voice.synthesize(0.0);
        // At t=0, sin(0) = 0, mod_env = 0.5, pan = 0, so l_gain = r_gain = 0.5
        assert!((left - 0.0).abs() < 1e-6);
        assert!((right - 0.0).abs() < 1e-6);

        // Quarter period - should produce some non-zero output
        let (left, right) = voice.synthesize(1.0 / (4.0 * 440.0));

        assert!(left.abs() > 0.0 || right.abs() > 0.0);
    }

    #[test]
    fn test_generator_envelope_attack_phase() {
        let generator = generator_for_envelope();
        assert_eq!(generator.envelope(0.0), 0.0);
        assert_eq!(generator.envelope(2.5), 0.5);
        assert_eq!(generator.envelope(5.0), 1.0);
    }

    #[test]
    fn test_generator_envelope_sustain_phase() {
        let generator = generator_for_envelope();
        assert_eq!(generator.envelope(10.0), 1.0);
    }

    #[test]
    fn test_generator_envelope_release_phase() {
        let generator = generator_for_envelope();

        // duration=20, release=10, so release starts at t=10
        assert_eq!(generator.envelope(10.0), 1.0);
        assert_eq!(generator.envelope(15.0), 0.5);
        assert_eq!(generator.envelope(20.0), 0.0);
    }

    #[test]
    fn test_generator_envelope_beyond_duration() {
        let generator = generator_for_envelope();
        assert_eq!(generator.envelope(25.0), 0.0);
    }

    #[test]
    fn test_generator_burst_env_oscillation() {
        assert!((Generator::burst_env(0.0) - 0.5).abs() < 1e-6);
        let mut values = vec![];
        for i in 0..100 {
            let t = i as f32 * 0.01;
            values.push(Generator::burst_env(t));
        }

        let min_val = values.iter().fold(f32::INFINITY, |a, &b| a.min(b));
        let max_val = values.iter().fold(f32::NEG_INFINITY, |a, &b| a.max(b));

        assert!(min_val >= 0.0);
        assert!(max_val <= 1.0);
    }

    #[test]
    fn test_generator_mutate() {
        let mut left = 0.1;
        let mut right = 0.2;
        let value = 0.05;

        let (new_left, new_right) = Generator::mutate(&mut left, &mut right, &value);

        assert_eq!(new_left, 0.15);
        assert_eq!(new_right, 0.25);
        assert_eq!(left, 0.15);
        assert_eq!(right, 0.25);
    }

    #[test]
    fn test_generator_creation() {
        let generator = Generator::new(params()).unwrap();

        assert_eq!(generator.voices.len(), 4); // params() has voices=4
        assert_eq!(generator.num_samples, 882000); // 44100 * 20.0
        assert!(generator.samples.is_empty());
    }

    #[test]
    fn test_error_wraps_hound_error() {
        let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
        let hound_err = hound::Error::IoError(io_err);
        assert_eq!(
            format!("{}", Error::HoundError(hound_err)),
            "Hound WAV Error"
        );
    }

    #[test]
    fn test_json_config_conversion() {
        let config = JsonConfig {
            output: Some("test_json.wav".to_string()),
            duration: 45.0,
            sample_rate: 48000,
            voices: 8,
            base_freq: 440.0,
            lfo_rate_range: [0.1, 0.3],
            noise_level: 0.02,
            mod_depth_range: [0.3, 0.8],
            attack: 3.0,
            release: 5.0,
            reverb_mix: 0.4,
        };

        let params = config.to_params();

        assert_eq!(params.filename, "test_json.wav");
        assert_eq!(params.duration, 45.0);
        assert_eq!(params.sample_rate, 48000);
        assert_eq!(params.voices, 8);
        assert_eq!(params.base_freq, 440.0);
        assert_eq!(params.lfo_range, "0.1:0.3");
        assert_eq!(params.noise_level, 0.02);
        assert_eq!(params.mod_depth_range, "0.3:0.8");
        assert_eq!(params.attack, 3.0);
        assert_eq!(params.release, 5.0);
        assert_eq!(params.reverb_mix, 0.4);
    }

    #[test]
    fn test_default_json_conf() {
        let default_config = JsonConfig::default();
        assert_eq!(default_config.duration, 60.0);
        assert_eq!(default_config.sample_rate, 44100);
        assert_eq!(default_config.voices, 4);
        assert_eq!(default_config.base_freq, 330.0);
        assert_eq!(default_config.lfo_rate_range, [0.05, 0.2]);
        assert_eq!(default_config.noise_level, 0.005);
        assert_eq!(default_config.mod_depth_range, [0.5, 1.0]);
        assert_eq!(default_config.attack, 5.0);
        assert_eq!(default_config.release, 10.0);
        assert_eq!(default_config.reverb_mix, 0.3);
        assert!(default_config.output.is_none());
    }
}
