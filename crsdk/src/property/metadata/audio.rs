//! Audio property metadata (descriptions, display names).

use crsdk_sys::DevicePropertyCode;

pub fn description(code: DevicePropertyCode) -> &'static str {
    match code {
        DevicePropertyCode::AudioRecording => {
            "Enables or disables audio recording with video. Turn off when using external audio recorders."
        }
        DevicePropertyCode::AudioSignals => {
            "Camera beeps for focus confirmation and self-timer. Disable for quiet shooting environments."
        }
        DevicePropertyCode::AudioSignalsStartEnd => {
            "Audio signals at the start and end of recording. Helps confirm recording status."
        }
        DevicePropertyCode::AudioSignalsVolume => {
            "Volume level for camera beeps and audio signals."
        }
        DevicePropertyCode::AudioLevelDisplay => {
            "Shows audio input levels on screen. Essential for monitoring recording quality."
        }
        DevicePropertyCode::WindNoiseReduct => {
            "Reduces low-frequency wind noise in the built-in microphone. May slightly affect audio quality."
        }
        DevicePropertyCode::AudioInput1TypeSelect | DevicePropertyCode::AudioInput2TypeSelect => {
            "Selects the type of audio input for this connector (XLR, line, etc.)."
        }
        DevicePropertyCode::AudioInputCH1InputSelect
        | DevicePropertyCode::AudioInputCH2InputSelect
        | DevicePropertyCode::AudioInputCH3InputSelect
        | DevicePropertyCode::AudioInputCH4InputSelect => {
            "Selects the audio source for this channel (internal mic, external, line in)."
        }
        DevicePropertyCode::AudioInputCH1Level
        | DevicePropertyCode::AudioInputCH2Level
        | DevicePropertyCode::AudioInputCH3Level
        | DevicePropertyCode::AudioInputCH4Level
        | DevicePropertyCode::AudioInputMasterLevel => {
            "Audio recording level for this channel. Adjust to avoid clipping or too-quiet recordings."
        }
        DevicePropertyCode::AudioInputCH1LevelControl
        | DevicePropertyCode::AudioInputCH2LevelControl
        | DevicePropertyCode::AudioInputCH3LevelControl
        | DevicePropertyCode::AudioInputCH4LevelControl => {
            "How audio level is controlled (auto, manual). Auto adjusts dynamically; manual gives precise control."
        }
        DevicePropertyCode::AudioInputCH1WindFilter
        | DevicePropertyCode::AudioInputCH2WindFilter
        | DevicePropertyCode::AudioInputCH3WindFilter
        | DevicePropertyCode::AudioInputCH4WindFilter => {
            "Wind noise filter for this audio channel. Reduces rumble from wind but may affect bass response."
        }
        DevicePropertyCode::AudioStreamBitDepth => {
            "Bit depth for audio streaming. Higher values capture more dynamic range."
        }
        DevicePropertyCode::AudioStreamChannel => "Number of audio channels for streaming (mono, stereo, etc.).",
        DevicePropertyCode::AudioStreamCodecType => "Audio codec used for streaming. Different codecs have different quality and bandwidth tradeoffs.",
        DevicePropertyCode::AudioStreamSamplingFrequency => {
            "Audio sample rate for streaming. Higher rates capture more high-frequency detail."
        }
        _ => "",
    }
}

pub fn display_name(code: DevicePropertyCode) -> &'static str {
    match code {
        DevicePropertyCode::AudioRecording => "Audio Rec",
        DevicePropertyCode::AudioSignals => "Beep",
        DevicePropertyCode::AudioSignalsStartEnd => "Rec Start/End Beep",
        DevicePropertyCode::AudioSignalsVolume => "Beep Volume",
        DevicePropertyCode::AudioLevelDisplay => "Audio Meter",
        DevicePropertyCode::WindNoiseReduct => "Wind Noise Reduct.",
        DevicePropertyCode::AudioInput1TypeSelect => "Input 1 Type",
        DevicePropertyCode::AudioInput2TypeSelect => "Input 2 Type",
        DevicePropertyCode::AudioInputCH1InputSelect => "CH1 Input",
        DevicePropertyCode::AudioInputCH2InputSelect => "CH2 Input",
        DevicePropertyCode::AudioInputCH3InputSelect => "CH3 Input",
        DevicePropertyCode::AudioInputCH4InputSelect => "CH4 Input",
        DevicePropertyCode::AudioInputCH1Level => "CH1 Level",
        DevicePropertyCode::AudioInputCH2Level => "CH2 Level",
        DevicePropertyCode::AudioInputCH3Level => "CH3 Level",
        DevicePropertyCode::AudioInputCH4Level => "CH4 Level",
        DevicePropertyCode::AudioInputMasterLevel => "Master Level",
        DevicePropertyCode::AudioInputCH1LevelControl => "CH1 Level Ctrl",
        DevicePropertyCode::AudioInputCH2LevelControl => "CH2 Level Ctrl",
        DevicePropertyCode::AudioInputCH3LevelControl => "CH3 Level Ctrl",
        DevicePropertyCode::AudioInputCH4LevelControl => "CH4 Level Ctrl",
        DevicePropertyCode::AudioInputCH1WindFilter => "CH1 Wind Filter",
        DevicePropertyCode::AudioInputCH2WindFilter => "CH2 Wind Filter",
        DevicePropertyCode::AudioInputCH3WindFilter => "CH3 Wind Filter",
        DevicePropertyCode::AudioInputCH4WindFilter => "CH4 Wind Filter",
        DevicePropertyCode::AudioStreamBitDepth => "Stream Bit Depth",
        DevicePropertyCode::AudioStreamChannel => "Stream Channels",
        DevicePropertyCode::AudioStreamCodecType => "Stream Codec",
        DevicePropertyCode::AudioStreamSamplingFrequency => "Stream Sample Rate",
        _ => code.name(),
    }
}
