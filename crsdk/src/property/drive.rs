//! Drive mode property types and metadata.

use super::PropertyValueType;
use crsdk_sys::DevicePropertyCode;

/// Drive mode / shooting mode settings
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum DriveMode {
    Single = crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Single,
    ContinuousHi = crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Hi,
    ContinuousHiPlus = crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Hi_Plus,
    ContinuousHiLive = crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Hi_Live,
    ContinuousLo = crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Lo,
    Continuous = crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous,
    ContinuousSpeedPriority = crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_SpeedPriority,
    ContinuousMid = crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Mid,
    ContinuousMidLive = crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Mid_Live,
    ContinuousLoLive = crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Lo_Live,
    SingleBurstShootingLo = crsdk_sys::SCRSDK::CrDriveMode_CrDrive_SingleBurstShooting_lo,
    SingleBurstShootingMid = crsdk_sys::SCRSDK::CrDriveMode_CrDrive_SingleBurstShooting_mid,
    SingleBurstShootingHi = crsdk_sys::SCRSDK::CrDriveMode_CrDrive_SingleBurstShooting_hi,
    FocusBracket = crsdk_sys::SCRSDK::CrDriveMode_CrDrive_FocusBracket,
    Timelapse = crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Timelapse,
    Timer2s = crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Timer_2s,
    Timer5s = crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Timer_5s,
    Timer10s = crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Timer_10s,
    ContinuousBracket03Ev3Pics =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Bracket_03Ev_3pics,
    ContinuousBracket03Ev5Pics =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Bracket_03Ev_5pics,
    ContinuousBracket03Ev9Pics =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Bracket_03Ev_9pics,
    ContinuousBracket05Ev3Pics =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Bracket_05Ev_3pics,
    ContinuousBracket05Ev5Pics =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Bracket_05Ev_5pics,
    ContinuousBracket05Ev9Pics =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Bracket_05Ev_9pics,
    ContinuousBracket07Ev3Pics =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Bracket_07Ev_3pics,
    ContinuousBracket07Ev5Pics =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Bracket_07Ev_5pics,
    ContinuousBracket07Ev9Pics =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Bracket_07Ev_9pics,
    ContinuousBracket10Ev3Pics =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Bracket_10Ev_3pics,
    ContinuousBracket10Ev5Pics =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Bracket_10Ev_5pics,
    ContinuousBracket10Ev9Pics =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Bracket_10Ev_9pics,
    ContinuousBracket20Ev3Pics =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Bracket_20Ev_3pics,
    ContinuousBracket20Ev5Pics =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Bracket_20Ev_5pics,
    ContinuousBracket30Ev3Pics =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Bracket_30Ev_3pics,
    ContinuousBracket30Ev5Pics =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Bracket_30Ev_5pics,
    ContinuousBracket03Ev2PicsPlus =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Bracket_03Ev_2pics_Plus,
    ContinuousBracket03Ev2PicsMinus =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Bracket_03Ev_2pics_Minus,
    ContinuousBracket03Ev7Pics =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Bracket_03Ev_7pics,
    ContinuousBracket05Ev2PicsPlus =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Bracket_05Ev_2pics_Plus,
    ContinuousBracket05Ev2PicsMinus =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Bracket_05Ev_2pics_Minus,
    ContinuousBracket05Ev7Pics =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Bracket_05Ev_7pics,
    ContinuousBracket07Ev2PicsPlus =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Bracket_07Ev_2pics_Plus,
    ContinuousBracket07Ev2PicsMinus =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Bracket_07Ev_2pics_Minus,
    ContinuousBracket07Ev7Pics =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Bracket_07Ev_7pics,
    ContinuousBracket10Ev2PicsPlus =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Bracket_10Ev_2pics_Plus,
    ContinuousBracket10Ev2PicsMinus =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Bracket_10Ev_2pics_Minus,
    ContinuousBracket10Ev7Pics =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Bracket_10Ev_7pics,
    ContinuousBracket13Ev2PicsPlus =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Bracket_13Ev_2pics_Plus,
    ContinuousBracket13Ev2PicsMinus =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Bracket_13Ev_2pics_Minus,
    ContinuousBracket13Ev3Pics =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Bracket_13Ev_3pics,
    ContinuousBracket13Ev5Pics =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Bracket_13Ev_5pics,
    ContinuousBracket13Ev7Pics =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Bracket_13Ev_7pics,
    ContinuousBracket15Ev2PicsPlus =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Bracket_15Ev_2pics_Plus,
    ContinuousBracket15Ev2PicsMinus =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Bracket_15Ev_2pics_Minus,
    ContinuousBracket15Ev3Pics =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Bracket_15Ev_3pics,
    ContinuousBracket15Ev5Pics =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Bracket_15Ev_5pics,
    ContinuousBracket15Ev7Pics =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Bracket_15Ev_7pics,
    ContinuousBracket17Ev2PicsPlus =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Bracket_17Ev_2pics_Plus,
    ContinuousBracket17Ev2PicsMinus =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Bracket_17Ev_2pics_Minus,
    ContinuousBracket17Ev3Pics =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Bracket_17Ev_3pics,
    ContinuousBracket17Ev5Pics =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Bracket_17Ev_5pics,
    ContinuousBracket17Ev7Pics =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Bracket_17Ev_7pics,
    ContinuousBracket20Ev2PicsPlus =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Bracket_20Ev_2pics_Plus,
    ContinuousBracket20Ev2PicsMinus =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Bracket_20Ev_2pics_Minus,
    ContinuousBracket20Ev7Pics =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Bracket_20Ev_7pics,
    ContinuousBracket23Ev2PicsPlus =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Bracket_23Ev_2pics_Plus,
    ContinuousBracket23Ev2PicsMinus =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Bracket_23Ev_2pics_Minus,
    ContinuousBracket23Ev3Pics =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Bracket_23Ev_3pics,
    ContinuousBracket23Ev5Pics =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Bracket_23Ev_5pics,
    ContinuousBracket25Ev2PicsPlus =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Bracket_25Ev_2pics_Plus,
    ContinuousBracket25Ev2PicsMinus =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Bracket_25Ev_2pics_Minus,
    ContinuousBracket25Ev3Pics =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Bracket_25Ev_3pics,
    ContinuousBracket25Ev5Pics =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Bracket_25Ev_5pics,
    ContinuousBracket27Ev2PicsPlus =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Bracket_27Ev_2pics_Plus,
    ContinuousBracket27Ev2PicsMinus =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Bracket_27Ev_2pics_Minus,
    ContinuousBracket27Ev3Pics =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Bracket_27Ev_3pics,
    ContinuousBracket27Ev5Pics =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Bracket_27Ev_5pics,
    ContinuousBracket30Ev2PicsPlus =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Bracket_30Ev_2pics_Plus,
    ContinuousBracket30Ev2PicsMinus =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Bracket_30Ev_2pics_Minus,
    SingleBracket03Ev3Pics = crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Single_Bracket_03Ev_3pics,
    SingleBracket03Ev5Pics = crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Single_Bracket_03Ev_5pics,
    SingleBracket03Ev9Pics = crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Single_Bracket_03Ev_9pics,
    SingleBracket05Ev3Pics = crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Single_Bracket_05Ev_3pics,
    SingleBracket05Ev5Pics = crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Single_Bracket_05Ev_5pics,
    SingleBracket05Ev9Pics = crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Single_Bracket_05Ev_9pics,
    SingleBracket07Ev3Pics = crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Single_Bracket_07Ev_3pics,
    SingleBracket07Ev5Pics = crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Single_Bracket_07Ev_5pics,
    SingleBracket07Ev9Pics = crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Single_Bracket_07Ev_9pics,
    SingleBracket10Ev3Pics = crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Single_Bracket_10Ev_3pics,
    SingleBracket10Ev5Pics = crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Single_Bracket_10Ev_5pics,
    SingleBracket10Ev9Pics = crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Single_Bracket_10Ev_9pics,
    SingleBracket20Ev3Pics = crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Single_Bracket_20Ev_3pics,
    SingleBracket20Ev5Pics = crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Single_Bracket_20Ev_5pics,
    SingleBracket30Ev3Pics = crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Single_Bracket_30Ev_3pics,
    SingleBracket30Ev5Pics = crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Single_Bracket_30Ev_5pics,
    SingleBracket03Ev2PicsPlus =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Single_Bracket_03Ev_2pics_Plus,
    SingleBracket03Ev2PicsMinus =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Single_Bracket_03Ev_2pics_Minus,
    SingleBracket03Ev7Pics = crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Single_Bracket_03Ev_7pics,
    SingleBracket05Ev2PicsPlus =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Single_Bracket_05Ev_2pics_Plus,
    SingleBracket05Ev2PicsMinus =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Single_Bracket_05Ev_2pics_Minus,
    SingleBracket05Ev7Pics = crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Single_Bracket_05Ev_7pics,
    SingleBracket07Ev2PicsPlus =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Single_Bracket_07Ev_2pics_Plus,
    SingleBracket07Ev2PicsMinus =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Single_Bracket_07Ev_2pics_Minus,
    SingleBracket07Ev7Pics = crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Single_Bracket_07Ev_7pics,
    SingleBracket10Ev2PicsPlus =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Single_Bracket_10Ev_2pics_Plus,
    SingleBracket10Ev2PicsMinus =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Single_Bracket_10Ev_2pics_Minus,
    SingleBracket10Ev7Pics = crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Single_Bracket_10Ev_7pics,
    SingleBracket13Ev2PicsPlus =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Single_Bracket_13Ev_2pics_Plus,
    SingleBracket13Ev2PicsMinus =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Single_Bracket_13Ev_2pics_Minus,
    SingleBracket13Ev3Pics = crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Single_Bracket_13Ev_3pics,
    SingleBracket13Ev5Pics = crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Single_Bracket_13Ev_5pics,
    SingleBracket13Ev7Pics = crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Single_Bracket_13Ev_7pics,
    SingleBracket15Ev2PicsPlus =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Single_Bracket_15Ev_2pics_Plus,
    SingleBracket15Ev2PicsMinus =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Single_Bracket_15Ev_2pics_Minus,
    SingleBracket15Ev3Pics = crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Single_Bracket_15Ev_3pics,
    SingleBracket15Ev5Pics = crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Single_Bracket_15Ev_5pics,
    SingleBracket15Ev7Pics = crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Single_Bracket_15Ev_7pics,
    SingleBracket17Ev2PicsPlus =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Single_Bracket_17Ev_2pics_Plus,
    SingleBracket17Ev2PicsMinus =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Single_Bracket_17Ev_2pics_Minus,
    SingleBracket17Ev3Pics = crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Single_Bracket_17Ev_3pics,
    SingleBracket17Ev5Pics = crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Single_Bracket_17Ev_5pics,
    SingleBracket17Ev7Pics = crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Single_Bracket_17Ev_7pics,
    SingleBracket20Ev2PicsPlus =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Single_Bracket_20Ev_2pics_Plus,
    SingleBracket20Ev2PicsMinus =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Single_Bracket_20Ev_2pics_Minus,
    SingleBracket20Ev7Pics = crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Single_Bracket_20Ev_7pics,
    SingleBracket23Ev2PicsPlus =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Single_Bracket_23Ev_2pics_Plus,
    SingleBracket23Ev2PicsMinus =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Single_Bracket_23Ev_2pics_Minus,
    SingleBracket23Ev3Pics = crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Single_Bracket_23Ev_3pics,
    SingleBracket23Ev5Pics = crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Single_Bracket_23Ev_5pics,
    SingleBracket25Ev2PicsPlus =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Single_Bracket_25Ev_2pics_Plus,
    SingleBracket25Ev2PicsMinus =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Single_Bracket_25Ev_2pics_Minus,
    SingleBracket25Ev3Pics = crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Single_Bracket_25Ev_3pics,
    SingleBracket25Ev5Pics = crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Single_Bracket_25Ev_5pics,
    SingleBracket27Ev2PicsPlus =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Single_Bracket_27Ev_2pics_Plus,
    SingleBracket27Ev2PicsMinus =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Single_Bracket_27Ev_2pics_Minus,
    SingleBracket27Ev3Pics = crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Single_Bracket_27Ev_3pics,
    SingleBracket27Ev5Pics = crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Single_Bracket_27Ev_5pics,
    SingleBracket30Ev2PicsPlus =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Single_Bracket_30Ev_2pics_Plus,
    SingleBracket30Ev2PicsMinus =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Single_Bracket_30Ev_2pics_Minus,
    WbBracketLo = crsdk_sys::SCRSDK::CrDriveMode_CrDrive_WB_Bracket_Lo,
    WbBracketHi = crsdk_sys::SCRSDK::CrDriveMode_CrDrive_WB_Bracket_Hi,
    DroBracketLo = crsdk_sys::SCRSDK::CrDriveMode_CrDrive_DRO_Bracket_Lo,
    DroBracketHi = crsdk_sys::SCRSDK::CrDriveMode_CrDrive_DRO_Bracket_Hi,
    ContinuousTimer3Pics = crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Timer_3pics,
    ContinuousTimer5Pics = crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Timer_5pics,
    ContinuousTimer2s3Pics = crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Timer_2s_3pics,
    ContinuousTimer2s5Pics = crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Timer_2s_5pics,
    ContinuousTimer5s3Pics = crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Timer_5s_3pics,
    ContinuousTimer5s5Pics = crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Timer_5s_5pics,
    LpfBracket = crsdk_sys::SCRSDK::CrDriveMode_CrDrive_LPF_Bracket,
    RemoteCommander = crsdk_sys::SCRSDK::CrDriveMode_CrDrive_RemoteCommander,
    MirrorUp = crsdk_sys::SCRSDK::CrDriveMode_CrDrive_MirrorUp,
    SelfPortrait1 = crsdk_sys::SCRSDK::CrDriveMode_CrDrive_SelfPortrait_1,
    SelfPortrait2 = crsdk_sys::SCRSDK::CrDriveMode_CrDrive_SelfPortrait_2,
}

impl DriveMode {
    pub fn as_raw(self) -> u64 {
        self as u64
    }

    pub fn from_raw(value: u64) -> Option<Self> {
        use crsdk_sys::SCRSDK::*;
        Some(match value as u32 {
            CrDriveMode_CrDrive_Single => Self::Single,
            CrDriveMode_CrDrive_Continuous_Hi => Self::ContinuousHi,
            CrDriveMode_CrDrive_Continuous_Hi_Plus => Self::ContinuousHiPlus,
            CrDriveMode_CrDrive_Continuous_Hi_Live => Self::ContinuousHiLive,
            CrDriveMode_CrDrive_Continuous_Lo => Self::ContinuousLo,
            CrDriveMode_CrDrive_Continuous => Self::Continuous,
            CrDriveMode_CrDrive_Continuous_SpeedPriority => Self::ContinuousSpeedPriority,
            CrDriveMode_CrDrive_Continuous_Mid => Self::ContinuousMid,
            CrDriveMode_CrDrive_Continuous_Mid_Live => Self::ContinuousMidLive,
            CrDriveMode_CrDrive_Continuous_Lo_Live => Self::ContinuousLoLive,
            CrDriveMode_CrDrive_SingleBurstShooting_lo => Self::SingleBurstShootingLo,
            CrDriveMode_CrDrive_SingleBurstShooting_mid => Self::SingleBurstShootingMid,
            CrDriveMode_CrDrive_SingleBurstShooting_hi => Self::SingleBurstShootingHi,
            CrDriveMode_CrDrive_FocusBracket => Self::FocusBracket,
            CrDriveMode_CrDrive_Timelapse => Self::Timelapse,
            CrDriveMode_CrDrive_Timer_2s => Self::Timer2s,
            CrDriveMode_CrDrive_Timer_5s => Self::Timer5s,
            CrDriveMode_CrDrive_Timer_10s => Self::Timer10s,
            CrDriveMode_CrDrive_Continuous_Bracket_03Ev_3pics => Self::ContinuousBracket03Ev3Pics,
            CrDriveMode_CrDrive_Continuous_Bracket_03Ev_5pics => Self::ContinuousBracket03Ev5Pics,
            CrDriveMode_CrDrive_Continuous_Bracket_03Ev_9pics => Self::ContinuousBracket03Ev9Pics,
            CrDriveMode_CrDrive_Continuous_Bracket_05Ev_3pics => Self::ContinuousBracket05Ev3Pics,
            CrDriveMode_CrDrive_Continuous_Bracket_05Ev_5pics => Self::ContinuousBracket05Ev5Pics,
            CrDriveMode_CrDrive_Continuous_Bracket_05Ev_9pics => Self::ContinuousBracket05Ev9Pics,
            CrDriveMode_CrDrive_Continuous_Bracket_07Ev_3pics => Self::ContinuousBracket07Ev3Pics,
            CrDriveMode_CrDrive_Continuous_Bracket_07Ev_5pics => Self::ContinuousBracket07Ev5Pics,
            CrDriveMode_CrDrive_Continuous_Bracket_07Ev_9pics => Self::ContinuousBracket07Ev9Pics,
            CrDriveMode_CrDrive_Continuous_Bracket_10Ev_3pics => Self::ContinuousBracket10Ev3Pics,
            CrDriveMode_CrDrive_Continuous_Bracket_10Ev_5pics => Self::ContinuousBracket10Ev5Pics,
            CrDriveMode_CrDrive_Continuous_Bracket_10Ev_9pics => Self::ContinuousBracket10Ev9Pics,
            CrDriveMode_CrDrive_Continuous_Bracket_20Ev_3pics => Self::ContinuousBracket20Ev3Pics,
            CrDriveMode_CrDrive_Continuous_Bracket_20Ev_5pics => Self::ContinuousBracket20Ev5Pics,
            CrDriveMode_CrDrive_Continuous_Bracket_30Ev_3pics => Self::ContinuousBracket30Ev3Pics,
            CrDriveMode_CrDrive_Continuous_Bracket_30Ev_5pics => Self::ContinuousBracket30Ev5Pics,
            CrDriveMode_CrDrive_Continuous_Bracket_03Ev_2pics_Plus => {
                Self::ContinuousBracket03Ev2PicsPlus
            }
            CrDriveMode_CrDrive_Continuous_Bracket_03Ev_2pics_Minus => {
                Self::ContinuousBracket03Ev2PicsMinus
            }
            CrDriveMode_CrDrive_Continuous_Bracket_03Ev_7pics => Self::ContinuousBracket03Ev7Pics,
            CrDriveMode_CrDrive_Continuous_Bracket_05Ev_2pics_Plus => {
                Self::ContinuousBracket05Ev2PicsPlus
            }
            CrDriveMode_CrDrive_Continuous_Bracket_05Ev_2pics_Minus => {
                Self::ContinuousBracket05Ev2PicsMinus
            }
            CrDriveMode_CrDrive_Continuous_Bracket_05Ev_7pics => Self::ContinuousBracket05Ev7Pics,
            CrDriveMode_CrDrive_Continuous_Bracket_07Ev_2pics_Plus => {
                Self::ContinuousBracket07Ev2PicsPlus
            }
            CrDriveMode_CrDrive_Continuous_Bracket_07Ev_2pics_Minus => {
                Self::ContinuousBracket07Ev2PicsMinus
            }
            CrDriveMode_CrDrive_Continuous_Bracket_07Ev_7pics => Self::ContinuousBracket07Ev7Pics,
            CrDriveMode_CrDrive_Continuous_Bracket_10Ev_2pics_Plus => {
                Self::ContinuousBracket10Ev2PicsPlus
            }
            CrDriveMode_CrDrive_Continuous_Bracket_10Ev_2pics_Minus => {
                Self::ContinuousBracket10Ev2PicsMinus
            }
            CrDriveMode_CrDrive_Continuous_Bracket_10Ev_7pics => Self::ContinuousBracket10Ev7Pics,
            CrDriveMode_CrDrive_Continuous_Bracket_13Ev_2pics_Plus => {
                Self::ContinuousBracket13Ev2PicsPlus
            }
            CrDriveMode_CrDrive_Continuous_Bracket_13Ev_2pics_Minus => {
                Self::ContinuousBracket13Ev2PicsMinus
            }
            CrDriveMode_CrDrive_Continuous_Bracket_13Ev_3pics => Self::ContinuousBracket13Ev3Pics,
            CrDriveMode_CrDrive_Continuous_Bracket_13Ev_5pics => Self::ContinuousBracket13Ev5Pics,
            CrDriveMode_CrDrive_Continuous_Bracket_13Ev_7pics => Self::ContinuousBracket13Ev7Pics,
            CrDriveMode_CrDrive_Continuous_Bracket_15Ev_2pics_Plus => {
                Self::ContinuousBracket15Ev2PicsPlus
            }
            CrDriveMode_CrDrive_Continuous_Bracket_15Ev_2pics_Minus => {
                Self::ContinuousBracket15Ev2PicsMinus
            }
            CrDriveMode_CrDrive_Continuous_Bracket_15Ev_3pics => Self::ContinuousBracket15Ev3Pics,
            CrDriveMode_CrDrive_Continuous_Bracket_15Ev_5pics => Self::ContinuousBracket15Ev5Pics,
            CrDriveMode_CrDrive_Continuous_Bracket_15Ev_7pics => Self::ContinuousBracket15Ev7Pics,
            CrDriveMode_CrDrive_Continuous_Bracket_17Ev_2pics_Plus => {
                Self::ContinuousBracket17Ev2PicsPlus
            }
            CrDriveMode_CrDrive_Continuous_Bracket_17Ev_2pics_Minus => {
                Self::ContinuousBracket17Ev2PicsMinus
            }
            CrDriveMode_CrDrive_Continuous_Bracket_17Ev_3pics => Self::ContinuousBracket17Ev3Pics,
            CrDriveMode_CrDrive_Continuous_Bracket_17Ev_5pics => Self::ContinuousBracket17Ev5Pics,
            CrDriveMode_CrDrive_Continuous_Bracket_17Ev_7pics => Self::ContinuousBracket17Ev7Pics,
            CrDriveMode_CrDrive_Continuous_Bracket_20Ev_2pics_Plus => {
                Self::ContinuousBracket20Ev2PicsPlus
            }
            CrDriveMode_CrDrive_Continuous_Bracket_20Ev_2pics_Minus => {
                Self::ContinuousBracket20Ev2PicsMinus
            }
            CrDriveMode_CrDrive_Continuous_Bracket_20Ev_7pics => Self::ContinuousBracket20Ev7Pics,
            CrDriveMode_CrDrive_Continuous_Bracket_23Ev_2pics_Plus => {
                Self::ContinuousBracket23Ev2PicsPlus
            }
            CrDriveMode_CrDrive_Continuous_Bracket_23Ev_2pics_Minus => {
                Self::ContinuousBracket23Ev2PicsMinus
            }
            CrDriveMode_CrDrive_Continuous_Bracket_23Ev_3pics => Self::ContinuousBracket23Ev3Pics,
            CrDriveMode_CrDrive_Continuous_Bracket_23Ev_5pics => Self::ContinuousBracket23Ev5Pics,
            CrDriveMode_CrDrive_Continuous_Bracket_25Ev_2pics_Plus => {
                Self::ContinuousBracket25Ev2PicsPlus
            }
            CrDriveMode_CrDrive_Continuous_Bracket_25Ev_2pics_Minus => {
                Self::ContinuousBracket25Ev2PicsMinus
            }
            CrDriveMode_CrDrive_Continuous_Bracket_25Ev_3pics => Self::ContinuousBracket25Ev3Pics,
            CrDriveMode_CrDrive_Continuous_Bracket_25Ev_5pics => Self::ContinuousBracket25Ev5Pics,
            CrDriveMode_CrDrive_Continuous_Bracket_27Ev_2pics_Plus => {
                Self::ContinuousBracket27Ev2PicsPlus
            }
            CrDriveMode_CrDrive_Continuous_Bracket_27Ev_2pics_Minus => {
                Self::ContinuousBracket27Ev2PicsMinus
            }
            CrDriveMode_CrDrive_Continuous_Bracket_27Ev_3pics => Self::ContinuousBracket27Ev3Pics,
            CrDriveMode_CrDrive_Continuous_Bracket_27Ev_5pics => Self::ContinuousBracket27Ev5Pics,
            CrDriveMode_CrDrive_Continuous_Bracket_30Ev_2pics_Plus => {
                Self::ContinuousBracket30Ev2PicsPlus
            }
            CrDriveMode_CrDrive_Continuous_Bracket_30Ev_2pics_Minus => {
                Self::ContinuousBracket30Ev2PicsMinus
            }
            CrDriveMode_CrDrive_Single_Bracket_03Ev_3pics => Self::SingleBracket03Ev3Pics,
            CrDriveMode_CrDrive_Single_Bracket_03Ev_5pics => Self::SingleBracket03Ev5Pics,
            CrDriveMode_CrDrive_Single_Bracket_03Ev_9pics => Self::SingleBracket03Ev9Pics,
            CrDriveMode_CrDrive_Single_Bracket_05Ev_3pics => Self::SingleBracket05Ev3Pics,
            CrDriveMode_CrDrive_Single_Bracket_05Ev_5pics => Self::SingleBracket05Ev5Pics,
            CrDriveMode_CrDrive_Single_Bracket_05Ev_9pics => Self::SingleBracket05Ev9Pics,
            CrDriveMode_CrDrive_Single_Bracket_07Ev_3pics => Self::SingleBracket07Ev3Pics,
            CrDriveMode_CrDrive_Single_Bracket_07Ev_5pics => Self::SingleBracket07Ev5Pics,
            CrDriveMode_CrDrive_Single_Bracket_07Ev_9pics => Self::SingleBracket07Ev9Pics,
            CrDriveMode_CrDrive_Single_Bracket_10Ev_3pics => Self::SingleBracket10Ev3Pics,
            CrDriveMode_CrDrive_Single_Bracket_10Ev_5pics => Self::SingleBracket10Ev5Pics,
            CrDriveMode_CrDrive_Single_Bracket_10Ev_9pics => Self::SingleBracket10Ev9Pics,
            CrDriveMode_CrDrive_Single_Bracket_20Ev_3pics => Self::SingleBracket20Ev3Pics,
            CrDriveMode_CrDrive_Single_Bracket_20Ev_5pics => Self::SingleBracket20Ev5Pics,
            CrDriveMode_CrDrive_Single_Bracket_30Ev_3pics => Self::SingleBracket30Ev3Pics,
            CrDriveMode_CrDrive_Single_Bracket_30Ev_5pics => Self::SingleBracket30Ev5Pics,
            CrDriveMode_CrDrive_Single_Bracket_03Ev_2pics_Plus => Self::SingleBracket03Ev2PicsPlus,
            CrDriveMode_CrDrive_Single_Bracket_03Ev_2pics_Minus => {
                Self::SingleBracket03Ev2PicsMinus
            }
            CrDriveMode_CrDrive_Single_Bracket_03Ev_7pics => Self::SingleBracket03Ev7Pics,
            CrDriveMode_CrDrive_Single_Bracket_05Ev_2pics_Plus => Self::SingleBracket05Ev2PicsPlus,
            CrDriveMode_CrDrive_Single_Bracket_05Ev_2pics_Minus => {
                Self::SingleBracket05Ev2PicsMinus
            }
            CrDriveMode_CrDrive_Single_Bracket_05Ev_7pics => Self::SingleBracket05Ev7Pics,
            CrDriveMode_CrDrive_Single_Bracket_07Ev_2pics_Plus => Self::SingleBracket07Ev2PicsPlus,
            CrDriveMode_CrDrive_Single_Bracket_07Ev_2pics_Minus => {
                Self::SingleBracket07Ev2PicsMinus
            }
            CrDriveMode_CrDrive_Single_Bracket_07Ev_7pics => Self::SingleBracket07Ev7Pics,
            CrDriveMode_CrDrive_Single_Bracket_10Ev_2pics_Plus => Self::SingleBracket10Ev2PicsPlus,
            CrDriveMode_CrDrive_Single_Bracket_10Ev_2pics_Minus => {
                Self::SingleBracket10Ev2PicsMinus
            }
            CrDriveMode_CrDrive_Single_Bracket_10Ev_7pics => Self::SingleBracket10Ev7Pics,
            CrDriveMode_CrDrive_Single_Bracket_13Ev_2pics_Plus => Self::SingleBracket13Ev2PicsPlus,
            CrDriveMode_CrDrive_Single_Bracket_13Ev_2pics_Minus => {
                Self::SingleBracket13Ev2PicsMinus
            }
            CrDriveMode_CrDrive_Single_Bracket_13Ev_3pics => Self::SingleBracket13Ev3Pics,
            CrDriveMode_CrDrive_Single_Bracket_13Ev_5pics => Self::SingleBracket13Ev5Pics,
            CrDriveMode_CrDrive_Single_Bracket_13Ev_7pics => Self::SingleBracket13Ev7Pics,
            CrDriveMode_CrDrive_Single_Bracket_15Ev_2pics_Plus => Self::SingleBracket15Ev2PicsPlus,
            CrDriveMode_CrDrive_Single_Bracket_15Ev_2pics_Minus => {
                Self::SingleBracket15Ev2PicsMinus
            }
            CrDriveMode_CrDrive_Single_Bracket_15Ev_3pics => Self::SingleBracket15Ev3Pics,
            CrDriveMode_CrDrive_Single_Bracket_15Ev_5pics => Self::SingleBracket15Ev5Pics,
            CrDriveMode_CrDrive_Single_Bracket_15Ev_7pics => Self::SingleBracket15Ev7Pics,
            CrDriveMode_CrDrive_Single_Bracket_17Ev_2pics_Plus => Self::SingleBracket17Ev2PicsPlus,
            CrDriveMode_CrDrive_Single_Bracket_17Ev_2pics_Minus => {
                Self::SingleBracket17Ev2PicsMinus
            }
            CrDriveMode_CrDrive_Single_Bracket_17Ev_3pics => Self::SingleBracket17Ev3Pics,
            CrDriveMode_CrDrive_Single_Bracket_17Ev_5pics => Self::SingleBracket17Ev5Pics,
            CrDriveMode_CrDrive_Single_Bracket_17Ev_7pics => Self::SingleBracket17Ev7Pics,
            CrDriveMode_CrDrive_Single_Bracket_20Ev_2pics_Plus => Self::SingleBracket20Ev2PicsPlus,
            CrDriveMode_CrDrive_Single_Bracket_20Ev_2pics_Minus => {
                Self::SingleBracket20Ev2PicsMinus
            }
            CrDriveMode_CrDrive_Single_Bracket_20Ev_7pics => Self::SingleBracket20Ev7Pics,
            CrDriveMode_CrDrive_Single_Bracket_23Ev_2pics_Plus => Self::SingleBracket23Ev2PicsPlus,
            CrDriveMode_CrDrive_Single_Bracket_23Ev_2pics_Minus => {
                Self::SingleBracket23Ev2PicsMinus
            }
            CrDriveMode_CrDrive_Single_Bracket_23Ev_3pics => Self::SingleBracket23Ev3Pics,
            CrDriveMode_CrDrive_Single_Bracket_23Ev_5pics => Self::SingleBracket23Ev5Pics,
            CrDriveMode_CrDrive_Single_Bracket_25Ev_2pics_Plus => Self::SingleBracket25Ev2PicsPlus,
            CrDriveMode_CrDrive_Single_Bracket_25Ev_2pics_Minus => {
                Self::SingleBracket25Ev2PicsMinus
            }
            CrDriveMode_CrDrive_Single_Bracket_25Ev_3pics => Self::SingleBracket25Ev3Pics,
            CrDriveMode_CrDrive_Single_Bracket_25Ev_5pics => Self::SingleBracket25Ev5Pics,
            CrDriveMode_CrDrive_Single_Bracket_27Ev_2pics_Plus => Self::SingleBracket27Ev2PicsPlus,
            CrDriveMode_CrDrive_Single_Bracket_27Ev_2pics_Minus => {
                Self::SingleBracket27Ev2PicsMinus
            }
            CrDriveMode_CrDrive_Single_Bracket_27Ev_3pics => Self::SingleBracket27Ev3Pics,
            CrDriveMode_CrDrive_Single_Bracket_27Ev_5pics => Self::SingleBracket27Ev5Pics,
            CrDriveMode_CrDrive_Single_Bracket_30Ev_2pics_Plus => Self::SingleBracket30Ev2PicsPlus,
            CrDriveMode_CrDrive_Single_Bracket_30Ev_2pics_Minus => {
                Self::SingleBracket30Ev2PicsMinus
            }
            CrDriveMode_CrDrive_WB_Bracket_Lo => Self::WbBracketLo,
            CrDriveMode_CrDrive_WB_Bracket_Hi => Self::WbBracketHi,
            CrDriveMode_CrDrive_DRO_Bracket_Lo => Self::DroBracketLo,
            CrDriveMode_CrDrive_DRO_Bracket_Hi => Self::DroBracketHi,
            CrDriveMode_CrDrive_Continuous_Timer_3pics => Self::ContinuousTimer3Pics,
            CrDriveMode_CrDrive_Continuous_Timer_5pics => Self::ContinuousTimer5Pics,
            CrDriveMode_CrDrive_Continuous_Timer_2s_3pics => Self::ContinuousTimer2s3Pics,
            CrDriveMode_CrDrive_Continuous_Timer_2s_5pics => Self::ContinuousTimer2s5Pics,
            CrDriveMode_CrDrive_Continuous_Timer_5s_3pics => Self::ContinuousTimer5s3Pics,
            CrDriveMode_CrDrive_Continuous_Timer_5s_5pics => Self::ContinuousTimer5s5Pics,
            CrDriveMode_CrDrive_LPF_Bracket => Self::LpfBracket,
            CrDriveMode_CrDrive_RemoteCommander => Self::RemoteCommander,
            CrDriveMode_CrDrive_MirrorUp => Self::MirrorUp,
            CrDriveMode_CrDrive_SelfPortrait_1 => Self::SelfPortrait1,
            CrDriveMode_CrDrive_SelfPortrait_2 => Self::SelfPortrait2,
            _ => return None,
        })
    }
}

/// Shutter type for interval recording (timelapse)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum IntervalRecShutterType {
    Auto = 1,
    Mechanical = 2,
    Electronic = 3,
}

impl IntervalRecShutterType {
    pub fn as_raw(self) -> u64 {
        self as u64
    }

    pub fn from_raw(value: u64) -> Option<Self> {
        Some(match value as u8 {
            1 => Self::Auto,
            2 => Self::Mechanical,
            3 => Self::Electronic,
            _ => return None,
        })
    }
}

impl std::fmt::Display for IntervalRecShutterType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Auto => write!(f, "Auto"),
            Self::Mechanical => write!(f, "Mechanical"),
            Self::Electronic => write!(f, "Electronic"),
        }
    }
}

pub fn description(code: DevicePropertyCode) -> &'static str {
    match code {
        DevicePropertyCode::DriveMode => {
            "Single shot takes one photo per press. Continuous shoots multiple frames while held. Self-timer delays the shot. Bracket takes multiple exposures."
        }
        DevicePropertyCode::BracketOrder => {
            "Sequence of bracketed exposures. 0/−/+ starts with normal exposure. −/0/+ starts with underexposed."
        }
        DevicePropertyCode::RecordingSelfTimer => {
            "Delay before the shutter fires. Useful for self-portraits or to avoid camera shake from pressing the button."
        }
        DevicePropertyCode::IntervalRecMode => {
            "Captures images at set intervals for time-lapse photography. The camera can optionally compile them into a video."
        }
        DevicePropertyCode::IntervalRecShutterType => {
            "Shutter type for interval shooting. Auto selects automatically. Mechanical uses the physical shutter. Electronic is silent but may cause rolling shutter with moving subjects."
        }
        _ => "",
    }
}

pub fn display_name(code: DevicePropertyCode) -> &'static str {
    match code {
        DevicePropertyCode::DriveMode => "Drive Mode",
        DevicePropertyCode::BracketOrder => "Bracket Order",
        DevicePropertyCode::RecordingSelfTimer => "Self Timer",
        DevicePropertyCode::RecordingSelfTimerCountTime => "Self Timer Duration",
        DevicePropertyCode::RecordingSelfTimerStatus => "Self Timer Status",
        DevicePropertyCode::IntervalRecNumberOfShots => "Interval Shots",
        DevicePropertyCode::IntervalRecShootingInterval => "Interval Time",
        DevicePropertyCode::IntervalRecStatus => "Interval Status",
        DevicePropertyCode::IntervalRecMode => "Interval Mode",
        DevicePropertyCode::IntervalRecShutterType => "Interval Shutter Type",
        DevicePropertyCode::ContinuousShootingSpotBoostStatus => "Burst Boost Status",
        DevicePropertyCode::ContinuousShootingSpotBoostFrameSpeed => "Burst Boost Speed",
        _ => code.name(),
    }
}

pub fn value_type(code: DevicePropertyCode) -> Option<PropertyValueType> {
    use PropertyValueType as V;

    Some(match code {
        DevicePropertyCode::DriveMode => V::DriveMode,
        DevicePropertyCode::IntervalRecShutterType => V::IntervalRecShutterType,
        _ => return None,
    })
}
