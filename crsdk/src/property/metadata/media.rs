//! Media/storage property metadata (descriptions, display names).

use crsdk_sys::DevicePropertyCode;

pub fn description(code: DevicePropertyCode) -> &'static str {
    match code {
        DevicePropertyCode::MediaSLOT1Status => {
            "Status of memory card in slot 1. Shows if a card is inserted, its capacity, and any errors."
        }
        DevicePropertyCode::MediaSLOT2Status => {
            "Status of memory card in slot 2. Shows if a card is inserted, its capacity, and any errors."
        }
        DevicePropertyCode::MediaSLOT3Status => {
            "Status of memory card in slot 3. Shows if a card is inserted, its capacity, and any errors."
        }
        DevicePropertyCode::AutoSwitchMedia => {
            "Automatically switches to the other card slot when the current card fills up. Prevents missed shots."
        }
        DevicePropertyCode::SimulRecSetting => {
            "Records to both card slots simultaneously. Provides instant backup of every shot."
        }
        DevicePropertyCode::MediaSLOT1FileType => {
            "File type (RAW/JPEG/etc.) saved to memory card slot 1."
        }
        DevicePropertyCode::MediaSLOT2FileType => {
            "File type (RAW/JPEG/etc.) saved to memory card slot 2."
        }
        DevicePropertyCode::MediaSLOT1ImageSize | DevicePropertyCode::MediaSLOT2ImageSize => {
            "Image resolution for files saved to this slot."
        }
        DevicePropertyCode::MediaSLOT1ImageQuality | DevicePropertyCode::MediaSLOT2ImageQuality => {
            "JPEG compression quality for files saved to this slot."
        }
        DevicePropertyCode::MediaSLOT1RAWFileCompressionType
        | DevicePropertyCode::MediaSLOT2RAWFileCompressionType => {
            "RAW file compression type. Uncompressed preserves all data. Lossless/lossy compressed saves space."
        }
        DevicePropertyCode::MediaSLOT1RemainingNumber | DevicePropertyCode::MediaSLOT2RemainingNumber => {
            "Estimated number of photos remaining at current settings."
        }
        DevicePropertyCode::MediaSLOT1RemainingTime
        | DevicePropertyCode::MediaSLOT2RemainingTime
        | DevicePropertyCode::MediaSLOT3RemainingTime => {
            "Estimated recording time remaining at current video settings."
        }
        DevicePropertyCode::MediaSLOT1FormatEnableStatus
        | DevicePropertyCode::MediaSLOT2FormatEnableStatus => {
            "Whether the card in this slot can be formatted."
        }
        DevicePropertyCode::MediaSLOT1QuickFormatEnableStatus
        | DevicePropertyCode::MediaSLOT2QuickFormatEnableStatus => {
            "Whether quick format is available for the card in this slot."
        }
        DevicePropertyCode::MediaSLOT1WritingState | DevicePropertyCode::MediaSLOT2WritingState => {
            "Current write status of the card. Shows if data is being written."
        }
        DevicePropertyCode::MediaSLOT1Player | DevicePropertyCode::MediaSLOT2Player => {
            "Playback source selection for this card slot."
        }
        DevicePropertyCode::MediaSLOT1RecordingAvailableType
        | DevicePropertyCode::MediaSLOT2RecordingAvailableType
        | DevicePropertyCode::MediaSLOT3RecordingAvailableType => {
            "Types of recordings supported by the card in this slot (photo, video, etc.)."
        }
        DevicePropertyCode::MediaSLOT1ContentsInfoListEnableStatus
        | DevicePropertyCode::MediaSLOT2ContentsInfoListEnableStatus => {
            "Whether the content list for this slot is available for reading."
        }
        DevicePropertyCode::MediaSLOT1ContentsInfoListUpdateTime
        | DevicePropertyCode::MediaSLOT2ContentsInfoListUpdateTime => {
            "Last update timestamp of the content info list."
        }
        DevicePropertyCode::MediaSLOT1ContentsInfoListRegenerateUpdateTime
        | DevicePropertyCode::MediaSLOT2ContentsInfoListRegenerateUpdateTime => {
            "Timestamp when the content list was regenerated."
        }
        DevicePropertyCode::MediaFormatProgressRate => {
            "Progress percentage of current format operation."
        }
        DevicePropertyCode::PresetPTZFSlotNumber => {
            "Preset slot for Pan/Tilt/Zoom/Focus position recall."
        }
        DevicePropertyCode::PlaySetOfMultiMedia => {
            "Configuration for playing content from multiple media sources."
        }
        DevicePropertyCode::PlaybackMedia => {
            "Memory card slot or media source for playback. Selects which card to browse."
        }
        DevicePropertyCode::RemoteKeySLOTSelectButton => {
            "Remote control button to select memory card slot."
        }
        DevicePropertyCode::CancelMediaFormatEnableStatus => {
            "Whether media format operation can be cancelled."
        }
        DevicePropertyCode::DeleteContentOperationEnableStatusSLOT1 => {
            "Whether content deletion is available for slot 1."
        }
        DevicePropertyCode::DeleteContentOperationEnableStatusSLOT2 => {
            "Whether content deletion is available for slot 2."
        }
        _ => "",
    }
}

pub fn display_name(code: DevicePropertyCode) -> &'static str {
    match code {
        DevicePropertyCode::MediaSLOT1Status => "Slot 1 Status",
        DevicePropertyCode::MediaSLOT2Status => "Slot 2 Status",
        DevicePropertyCode::MediaSLOT3Status => "Slot 3 Status",
        DevicePropertyCode::MediaSLOT1RemainingNumber => "Slot 1 Remain #",
        DevicePropertyCode::MediaSLOT2RemainingNumber => "Slot 2 Remain #",
        DevicePropertyCode::MediaSLOT1RemainingTime => "Slot 1 Time",
        DevicePropertyCode::MediaSLOT2RemainingTime => "Slot 2 Time",
        DevicePropertyCode::MediaSLOT3RemainingTime => "Slot 3 Time",
        DevicePropertyCode::MediaSLOT1FileType => "Slot 1 File Type",
        DevicePropertyCode::MediaSLOT2FileType => "Slot 2 File Type",
        DevicePropertyCode::MediaSLOT1ImageQuality => "Slot 1 Quality",
        DevicePropertyCode::MediaSLOT2ImageQuality => "Slot 2 Quality",
        DevicePropertyCode::MediaSLOT1ImageSize => "Slot 1 Size",
        DevicePropertyCode::MediaSLOT2ImageSize => "Slot 2 Size",
        DevicePropertyCode::MediaSLOT1RAWFileCompressionType => "Slot 1 RAW Comp",
        DevicePropertyCode::MediaSLOT2RAWFileCompressionType => "Slot 2 RAW Comp",
        DevicePropertyCode::MediaSLOT1QuickFormatEnableStatus => "Slot 1 Quick Fmt",
        DevicePropertyCode::MediaSLOT2QuickFormatEnableStatus => "Slot 2 Quick Fmt",
        DevicePropertyCode::MediaSLOT1FormatEnableStatus => "Slot 1 Format",
        DevicePropertyCode::MediaSLOT2FormatEnableStatus => "Slot 2 Format",
        DevicePropertyCode::MediaSLOT1WritingState => "Slot 1 Writing",
        DevicePropertyCode::MediaSLOT2WritingState => "Slot 2 Writing",
        DevicePropertyCode::MediaSLOT1Player => "Slot 1 Player",
        DevicePropertyCode::MediaSLOT2Player => "Slot 2 Player",
        DevicePropertyCode::MediaSLOT1RecordingAvailableType => "Slot 1 Rec Type",
        DevicePropertyCode::MediaSLOT2RecordingAvailableType => "Slot 2 Rec Type",
        DevicePropertyCode::MediaSLOT3RecordingAvailableType => "Slot 3 Rec Type",
        DevicePropertyCode::MediaSLOT1ContentsInfoListEnableStatus => "Slot 1 Content",
        DevicePropertyCode::MediaSLOT2ContentsInfoListEnableStatus => "Slot 2 Content",
        DevicePropertyCode::MediaSLOT1ContentsInfoListUpdateTime => "Slot 1 Updated",
        DevicePropertyCode::MediaSLOT2ContentsInfoListUpdateTime => "Slot 2 Updated",
        DevicePropertyCode::MediaSLOT1ContentsInfoListRegenerateUpdateTime => "Slot 1 Regen",
        DevicePropertyCode::MediaSLOT2ContentsInfoListRegenerateUpdateTime => "Slot 2 Regen",
        DevicePropertyCode::MediaFormatProgressRate => "Format Progress",
        DevicePropertyCode::AutoSwitchMedia => "Auto Switch",
        DevicePropertyCode::SimulRecSetting => "Simul. Rec",
        DevicePropertyCode::RemoteKeySLOTSelectButton => "Slot Select Button",
        DevicePropertyCode::PlaySetOfMultiMedia => "Multi-Media Set",
        DevicePropertyCode::CancelMediaFormatEnableStatus => "Cancel Format",
        DevicePropertyCode::DeleteContentOperationEnableStatusSLOT1 => "Delete Slot 1",
        DevicePropertyCode::DeleteContentOperationEnableStatusSLOT2 => "Delete Slot 2",
        DevicePropertyCode::PlaybackMedia => "Play Media",
        DevicePropertyCode::PresetPTZFSlotNumber => "PTZF Slot #",
        _ => code.name(),
    }
}
