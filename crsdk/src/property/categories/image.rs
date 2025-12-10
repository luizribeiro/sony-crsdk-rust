//! Image category: image quality, format, and storage properties.

use super::{Category, PropertyDef, PropertyValueType};
use crsdk_sys::DevicePropertyCode;

use DevicePropertyCode as C;
use PropertyValueType as V;

/// Image quality, format, and storage properties.
pub struct Image;

impl Category for Image {
    const NAME: &'static str = "Image";
    const PROPERTIES: &'static [PropertyDef] = &[
        PropertyDef::new(
            C::FileType,
            "File Format",
            "File format for recording. JPEG is compressed with some quality loss. RAW captures all sensor data for maximum editing flexibility. RAW+JPEG saves both.",
            Some(V::FileType),
        ),
        PropertyDef::new(
            C::StillImageQuality,
            "JPEG Quality",
            "Compression level for JPEG images. Extra Fine has least compression and best quality. Standard is smaller files but more compression artifacts.",
            Some(V::ImageQuality),
        ),
        PropertyDef::new(
            C::ImageSize,
            "Image Resolution",
            "Resolution of captured images. Larger sizes have more detail but create bigger files.",
            Some(V::ImageSize),
        ),
        PropertyDef::new(
            C::CompressionFileFormatStill,
            "Still Format",
            "Compression format for still images.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::RAWFileCompressionType,
            "RAW Compression",
            "RAW file compression. Uncompressed has best quality but largest files. Lossless compressed is smaller with no quality loss. Compressed is smallest but some data loss.",
            Some(V::RAWFileCompressionType),
        ),
        PropertyDef::new(
            C::StillImageStoreDestination,
            "Save To",
            "Where to save still images: PC, memory card, or both.",
            Some(V::StillImageStoreDestination),
        ),
        PropertyDef::new(
            C::ImageIDNumSetting,
            "Image ID Setting",
            "Numeric identifier embedded in image metadata.",
            Some(V::Switch),
        ),
        PropertyDef::new(
            C::ImageIDNum,
            "Image ID",
            "Current image ID number.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::ImageIDString,
            "Image ID Text",
            "String identifier embedded in image metadata.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::HDMIResolutionStillPlay,
            "HDMI Res (Still)",
            "HDMI output resolution during still image playback.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::HLGStillImage,
            "HLG Still",
            "Enables HLG (Hybrid Log-Gamma) for still images. Creates images optimized for HDR displays.",
            Some(V::Switch),
        ),
        PropertyDef::new(
            C::RemoteSaveImageSize,
            "Remote Save Size",
            "Image size for remote/tethered capture saves.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::FTPTransferStillImageQualitySize,
            "FTP Image Quality",
            "Quality and size settings for FTP transfer of still images.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::FTPAutoTransferTargetStillImage,
            "FTP Auto Target",
            "Target for automatic FTP transfer of still images.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::ProtectImageInFTPTransfer,
            "FTP Protect",
            "Protect images during FTP transfer.",
            Some(V::Switch),
        ),
        PropertyDef::new(
            C::FileSettingsTitleNameSettings,
            "Title Name",
            "Title name embedded in file metadata.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::EmbedLUTFile,
            "Embed LUT",
            "Embeds LUT information in recorded files.",
            Some(V::Switch),
        ),
        PropertyDef::new(
            C::SceneFileIndex,
            "Scene File",
            "Index of the current scene file (camera settings preset).",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::CurrentSceneFileEdited,
            "Scene Modified",
            "Whether the current scene file has been modified.",
            Some(V::Switch),
        ),
        PropertyDef::new(
            C::RecorderExtRawStatus,
            "Ext RAW Status",
            "External RAW recorder status.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::ForcedFileNumberResetEnableStatus,
            "File # Reset Avail",
            "Whether forced file number reset is available.",
            Some(V::OnOff),
        ),
        PropertyDef::new(
            C::MaximumSizeOfImageIDString,
            "Max ID String Len",
            "Maximum length of image ID string.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::SceneFileCommandVersion,
            "Scene File Ver",
            "Scene file command protocol version.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::SceneFileUploadOperationEnableStatus,
            "Scene Upload Avail",
            "Whether scene file upload is available.",
            Some(V::OnOff),
        ),
        PropertyDef::new(
            C::SceneFileDownloadOperationEnableStatus,
            "Scene DL Avail",
            "Whether scene file download is available.",
            Some(V::OnOff),
        ),
        PropertyDef::new(
            C::SceneFileIndexesAvailableForDownload,
            "Scene DL Indexes",
            "Available scene file indexes for download.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::CustomGridLineFileCommandVersion,
            "Grid File Ver",
            "Custom grid line file command version.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::FileSettingsCameraId,
            "Camera ID",
            "Camera identifier in file metadata.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::FileSettingsReelNumber,
            "Reel Number",
            "Reel number for file naming.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::FileSettingsCameraPosition,
            "Camera Position",
            "Camera position identifier in metadata.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::OSDImageMode,
            "OSD Mode",
            "On-screen display image mode.",
            Some(V::OnOff),
        ),
        PropertyDef::new(
            C::LiveViewImageQualityByNumericalValue,
            "LV Quality Value",
            "Live view quality as numeric value.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::StillImageTransSize,
            "Transfer Size",
            "Transfer size for still images.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::RAWJPCSaveImage,
            "RAW+JPG Save",
            "RAW+JPEG save configuration.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::LiveViewImageQuality,
            "LV Quality",
            "Quality setting for live view stream.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::ImagerScanMode,
            "Scan Mode",
            "Image sensor scan mode.",
            Some(V::Integer),
        ),
    ];
}

crate::register_category!(Image);
