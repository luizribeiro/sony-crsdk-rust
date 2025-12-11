# Property Value Type Audit

This document tracks properties currently using `V::Integer` that have corresponding enum definitions in the Sony SDK header file (`CrDeviceProperty.h`).

## Summary

- **Total properties needing typed enums:** 35
- **Focus category:** 9 properties
- **Movie category:** 20 properties
- **Picture Profile category:** 7 properties

---

## Focus Category (9 properties)

| Property | SDK Enum | Type | Values |
|----------|----------|------|--------|
| FocusBracketOrder | `CrFocusBracketOrder` | CrInt8u | `0ToMinusToPlus`, `0ToPlus` |
| FocusBracketIntervalUntilNextShot | `CrFocusBracketIntervalUntilNextShot` | CrInt16u | `Invalid` (0x0000), numeric values (10x seconds) |
| FocusMagnificationTime | `CrFocusMagnificationTime` | CrInt8u | `NoLimit` (0xFF), numeric values |
| FocusBracketRecordingFolder | `CrFocusBracketRecordingFolder` | CrInt8u | Folder selection values |
| PushAutoFocus | `CrPushAutoFocus` | CrInt16u | `Up` (0x0001), `Down` (0x0002) |
| FocusOperationWithInt16 | `CrFocusOperationWithInt16EnableStatus` | CrInt8u | `Disable`, `Enable` |
| FocusTouchSpotStatus | `CrFocusTouchSpotStatus` | CrInt8u | `Stopped`, `Running` |
| FocusBracketShootingStatus | `CrFocusBracketShootingStatus` | CrInt8u | `NotShooting`, `Shooting` |
| FocusDrivingStatus | `CrFocusDrivingStatus` | CrInt8u | `NotDriving`, `Driving` |

---

## Movie Category (20 properties)

| Property | SDK Enum | Type | Values |
|----------|----------|------|--------|
| MovieRecordingFrameRateSetting | `CrRecordingFrameRateSettingMovie` | CrInt8u | Frame rate values |
| MovieShootingModeColorGamut | `CrMovieShootingModeColorGamut` | CrInt8u | `S_Gamut3_Cine`, `S_Gamut3` |
| MovieShootingModeTargetDisplay | `CrMovieShootingModeTargetDisplay` | CrInt8u | `BT709`, `BT2020` |
| SQFrameRate | `CrSQFrameRate` | CrInt16u | `Invalid` (0x0000), frame rate values |
| TimeCodePreset | `CrTimeCodePreset` | CrInt16u | Numeric preset values |
| UserBitTimeRec | `CrUserBitTimeRec` | CrInt8u | Recording mode values |
| RecorderSaveDestination | `CrRecorderSaveDestination` | CrInt16u | `External` (0x0001), `Internal` (0x0010), `ExternalAndInternal` (0x0011) |
| RecordingFolderFormat | `CrRecordingFolderFormat` | CrInt8u | `Standard`, `Date` |
| RecordingFileNumber | `CrRecordingFileNumber` | CrInt8u | Numbering format values |
| LogShootingMode | `CrLogShootingMode` | CrInt16u | Multiple log mode variants |
| VideoRecordingFormatQuality | `CrVideoRecordingFormatQuality` | CrInt16u | `Level1` (0x0001), `Level2` (0x0002), `Level3` (0x0003) |
| VideoRecordingFormatBitrateSetting | `CrVideoRecordingFormatBitrateSetting` | CrInt16u | `None` (0x0000), bitrate level values |
| ValidRecordingVideoFormat | Multiple enums | Various | Resolution, Bitrate, ScanLineType |
| VideoStreamMovieRecPermission | `CrVideoStreamMovieRecPermission` | CrInt8u | Permission flags |
| TimeCodePresetResetEnableStatus | `CrTimeCodePresetResetEnableStatus` | CrInt8u | `Disable`, `Enable` |
| MovieQualityFullAutoMode | `CrMovieQualityFullAutoMode` | CrInt8u | Quality level values |
| PictureCacheRecSizeAndTime | `CrPictureCacheRecSizeAndTime_CacheSizeMask` | CrInt32u | Bitmask for cache size |
| MovieRecReviewPlayingState | `CrMovieRecReviewPlayingState` | CrInt8u | Playback state values |
| MoviePlayingSpeed | `CrMoviePlayingSpeed` | CrInt64u | Multiple playback speeds |
| MoviePlayingState | `CrMoviePlayingState` | CrInt8u | Playback state values |

---

## Picture Profile Category (7 properties)

| Property | SDK Enum | Type | Values |
|----------|----------|------|--------|
| PictureProfileBlackLevel | (numeric adjustment) | - | Numeric values |
| PictureProfileDetailAdjustMode | `CrPictureProfileDetailAdjustMode` | CrInt8u | `Auto`, `Manual` |
| PictureProfileKneeMode | `CrPictureProfileKneeMode` | CrInt8 | `Auto`, `Manual` |
| PictureProfileKneeAutoSetSensitivity | `CrPictureProfileKneeAutoSetSensitivity` | CrInt8u | `Low`, `Mid`, `High` |
| PictureProfileResetEnableStatus | `CrPictureProfileResetEnableStatus` | CrInt8u | `Disable`, `Enable` |
| CreativeLookResetEnableStatus | `CrCreativeLookResetEnableStatus` | CrInt8u | Reset capability values |
| PlaybackContentsGammaType | `CrPlaybackContentsGammaType` | CrInt16u | `S_Gamut3_S_Log3` (0x0001), `S_Gamut3_Cine_S_Log3` (0x0002), `BT2020_HLG` (0x0003), `BT2020_S_Log3` (0x0004), `Others` (0xFFFF) |

---

## Implementation Priority

### High Priority (commonly used, clear enum definitions)

**Batch 5: Focus Status Types (4 types)**
- [ ] FocusDrivingStatus: NotDriving/Driving
- [ ] FocusBracketShootingStatus: NotShooting/Shooting
- [ ] FocusTouchSpotStatus: Stopped/Running
- [ ] FocusBracketOrder: 0ToMinusToPlus/0ToPlus

**Batch 6: Movie Color/Format Types (5 types)**
- [ ] PlaybackContentsGammaType: S-Log3, HLG, BT2020, etc.
- [ ] MovieShootingModeColorGamut: S_Gamut3_Cine/S_Gamut3
- [ ] MovieShootingModeTargetDisplay: BT709/BT2020
- [ ] RecorderSaveDestination: External/Internal/Both
- [ ] VideoRecordingFormatQuality: Level1/Level2/Level3

**Batch 7: Picture Profile Control Types (3 types)**
- [ ] PictureProfileDetailAdjustMode: Auto/Manual
- [ ] PictureProfileKneeMode: Auto/Manual
- [ ] PictureProfileKneeAutoSetSensitivity: Low/Mid/High

### Medium Priority (useful but less common)

**Batch 8: Movie Recording Types (4 types)**
- [ ] RecordingFolderFormat: Standard/Date
- [ ] MoviePlayingState: Playback states
- [ ] MovieRecReviewPlayingState: Review states
- [ ] TimeCodePresetResetEnableStatus: Disable/Enable

**Batch 9: Focus Control Types (4 types)**
- [ ] PushAutoFocus: Up/Down
- [ ] FocusOperationWithInt16EnableStatus: Disable/Enable
- [ ] FocusMagnificationTime: NoLimit + numeric
- [ ] FocusBracketRecordingFolder: Folder values

### Lower Priority (specialized use cases)

- LogShootingMode: Multiple variants
- MoviePlayingSpeed: Multiple speeds
- ValidRecordingVideoFormat: Complex multi-enum
- PictureCacheRecSizeAndTime: Bitmask handling
- Various enable/status flags

---

## Notes

- Most enums use CrInt8u (8-bit unsigned)
- Values typically start at 0x01 or 0x0001 (not 0)
- Enable/Disable patterns are common and could share a base type
- Some properties have complex packed values (bitmasks, multiple fields)
