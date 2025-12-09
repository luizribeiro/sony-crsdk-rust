//! Media category: memory card and media properties.

use super::{Category, PropertyDef, PropertyValueType};
use crsdk_sys::DevicePropertyCode;

use DevicePropertyCode as C;
use PropertyValueType as V;

pub struct Media;

impl Category for Media {
    const NAME: &'static str = "Media";
    const PROPERTIES: &'static [PropertyDef] = &[
        PropertyDef::new(
            C::MediaSLOT1FileType,
            "Slot 1 Format",
            "File type setting for memory card slot 1.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::MediaSLOT2FileType,
            "Slot 2 Format",
            "File type setting for memory card slot 2.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::MediaSLOT1ImageQuality,
            "Slot 1 Quality",
            "Image quality setting for slot 1.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::MediaSLOT2ImageQuality,
            "Slot 2 Quality",
            "Image quality setting for slot 2.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::MediaSLOT1ImageSize,
            "Slot 1 Size",
            "Image size setting for slot 1.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::MediaSLOT2ImageSize,
            "Slot 2 Size",
            "Image size setting for slot 2.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::MediaSLOT1RAWFileCompressionType,
            "Slot 1 RAW Comp",
            "RAW compression for slot 1.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::MediaSLOT2RAWFileCompressionType,
            "Slot 2 RAW Comp",
            "RAW compression for slot 2.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::PlaybackMedia,
            "Play Media",
            "Select which card slot for playback.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::MediaSLOT1Status,
            "Slot 1 Status",
            "Status of memory card in slot 1 (present, formatted, etc.).",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::MediaSLOT2Status,
            "Slot 2 Status",
            "Status of memory card in slot 2.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::MediaSLOT1RemainingNumber,
            "Slot 1 Remaining",
            "Remaining shots possible on slot 1.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::MediaSLOT2RemainingNumber,
            "Slot 2 Remaining",
            "Remaining shots possible on slot 2.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::MediaSLOT1RemainingTime,
            "Slot 1 Time",
            "Remaining recording time on slot 1.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::MediaSLOT2RemainingTime,
            "Slot 2 Time",
            "Remaining recording time on slot 2.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::MediaSLOT1RecordingAvailableType,
            "Slot 1 Rec Type",
            "Types of recording available on slot 1.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::MediaSLOT2RecordingAvailableType,
            "Slot 2 Rec Type",
            "Types of recording available on slot 2.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::CancelMediaFormatEnableStatus,
            "Cancel Format",
            "Whether format can be cancelled.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::DeleteContentOperationEnableStatusSLOT1,
            "Delete Slot 1",
            "Whether content deletion is available on slot 1.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::DeleteContentOperationEnableStatusSLOT2,
            "Delete Slot 2",
            "Whether content deletion is available on slot 2.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::RecordingMedia,
            "Rec Media",
            "Media destination for recording.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::AutoSwitchMedia,
            "Auto Switch",
            "Automatically switch to other card when full.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::RecordingSettingFileName,
            "File Name",
            "File naming settings for recordings.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::MediaSLOT1FormatEnableStatus,
            "Slot 1 Fmt Avail",
            "Whether slot 1 can be formatted.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::MediaSLOT2FormatEnableStatus,
            "Slot 2 Fmt Avail",
            "Whether slot 2 can be formatted.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::MediaFormatProgressRate,
            "Format Progress",
            "Progress of media format operation.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::MediaSLOT1QuickFormatEnableStatus,
            "Slot 1 Quick Fmt",
            "Whether quick format is available for slot 1.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::MediaSLOT2QuickFormatEnableStatus,
            "Slot 2 Quick Fmt",
            "Whether quick format is available for slot 2.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::MediaSLOT1Player,
            "Slot 1 Player",
            "Playback player status for slot 1.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::MediaSLOT2Player,
            "Slot 2 Player",
            "Playback player status for slot 2.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::MediaSLOT1WritingState,
            "Slot 1 Writing",
            "Writing state of slot 1.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::MediaSLOT2WritingState,
            "Slot 2 Writing",
            "Writing state of slot 2.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::MediaSLOT3RecordingAvailableType,
            "Slot 3 Rec Type",
            "Types of recording available on slot 3.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::MediaSLOT3Status,
            "Slot 3 Status",
            "Status of memory card in slot 3.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::MediaSLOT3RemainingTime,
            "Slot 3 Time",
            "Remaining recording time on slot 3.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::MediaSLOT1ContentsInfoListEnableStatus,
            "Slot 1 List Avail",
            "Whether contents list is available for slot 1.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::MediaSLOT2ContentsInfoListEnableStatus,
            "Slot 2 List Avail",
            "Whether contents list is available for slot 2.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::MediaSLOT1ContentsInfoListRegenerateUpdateTime,
            "Slot 1 List Regen",
            "Regenerate update time for slot 1 contents list.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::MediaSLOT2ContentsInfoListRegenerateUpdateTime,
            "Slot 2 List Regen",
            "Regenerate update time for slot 2 contents list.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::MediaSLOT1ContentsInfoListUpdateTime,
            "Slot 1 List Update",
            "Update time for slot 1 contents list.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::MediaSLOT2ContentsInfoListUpdateTime,
            "Slot 2 List Update",
            "Update time for slot 2 contents list.",
            Some(V::Integer),
        ),
    ];
}
