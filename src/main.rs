use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use hound::WavReader;
use serde_json::Value;
use walkdir::WalkDir;
use rustfft::{FftPlanner, num_complex::Complex};
use num_complex::Complex32;

fn read_wav_file(path: &Path) -> Result<(Vec<f32>, u32), Box<dyn std::error::Error>> {
    let mut reader = WavReader::open(path)?;
    let spec = reader.spec();
    let samples: Vec<f32> = reader.samples::<i16>()
        .map(|sample| sample.unwrap_or(0) as f32 / i16::MAX as f32)
        .collect();
    
    Ok((samples, spec.sample_rate))
}

fn compute_fft(samples: &[f32], window_size: usize) -> Vec<f32> {
    let mut planner = FftPlanner::new();
    let fft = planner.plan_fft_forward(window_size);
    
    // Prepare input data
    let mut buffer: Vec<Complex<f32>> = samples.iter()
        .take(window_size)
        .map(|&x| Complex::new(x, 0.0))
        .collect();
    
    // Apply Hanning window
    for i in 0..window_size {
        let window = 0.5 * (1.0 - (2.0 * std::f32::consts::PI * i as f32 / window_size as f32).cos());
        buffer[i] = buffer[i] * window;
    }
    
    // Compute FFT
    fft.process(&mut buffer);
    
    // Convert to magnitude spectrum
    buffer.iter()
        .take(window_size / 2)
        .map(|x| (x.norm() / window_size as f32).log10().max(-10.0))
        .collect()
}

fn analyze_audio(samples: &[f32], sample_rate: u32) -> (f32, f32, f32, Vec<f32>) {
    let max_amplitude = samples.iter().fold(0.0f32, |max, &x| max.max(x.abs()));
    let mean = samples.iter().sum::<f32>() / samples.len() as f32;
    let variance = samples.iter()
        .map(|&x| (x - mean).powi(2))
        .sum::<f32>() / samples.len() as f32;
    
    // Compute spectrum for a 1024-sample window at the 1-second mark
    let window_size = 1024;
    let start_idx = sample_rate as usize; // Start at 1 second
    let spectrum = compute_fft(&samples[start_idx..start_idx + window_size], window_size);
    
    (max_amplitude, mean, variance.sqrt(), spectrum)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Path to the NSynth test dataset
    let dataset_path = Path::new("/workspace/nsynth_data/nsynth-test");
    
    // Read metadata
    let metadata_file = File::open(dataset_path.join("examples.json"))?;
    let metadata: Value = serde_json::from_reader(BufReader::new(metadata_file))?;
    
    // Process a few example files
    let audio_dir = dataset_path.join("audio");
    let mut count = 0;
    
    for entry in WalkDir::new(&audio_dir).into_iter().filter_map(|e| e.ok()) {
        if entry.path().extension().and_then(|s| s.to_str()) == Some("wav") {
            let file_stem = entry.path().file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("");
            
            // Get metadata for this file
            if let Some(info) = metadata.get(file_stem) {
                let instrument_family = info["instrument_family_str"].as_str().unwrap_or("unknown");
                let source = info["instrument_source_str"].as_str().unwrap_or("unknown");
                
                // Read and analyze audio
                if let Ok((samples, sample_rate)) = read_wav_file(entry.path()) {
                    let (max_amp, mean, std_dev) = analyze_audio(&samples, sample_rate);
                    
                    println!("\nFile: {}", file_stem);
                    println!("Instrument Family: {}", instrument_family);
                    println!("Source: {}", source);
                    println!("Sample Rate: {} Hz", sample_rate);
                    println!("Duration: {:.2} seconds", samples.len() as f32 / sample_rate as f32);
                    println!("Max Amplitude: {:.3}", max_amp);
                    println!("Mean: {:.3}", mean);
                    println!("Standard Deviation: {:.3}", std_dev);
                    
                    count += 1;
                    if count >= 5 {
                        break;
                    }
                }
            }
        }
    }
    
    Ok(())
}
