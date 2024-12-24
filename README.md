# Musical Instrument Classifier

A Rust-based neural network project for classifying musical instruments using the NSynth dataset. This project is designed for educational purposes to understand the basics of audio processing and neural networks from scratch.

## Project Overview

This project aims to build a neural network that can classify musical instruments based on their audio characteristics. We're using the NSynth dataset, which contains high-quality audio samples of various musical instruments.

### Features

- Audio file loading and preprocessing
- Spectral analysis using FFT (Fast Fourier Transform)
- Basic audio feature extraction
- (Coming soon) Neural network implementation from scratch
- (Coming soon) Training and evaluation pipeline

## Requirements

- Rust (latest stable version)
- NSynth dataset (test subset)
- Cargo and its dependencies

### Dependencies

```toml
[dependencies]
hound = "3.5.1"        # WAV file handling
ndarray = "0.15.6"     # Numerical computations
serde_json = "1.0"     # JSON parsing
walkdir = "2.4.0"      # Directory traversal
rustfft = "6.1.0"      # FFT computations
num-complex = "0.4"    # Complex number support
```

## Quick Start

1. Install Rust:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

2. Clone the repository:
```bash
git clone [repository-url]
cd audio_classifier
```

3. Download the NSynth test dataset:
```bash
mkdir -p nsynth_data && cd nsynth_data
wget http://download.magenta.tensorflow.org/datasets/nsynth/nsynth-test.jsonwav.tar.gz
tar xzf nsynth-test.jsonwav.tar.gz
```

4. Build and run:
```bash
cargo run
```

## Project Structure

```
audio_classifier/
├── src/
│   └── main.rs        # Main application code
├── Cargo.toml         # Project dependencies
└── README.md         # This file
```

## Next Steps

1. Feature Extraction
   - [x] Basic audio analysis (amplitude, mean, std)
   - [x] FFT-based spectral analysis
   - [ ] Mel-frequency cepstral coefficients (MFCCs)
   - [ ] Temporal feature extraction

2. Neural Network Implementation
   - [ ] Basic matrix operations
   - [ ] Forward propagation
   - [ ] Backpropagation
   - [ ] Activation functions
   - [ ] Loss functions

3. Training Pipeline
   - [ ] Data loading and batching
   - [ ] Training loop
   - [ ] Validation
   - [ ] Model persistence

4. Evaluation and Visualization
   - [ ] Confusion matrix
   - [ ] Performance metrics
   - [ ] Feature visualization
   - [ ] Audio playback

## Contributing

This is an educational project, and contributions are welcome! Feel free to:
- Suggest improvements
- Add new features
- Fix bugs
- Improve documentation

## License

MIT License - See LICENSE file for details