#[path = "../../ai/voice/audio_pipeline.rs"]
mod audio_pipeline;

#[path = "../../ai/voice/voice_session.rs"]
mod voice_session;

#[test]
fn audio_frame_assigns_valid_sample_rate_and_rms() {
    let frame = audio_pipeline::AudioFrame::new(16000, 1, vec![1000, 2000, 3000, 4000]).unwrap();
    assert_eq!(frame.sample_rate, 16000);
    assert_eq!(frame.channels, 1);
    assert!(frame.rms() > 0.0);
}

#[test]
fn voice_session_validation_requires_minimum_sample_rate() {
    let valid = audio_pipeline::AudioFrame::new(16000, 2, vec![100, 200, -100, -50]).unwrap();
    let invalid = audio_pipeline::AudioFrame::new(4000, 1, vec![30, 40, 50]).unwrap();

    assert!(voice_session::validate_frame(&valid));
    assert!(!voice_session::validate_frame(&invalid));
}
