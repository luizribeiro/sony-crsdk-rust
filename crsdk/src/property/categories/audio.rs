//! Audio category: recording and monitoring properties.

use super::{Category, PropertyDef, PropertyValueType};
use crsdk_sys::DevicePropertyCode;

use DevicePropertyCode as C;
use PropertyValueType as V;

pub struct Audio;

impl Category for Audio {
    const NAME: &'static str = "Audio";
    const PROPERTIES: &'static [PropertyDef] = &[
        PropertyDef::new(
            C::AudioInputMasterLevel,
            "Master Audio Level",
            "Master level for all audio inputs. Controls overall recording volume.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::AudioLevelDisplay,
            "Audio Meter Display",
            "Shows audio levels on screen during recording.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::AudioSignals,
            "Audio Beep",
            "Enable or disable audio beeps for camera operations.",
            Some(V::OnOff),
        ),
        PropertyDef::new(
            C::AudioSignalsStartEnd,
            "Start/End Signals",
            "Audio signals at recording start and end.",
            Some(V::OnOff),
        ),
        PropertyDef::new(
            C::AudioSignalsVolume,
            "Signal Volume",
            "Volume level for audio signals and beeps.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::AudioInputCH1LevelControl,
            "CH1 Level Control",
            "Level control mode for audio channel 1.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::AudioInputCH2LevelControl,
            "CH2 Level Control",
            "Level control mode for audio channel 2.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::AudioInputCH3LevelControl,
            "CH3 Level Control",
            "Level control mode for audio channel 3.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::AudioInputCH4LevelControl,
            "CH4 Level Control",
            "Level control mode for audio channel 4.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::AudioInputCH1Level,
            "CH1 Level",
            "Recording level for audio channel 1.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::AudioInputCH2Level,
            "CH2 Level",
            "Recording level for audio channel 2.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::AudioInputCH3Level,
            "CH3 Level",
            "Recording level for audio channel 3.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::AudioInputCH4Level,
            "CH4 Level",
            "Recording level for audio channel 4.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::AudioInputCH1InputSelect,
            "CH1 Input",
            "Input source selection for audio channel 1.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::AudioInputCH2InputSelect,
            "CH2 Input",
            "Input source selection for audio channel 2.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::AudioInputCH3InputSelect,
            "CH3 Input",
            "Input source selection for audio channel 3.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::AudioInputCH4InputSelect,
            "CH4 Input",
            "Input source selection for audio channel 4.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::AudioInputCH1WindFilter,
            "CH1 Wind Filter",
            "Wind noise filter for audio channel 1.",
            Some(V::OnOff),
        ),
        PropertyDef::new(
            C::AudioInputCH2WindFilter,
            "CH2 Wind Filter",
            "Wind noise filter for audio channel 2.",
            Some(V::OnOff),
        ),
        PropertyDef::new(
            C::AudioInputCH3WindFilter,
            "CH3 Wind Filter",
            "Wind noise filter for audio channel 3.",
            Some(V::OnOff),
        ),
        PropertyDef::new(
            C::AudioInputCH4WindFilter,
            "CH4 Wind Filter",
            "Wind noise filter for audio channel 4.",
            Some(V::OnOff),
        ),
        PropertyDef::new(
            C::AudioInput1TypeSelect,
            "Input 1 Type",
            "Type of audio input 1 (internal mic, external, etc.).",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::AudioInput2TypeSelect,
            "Input 2 Type",
            "Type of audio input 2 (internal mic, external, etc.).",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::AudioStreamCodecType,
            "Stream Codec",
            "Audio codec for streaming output.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::AudioStreamSamplingFrequency,
            "Stream Sample Rate",
            "Sampling frequency for audio stream.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::AudioStreamBitDepth,
            "Stream Bit Depth",
            "Bit depth for audio stream.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::AudioStreamChannel,
            "Stream Channels",
            "Number of audio channels for streaming.",
            Some(V::Integer),
        ),
    ];
}
