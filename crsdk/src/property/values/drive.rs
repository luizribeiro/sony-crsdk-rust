//! Drive mode value types.

use super::super::PropertyValue;
use crate::types::ToCrsdk;

/// Drive mode / shooting mode settings
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum DriveMode {
    /// Single shot mode - takes one photo per shutter press
    Single = crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Single,
    /// High-speed continuous shooting
    ContinuousHi = crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Hi,
    /// High-speed continuous shooting with enhanced buffer capacity
    ContinuousHiPlus = crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Hi_Plus,
    /// High-speed continuous shooting with live view display
    ContinuousHiLive = crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Hi_Live,
    /// Low-speed continuous shooting
    ContinuousLo = crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Lo,
    /// Standard continuous shooting
    Continuous = crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous,
    /// Continuous shooting prioritizing maximum frame rate
    ContinuousSpeedPriority = crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_SpeedPriority,
    /// Medium-speed continuous shooting
    ContinuousMid = crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Mid,
    /// Medium-speed continuous shooting with live view display
    ContinuousMidLive = crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Mid_Live,
    /// Low-speed continuous shooting with live view display
    ContinuousLoLive = crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Lo_Live,
    /// Single burst shooting at low speed
    SingleBurstShootingLo = crsdk_sys::SCRSDK::CrDriveMode_CrDrive_SingleBurstShooting_lo,
    /// Single burst shooting at medium speed
    SingleBurstShootingMid = crsdk_sys::SCRSDK::CrDriveMode_CrDrive_SingleBurstShooting_mid,
    /// Single burst shooting at high speed
    SingleBurstShootingHi = crsdk_sys::SCRSDK::CrDriveMode_CrDrive_SingleBurstShooting_hi,
    /// Focus bracketing across multiple focal planes
    FocusBracket = crsdk_sys::SCRSDK::CrDriveMode_CrDrive_FocusBracket,
    /// Time-lapse photography mode
    Timelapse = crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Timelapse,
    /// 2-second self-timer
    Timer2s = crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Timer_2s,
    /// 5-second self-timer
    Timer5s = crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Timer_5s,
    /// 10-second self-timer
    Timer10s = crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Timer_10s,
    /// Continuous exposure bracket: 0.3 EV step, 3 shots
    ContinuousBracket03Ev3Pics =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Bracket_03Ev_3pics,
    /// Continuous exposure bracket: 0.3 EV step, 5 shots
    ContinuousBracket03Ev5Pics =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Bracket_03Ev_5pics,
    /// Continuous exposure bracket: 0.3 EV step, 9 shots
    ContinuousBracket03Ev9Pics =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Bracket_03Ev_9pics,
    /// Continuous exposure bracket: 0.5 EV step, 3 shots
    ContinuousBracket05Ev3Pics =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Bracket_05Ev_3pics,
    /// Continuous exposure bracket: 0.5 EV step, 5 shots
    ContinuousBracket05Ev5Pics =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Bracket_05Ev_5pics,
    /// Continuous exposure bracket: 0.5 EV step, 9 shots
    ContinuousBracket05Ev9Pics =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Bracket_05Ev_9pics,
    /// Continuous exposure bracket: 0.7 EV step, 3 shots
    ContinuousBracket07Ev3Pics =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Bracket_07Ev_3pics,
    /// Continuous exposure bracket: 0.7 EV step, 5 shots
    ContinuousBracket07Ev5Pics =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Bracket_07Ev_5pics,
    /// Continuous exposure bracket: 0.7 EV step, 9 shots
    ContinuousBracket07Ev9Pics =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Bracket_07Ev_9pics,
    /// Continuous exposure bracket: 1.0 EV step, 3 shots
    ContinuousBracket10Ev3Pics =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Bracket_10Ev_3pics,
    /// Continuous exposure bracket: 1.0 EV step, 5 shots
    ContinuousBracket10Ev5Pics =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Bracket_10Ev_5pics,
    /// Continuous exposure bracket: 1.0 EV step, 9 shots
    ContinuousBracket10Ev9Pics =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Bracket_10Ev_9pics,
    /// Continuous exposure bracket: 2.0 EV step, 3 shots
    ContinuousBracket20Ev3Pics =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Bracket_20Ev_3pics,
    /// Continuous exposure bracket: 2.0 EV step, 5 shots
    ContinuousBracket20Ev5Pics =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Bracket_20Ev_5pics,
    /// Continuous exposure bracket: 3.0 EV step, 3 shots
    ContinuousBracket30Ev3Pics =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Bracket_30Ev_3pics,
    /// Continuous exposure bracket: 3.0 EV step, 5 shots
    ContinuousBracket30Ev5Pics =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Bracket_30Ev_5pics,
    /// Continuous exposure bracket: 0.3 EV step, 2 shots (overexposed only)
    ContinuousBracket03Ev2PicsPlus =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Bracket_03Ev_2pics_Plus,
    /// Continuous exposure bracket: 0.3 EV step, 2 shots (underexposed only)
    ContinuousBracket03Ev2PicsMinus =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Bracket_03Ev_2pics_Minus,
    /// Continuous exposure bracket: 0.3 EV step, 7 shots
    ContinuousBracket03Ev7Pics =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Bracket_03Ev_7pics,
    /// Continuous exposure bracket: 0.5 EV step, 2 shots (overexposed only)
    ContinuousBracket05Ev2PicsPlus =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Bracket_05Ev_2pics_Plus,
    /// Continuous exposure bracket: 0.5 EV step, 2 shots (underexposed only)
    ContinuousBracket05Ev2PicsMinus =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Bracket_05Ev_2pics_Minus,
    /// Continuous exposure bracket: 0.5 EV step, 7 shots
    ContinuousBracket05Ev7Pics =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Bracket_05Ev_7pics,
    /// Continuous exposure bracket: 0.7 EV step, 2 shots (overexposed only)
    ContinuousBracket07Ev2PicsPlus =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Bracket_07Ev_2pics_Plus,
    /// Continuous exposure bracket: 0.7 EV step, 2 shots (underexposed only)
    ContinuousBracket07Ev2PicsMinus =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Bracket_07Ev_2pics_Minus,
    /// Continuous exposure bracket: 0.7 EV step, 7 shots
    ContinuousBracket07Ev7Pics =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Bracket_07Ev_7pics,
    /// Continuous exposure bracket: 1.0 EV step, 2 shots (overexposed only)
    ContinuousBracket10Ev2PicsPlus =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Bracket_10Ev_2pics_Plus,
    /// Continuous exposure bracket: 1.0 EV step, 2 shots (underexposed only)
    ContinuousBracket10Ev2PicsMinus =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Bracket_10Ev_2pics_Minus,
    /// Continuous exposure bracket: 1.0 EV step, 7 shots
    ContinuousBracket10Ev7Pics =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Bracket_10Ev_7pics,
    /// Continuous exposure bracket: 1.3 EV step, 2 shots (overexposed only)
    ContinuousBracket13Ev2PicsPlus =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Bracket_13Ev_2pics_Plus,
    /// Continuous exposure bracket: 1.3 EV step, 2 shots (underexposed only)
    ContinuousBracket13Ev2PicsMinus =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Bracket_13Ev_2pics_Minus,
    /// Continuous exposure bracket: 1.3 EV step, 3 shots
    ContinuousBracket13Ev3Pics =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Bracket_13Ev_3pics,
    /// Continuous exposure bracket: 1.3 EV step, 5 shots
    ContinuousBracket13Ev5Pics =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Bracket_13Ev_5pics,
    /// Continuous exposure bracket: 1.3 EV step, 7 shots
    ContinuousBracket13Ev7Pics =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Bracket_13Ev_7pics,
    /// Continuous exposure bracket: 1.5 EV step, 2 shots (overexposed only)
    ContinuousBracket15Ev2PicsPlus =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Bracket_15Ev_2pics_Plus,
    /// Continuous exposure bracket: 1.5 EV step, 2 shots (underexposed only)
    ContinuousBracket15Ev2PicsMinus =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Bracket_15Ev_2pics_Minus,
    /// Continuous exposure bracket: 1.5 EV step, 3 shots
    ContinuousBracket15Ev3Pics =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Bracket_15Ev_3pics,
    /// Continuous exposure bracket: 1.5 EV step, 5 shots
    ContinuousBracket15Ev5Pics =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Bracket_15Ev_5pics,
    /// Continuous exposure bracket: 1.5 EV step, 7 shots
    ContinuousBracket15Ev7Pics =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Bracket_15Ev_7pics,
    /// Continuous exposure bracket: 1.7 EV step, 2 shots (overexposed only)
    ContinuousBracket17Ev2PicsPlus =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Bracket_17Ev_2pics_Plus,
    /// Continuous exposure bracket: 1.7 EV step, 2 shots (underexposed only)
    ContinuousBracket17Ev2PicsMinus =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Bracket_17Ev_2pics_Minus,
    /// Continuous exposure bracket: 1.7 EV step, 3 shots
    ContinuousBracket17Ev3Pics =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Bracket_17Ev_3pics,
    /// Continuous exposure bracket: 1.7 EV step, 5 shots
    ContinuousBracket17Ev5Pics =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Bracket_17Ev_5pics,
    /// Continuous exposure bracket: 1.7 EV step, 7 shots
    ContinuousBracket17Ev7Pics =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Bracket_17Ev_7pics,
    /// Continuous exposure bracket: 2.0 EV step, 2 shots (overexposed only)
    ContinuousBracket20Ev2PicsPlus =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Bracket_20Ev_2pics_Plus,
    /// Continuous exposure bracket: 2.0 EV step, 2 shots (underexposed only)
    ContinuousBracket20Ev2PicsMinus =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Bracket_20Ev_2pics_Minus,
    /// Continuous exposure bracket: 2.0 EV step, 7 shots
    ContinuousBracket20Ev7Pics =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Bracket_20Ev_7pics,
    /// Continuous exposure bracket: 2.3 EV step, 2 shots (overexposed only)
    ContinuousBracket23Ev2PicsPlus =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Bracket_23Ev_2pics_Plus,
    /// Continuous exposure bracket: 2.3 EV step, 2 shots (underexposed only)
    ContinuousBracket23Ev2PicsMinus =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Bracket_23Ev_2pics_Minus,
    /// Continuous exposure bracket: 2.3 EV step, 3 shots
    ContinuousBracket23Ev3Pics =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Bracket_23Ev_3pics,
    /// Continuous exposure bracket: 2.3 EV step, 5 shots
    ContinuousBracket23Ev5Pics =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Bracket_23Ev_5pics,
    /// Continuous exposure bracket: 2.5 EV step, 2 shots (overexposed only)
    ContinuousBracket25Ev2PicsPlus =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Bracket_25Ev_2pics_Plus,
    /// Continuous exposure bracket: 2.5 EV step, 2 shots (underexposed only)
    ContinuousBracket25Ev2PicsMinus =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Bracket_25Ev_2pics_Minus,
    /// Continuous exposure bracket: 2.5 EV step, 3 shots
    ContinuousBracket25Ev3Pics =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Bracket_25Ev_3pics,
    /// Continuous exposure bracket: 2.5 EV step, 5 shots
    ContinuousBracket25Ev5Pics =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Bracket_25Ev_5pics,
    /// Continuous exposure bracket: 2.7 EV step, 2 shots (overexposed only)
    ContinuousBracket27Ev2PicsPlus =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Bracket_27Ev_2pics_Plus,
    /// Continuous exposure bracket: 2.7 EV step, 2 shots (underexposed only)
    ContinuousBracket27Ev2PicsMinus =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Bracket_27Ev_2pics_Minus,
    /// Continuous exposure bracket: 2.7 EV step, 3 shots
    ContinuousBracket27Ev3Pics =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Bracket_27Ev_3pics,
    /// Continuous exposure bracket: 2.7 EV step, 5 shots
    ContinuousBracket27Ev5Pics =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Bracket_27Ev_5pics,
    /// Continuous exposure bracket: 3.0 EV step, 2 shots (overexposed only)
    ContinuousBracket30Ev2PicsPlus =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Bracket_30Ev_2pics_Plus,
    /// Continuous exposure bracket: 3.0 EV step, 2 shots (underexposed only)
    ContinuousBracket30Ev2PicsMinus =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Bracket_30Ev_2pics_Minus,
    /// Single exposure bracket: 0.3 EV step, 3 shots
    SingleBracket03Ev3Pics = crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Single_Bracket_03Ev_3pics,
    /// Single exposure bracket: 0.3 EV step, 5 shots
    SingleBracket03Ev5Pics = crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Single_Bracket_03Ev_5pics,
    /// Single exposure bracket: 0.3 EV step, 9 shots
    SingleBracket03Ev9Pics = crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Single_Bracket_03Ev_9pics,
    /// Single exposure bracket: 0.5 EV step, 3 shots
    SingleBracket05Ev3Pics = crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Single_Bracket_05Ev_3pics,
    /// Single exposure bracket: 0.5 EV step, 5 shots
    SingleBracket05Ev5Pics = crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Single_Bracket_05Ev_5pics,
    /// Single exposure bracket: 0.5 EV step, 9 shots
    SingleBracket05Ev9Pics = crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Single_Bracket_05Ev_9pics,
    /// Single exposure bracket: 0.7 EV step, 3 shots
    SingleBracket07Ev3Pics = crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Single_Bracket_07Ev_3pics,
    /// Single exposure bracket: 0.7 EV step, 5 shots
    SingleBracket07Ev5Pics = crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Single_Bracket_07Ev_5pics,
    /// Single exposure bracket: 0.7 EV step, 9 shots
    SingleBracket07Ev9Pics = crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Single_Bracket_07Ev_9pics,
    /// Single exposure bracket: 1.0 EV step, 3 shots
    SingleBracket10Ev3Pics = crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Single_Bracket_10Ev_3pics,
    /// Single exposure bracket: 1.0 EV step, 5 shots
    SingleBracket10Ev5Pics = crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Single_Bracket_10Ev_5pics,
    /// Single exposure bracket: 1.0 EV step, 9 shots
    SingleBracket10Ev9Pics = crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Single_Bracket_10Ev_9pics,
    /// Single exposure bracket: 2.0 EV step, 3 shots
    SingleBracket20Ev3Pics = crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Single_Bracket_20Ev_3pics,
    /// Single exposure bracket: 2.0 EV step, 5 shots
    SingleBracket20Ev5Pics = crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Single_Bracket_20Ev_5pics,
    /// Single exposure bracket: 3.0 EV step, 3 shots
    SingleBracket30Ev3Pics = crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Single_Bracket_30Ev_3pics,
    /// Single exposure bracket: 3.0 EV step, 5 shots
    SingleBracket30Ev5Pics = crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Single_Bracket_30Ev_5pics,
    /// Single exposure bracket: 0.3 EV step, 2 shots (overexposed only)
    SingleBracket03Ev2PicsPlus =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Single_Bracket_03Ev_2pics_Plus,
    /// Single exposure bracket: 0.3 EV step, 2 shots (underexposed only)
    SingleBracket03Ev2PicsMinus =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Single_Bracket_03Ev_2pics_Minus,
    /// Single exposure bracket: 0.3 EV step, 7 shots
    SingleBracket03Ev7Pics = crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Single_Bracket_03Ev_7pics,
    /// Single exposure bracket: 0.5 EV step, 2 shots (overexposed only)
    SingleBracket05Ev2PicsPlus =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Single_Bracket_05Ev_2pics_Plus,
    /// Single exposure bracket: 0.5 EV step, 2 shots (underexposed only)
    SingleBracket05Ev2PicsMinus =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Single_Bracket_05Ev_2pics_Minus,
    /// Single exposure bracket: 0.5 EV step, 7 shots
    SingleBracket05Ev7Pics = crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Single_Bracket_05Ev_7pics,
    /// Single exposure bracket: 0.7 EV step, 2 shots (overexposed only)
    SingleBracket07Ev2PicsPlus =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Single_Bracket_07Ev_2pics_Plus,
    /// Single exposure bracket: 0.7 EV step, 2 shots (underexposed only)
    SingleBracket07Ev2PicsMinus =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Single_Bracket_07Ev_2pics_Minus,
    /// Single exposure bracket: 0.7 EV step, 7 shots
    SingleBracket07Ev7Pics = crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Single_Bracket_07Ev_7pics,
    /// Single exposure bracket: 1.0 EV step, 2 shots (overexposed only)
    SingleBracket10Ev2PicsPlus =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Single_Bracket_10Ev_2pics_Plus,
    /// Single exposure bracket: 1.0 EV step, 2 shots (underexposed only)
    SingleBracket10Ev2PicsMinus =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Single_Bracket_10Ev_2pics_Minus,
    /// Single exposure bracket: 1.0 EV step, 7 shots
    SingleBracket10Ev7Pics = crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Single_Bracket_10Ev_7pics,
    /// Single exposure bracket: 1.3 EV step, 2 shots (overexposed only)
    SingleBracket13Ev2PicsPlus =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Single_Bracket_13Ev_2pics_Plus,
    /// Single exposure bracket: 1.3 EV step, 2 shots (underexposed only)
    SingleBracket13Ev2PicsMinus =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Single_Bracket_13Ev_2pics_Minus,
    /// Single exposure bracket: 1.3 EV step, 3 shots
    SingleBracket13Ev3Pics = crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Single_Bracket_13Ev_3pics,
    /// Single exposure bracket: 1.3 EV step, 5 shots
    SingleBracket13Ev5Pics = crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Single_Bracket_13Ev_5pics,
    /// Single exposure bracket: 1.3 EV step, 7 shots
    SingleBracket13Ev7Pics = crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Single_Bracket_13Ev_7pics,
    /// Single exposure bracket: 1.5 EV step, 2 shots (overexposed only)
    SingleBracket15Ev2PicsPlus =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Single_Bracket_15Ev_2pics_Plus,
    /// Single exposure bracket: 1.5 EV step, 2 shots (underexposed only)
    SingleBracket15Ev2PicsMinus =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Single_Bracket_15Ev_2pics_Minus,
    /// Single exposure bracket: 1.5 EV step, 3 shots
    SingleBracket15Ev3Pics = crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Single_Bracket_15Ev_3pics,
    /// Single exposure bracket: 1.5 EV step, 5 shots
    SingleBracket15Ev5Pics = crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Single_Bracket_15Ev_5pics,
    /// Single exposure bracket: 1.5 EV step, 7 shots
    SingleBracket15Ev7Pics = crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Single_Bracket_15Ev_7pics,
    /// Single exposure bracket: 1.7 EV step, 2 shots (overexposed only)
    SingleBracket17Ev2PicsPlus =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Single_Bracket_17Ev_2pics_Plus,
    /// Single exposure bracket: 1.7 EV step, 2 shots (underexposed only)
    SingleBracket17Ev2PicsMinus =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Single_Bracket_17Ev_2pics_Minus,
    /// Single exposure bracket: 1.7 EV step, 3 shots
    SingleBracket17Ev3Pics = crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Single_Bracket_17Ev_3pics,
    /// Single exposure bracket: 1.7 EV step, 5 shots
    SingleBracket17Ev5Pics = crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Single_Bracket_17Ev_5pics,
    /// Single exposure bracket: 1.7 EV step, 7 shots
    SingleBracket17Ev7Pics = crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Single_Bracket_17Ev_7pics,
    /// Single exposure bracket: 2.0 EV step, 2 shots (overexposed only)
    SingleBracket20Ev2PicsPlus =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Single_Bracket_20Ev_2pics_Plus,
    /// Single exposure bracket: 2.0 EV step, 2 shots (underexposed only)
    SingleBracket20Ev2PicsMinus =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Single_Bracket_20Ev_2pics_Minus,
    /// Single exposure bracket: 2.0 EV step, 7 shots
    SingleBracket20Ev7Pics = crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Single_Bracket_20Ev_7pics,
    /// Single exposure bracket: 2.3 EV step, 2 shots (overexposed only)
    SingleBracket23Ev2PicsPlus =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Single_Bracket_23Ev_2pics_Plus,
    /// Single exposure bracket: 2.3 EV step, 2 shots (underexposed only)
    SingleBracket23Ev2PicsMinus =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Single_Bracket_23Ev_2pics_Minus,
    /// Single exposure bracket: 2.3 EV step, 3 shots
    SingleBracket23Ev3Pics = crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Single_Bracket_23Ev_3pics,
    /// Single exposure bracket: 2.3 EV step, 5 shots
    SingleBracket23Ev5Pics = crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Single_Bracket_23Ev_5pics,
    /// Single exposure bracket: 2.5 EV step, 2 shots (overexposed only)
    SingleBracket25Ev2PicsPlus =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Single_Bracket_25Ev_2pics_Plus,
    /// Single exposure bracket: 2.5 EV step, 2 shots (underexposed only)
    SingleBracket25Ev2PicsMinus =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Single_Bracket_25Ev_2pics_Minus,
    /// Single exposure bracket: 2.5 EV step, 3 shots
    SingleBracket25Ev3Pics = crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Single_Bracket_25Ev_3pics,
    /// Single exposure bracket: 2.5 EV step, 5 shots
    SingleBracket25Ev5Pics = crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Single_Bracket_25Ev_5pics,
    /// Single exposure bracket: 2.7 EV step, 2 shots (overexposed only)
    SingleBracket27Ev2PicsPlus =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Single_Bracket_27Ev_2pics_Plus,
    /// Single exposure bracket: 2.7 EV step, 2 shots (underexposed only)
    SingleBracket27Ev2PicsMinus =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Single_Bracket_27Ev_2pics_Minus,
    /// Single exposure bracket: 2.7 EV step, 3 shots
    SingleBracket27Ev3Pics = crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Single_Bracket_27Ev_3pics,
    /// Single exposure bracket: 2.7 EV step, 5 shots
    SingleBracket27Ev5Pics = crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Single_Bracket_27Ev_5pics,
    /// Single exposure bracket: 3.0 EV step, 2 shots (overexposed only)
    SingleBracket30Ev2PicsPlus =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Single_Bracket_30Ev_2pics_Plus,
    /// Single exposure bracket: 3.0 EV step, 2 shots (underexposed only)
    SingleBracket30Ev2PicsMinus =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Single_Bracket_30Ev_2pics_Minus,
    /// White balance bracketing with low deviation
    WbBracketLo = crsdk_sys::SCRSDK::CrDriveMode_CrDrive_WB_Bracket_Lo,
    /// White balance bracketing with high deviation
    WbBracketHi = crsdk_sys::SCRSDK::CrDriveMode_CrDrive_WB_Bracket_Hi,
    /// DRO (Dynamic Range Optimizer) bracketing with low levels
    DroBracketLo = crsdk_sys::SCRSDK::CrDriveMode_CrDrive_DRO_Bracket_Lo,
    /// DRO (Dynamic Range Optimizer) bracketing with high levels
    DroBracketHi = crsdk_sys::SCRSDK::CrDriveMode_CrDrive_DRO_Bracket_Hi,
    /// Continuous shooting with timer: 3 shots
    ContinuousTimer3Pics = crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Timer_3pics,
    /// Continuous shooting with timer: 5 shots
    ContinuousTimer5Pics = crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Timer_5pics,
    /// Continuous shooting with 2s timer: 3 shots
    ContinuousTimer2s3Pics = crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Timer_2s_3pics,
    /// Continuous shooting with 2s timer: 5 shots
    ContinuousTimer2s5Pics = crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Timer_2s_5pics,
    /// Continuous shooting with 5s timer: 3 shots
    ContinuousTimer5s3Pics = crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Timer_5s_3pics,
    /// Continuous shooting with 5s timer: 5 shots
    ContinuousTimer5s5Pics = crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Timer_5s_5pics,
    /// Low-pass filter effect bracketing
    LpfBracket = crsdk_sys::SCRSDK::CrDriveMode_CrDrive_LPF_Bracket,
    /// Remote commander control mode
    RemoteCommander = crsdk_sys::SCRSDK::CrDriveMode_CrDrive_RemoteCommander,
    /// Mirror lock-up mode to reduce vibration
    MirrorUp = crsdk_sys::SCRSDK::CrDriveMode_CrDrive_MirrorUp,
    /// Self-portrait mode with 2-second timer
    SelfPortrait1 = crsdk_sys::SCRSDK::CrDriveMode_CrDrive_SelfPortrait_1,
    /// Self-portrait mode with 5-second timer
    SelfPortrait2 = crsdk_sys::SCRSDK::CrDriveMode_CrDrive_SelfPortrait_2,
}

impl PropertyValue for DriveMode {
    fn from_raw(raw: u64) -> Option<Self> {
        use crsdk_sys::SCRSDK::*;
        let value = raw as u32;
        Some(match value {
            x if x == CrDriveMode_CrDrive_Single => Self::Single,
            x if x == CrDriveMode_CrDrive_Continuous_Hi => Self::ContinuousHi,
            x if x == CrDriveMode_CrDrive_Continuous_Hi_Plus => Self::ContinuousHiPlus,
            x if x == CrDriveMode_CrDrive_Continuous_Hi_Live => Self::ContinuousHiLive,
            x if x == CrDriveMode_CrDrive_Continuous_Lo => Self::ContinuousLo,
            x if x == CrDriveMode_CrDrive_Continuous => Self::Continuous,
            x if x == CrDriveMode_CrDrive_Continuous_SpeedPriority => Self::ContinuousSpeedPriority,
            x if x == CrDriveMode_CrDrive_Continuous_Mid => Self::ContinuousMid,
            x if x == CrDriveMode_CrDrive_Continuous_Mid_Live => Self::ContinuousMidLive,
            x if x == CrDriveMode_CrDrive_Continuous_Lo_Live => Self::ContinuousLoLive,
            x if x == CrDriveMode_CrDrive_SingleBurstShooting_lo => Self::SingleBurstShootingLo,
            x if x == CrDriveMode_CrDrive_SingleBurstShooting_mid => Self::SingleBurstShootingMid,
            x if x == CrDriveMode_CrDrive_SingleBurstShooting_hi => Self::SingleBurstShootingHi,
            x if x == CrDriveMode_CrDrive_FocusBracket => Self::FocusBracket,
            x if x == CrDriveMode_CrDrive_Timelapse => Self::Timelapse,
            x if x == CrDriveMode_CrDrive_Timer_2s => Self::Timer2s,
            x if x == CrDriveMode_CrDrive_Timer_5s => Self::Timer5s,
            x if x == CrDriveMode_CrDrive_Timer_10s => Self::Timer10s,
            x if x == CrDriveMode_CrDrive_Continuous_Bracket_03Ev_3pics => {
                Self::ContinuousBracket03Ev3Pics
            }
            x if x == CrDriveMode_CrDrive_Continuous_Bracket_03Ev_5pics => {
                Self::ContinuousBracket03Ev5Pics
            }
            x if x == CrDriveMode_CrDrive_Continuous_Bracket_03Ev_9pics => {
                Self::ContinuousBracket03Ev9Pics
            }
            x if x == CrDriveMode_CrDrive_Continuous_Bracket_05Ev_3pics => {
                Self::ContinuousBracket05Ev3Pics
            }
            x if x == CrDriveMode_CrDrive_Continuous_Bracket_05Ev_5pics => {
                Self::ContinuousBracket05Ev5Pics
            }
            x if x == CrDriveMode_CrDrive_Continuous_Bracket_05Ev_9pics => {
                Self::ContinuousBracket05Ev9Pics
            }
            x if x == CrDriveMode_CrDrive_Continuous_Bracket_07Ev_3pics => {
                Self::ContinuousBracket07Ev3Pics
            }
            x if x == CrDriveMode_CrDrive_Continuous_Bracket_07Ev_5pics => {
                Self::ContinuousBracket07Ev5Pics
            }
            x if x == CrDriveMode_CrDrive_Continuous_Bracket_07Ev_9pics => {
                Self::ContinuousBracket07Ev9Pics
            }
            x if x == CrDriveMode_CrDrive_Continuous_Bracket_10Ev_3pics => {
                Self::ContinuousBracket10Ev3Pics
            }
            x if x == CrDriveMode_CrDrive_Continuous_Bracket_10Ev_5pics => {
                Self::ContinuousBracket10Ev5Pics
            }
            x if x == CrDriveMode_CrDrive_Continuous_Bracket_10Ev_9pics => {
                Self::ContinuousBracket10Ev9Pics
            }
            x if x == CrDriveMode_CrDrive_Continuous_Bracket_20Ev_3pics => {
                Self::ContinuousBracket20Ev3Pics
            }
            x if x == CrDriveMode_CrDrive_Continuous_Bracket_20Ev_5pics => {
                Self::ContinuousBracket20Ev5Pics
            }
            x if x == CrDriveMode_CrDrive_Continuous_Bracket_30Ev_3pics => {
                Self::ContinuousBracket30Ev3Pics
            }
            x if x == CrDriveMode_CrDrive_Continuous_Bracket_30Ev_5pics => {
                Self::ContinuousBracket30Ev5Pics
            }
            x if x == CrDriveMode_CrDrive_Continuous_Bracket_03Ev_2pics_Plus => {
                Self::ContinuousBracket03Ev2PicsPlus
            }
            x if x == CrDriveMode_CrDrive_Continuous_Bracket_03Ev_2pics_Minus => {
                Self::ContinuousBracket03Ev2PicsMinus
            }
            x if x == CrDriveMode_CrDrive_Continuous_Bracket_03Ev_7pics => {
                Self::ContinuousBracket03Ev7Pics
            }
            x if x == CrDriveMode_CrDrive_Continuous_Bracket_05Ev_2pics_Plus => {
                Self::ContinuousBracket05Ev2PicsPlus
            }
            x if x == CrDriveMode_CrDrive_Continuous_Bracket_05Ev_2pics_Minus => {
                Self::ContinuousBracket05Ev2PicsMinus
            }
            x if x == CrDriveMode_CrDrive_Continuous_Bracket_05Ev_7pics => {
                Self::ContinuousBracket05Ev7Pics
            }
            x if x == CrDriveMode_CrDrive_Continuous_Bracket_07Ev_2pics_Plus => {
                Self::ContinuousBracket07Ev2PicsPlus
            }
            x if x == CrDriveMode_CrDrive_Continuous_Bracket_07Ev_2pics_Minus => {
                Self::ContinuousBracket07Ev2PicsMinus
            }
            x if x == CrDriveMode_CrDrive_Continuous_Bracket_07Ev_7pics => {
                Self::ContinuousBracket07Ev7Pics
            }
            x if x == CrDriveMode_CrDrive_Continuous_Bracket_10Ev_2pics_Plus => {
                Self::ContinuousBracket10Ev2PicsPlus
            }
            x if x == CrDriveMode_CrDrive_Continuous_Bracket_10Ev_2pics_Minus => {
                Self::ContinuousBracket10Ev2PicsMinus
            }
            x if x == CrDriveMode_CrDrive_Continuous_Bracket_10Ev_7pics => {
                Self::ContinuousBracket10Ev7Pics
            }
            x if x == CrDriveMode_CrDrive_Continuous_Bracket_13Ev_2pics_Plus => {
                Self::ContinuousBracket13Ev2PicsPlus
            }
            x if x == CrDriveMode_CrDrive_Continuous_Bracket_13Ev_2pics_Minus => {
                Self::ContinuousBracket13Ev2PicsMinus
            }
            x if x == CrDriveMode_CrDrive_Continuous_Bracket_13Ev_3pics => {
                Self::ContinuousBracket13Ev3Pics
            }
            x if x == CrDriveMode_CrDrive_Continuous_Bracket_13Ev_5pics => {
                Self::ContinuousBracket13Ev5Pics
            }
            x if x == CrDriveMode_CrDrive_Continuous_Bracket_13Ev_7pics => {
                Self::ContinuousBracket13Ev7Pics
            }
            x if x == CrDriveMode_CrDrive_Continuous_Bracket_15Ev_2pics_Plus => {
                Self::ContinuousBracket15Ev2PicsPlus
            }
            x if x == CrDriveMode_CrDrive_Continuous_Bracket_15Ev_2pics_Minus => {
                Self::ContinuousBracket15Ev2PicsMinus
            }
            x if x == CrDriveMode_CrDrive_Continuous_Bracket_15Ev_3pics => {
                Self::ContinuousBracket15Ev3Pics
            }
            x if x == CrDriveMode_CrDrive_Continuous_Bracket_15Ev_5pics => {
                Self::ContinuousBracket15Ev5Pics
            }
            x if x == CrDriveMode_CrDrive_Continuous_Bracket_15Ev_7pics => {
                Self::ContinuousBracket15Ev7Pics
            }
            x if x == CrDriveMode_CrDrive_Continuous_Bracket_17Ev_2pics_Plus => {
                Self::ContinuousBracket17Ev2PicsPlus
            }
            x if x == CrDriveMode_CrDrive_Continuous_Bracket_17Ev_2pics_Minus => {
                Self::ContinuousBracket17Ev2PicsMinus
            }
            x if x == CrDriveMode_CrDrive_Continuous_Bracket_17Ev_3pics => {
                Self::ContinuousBracket17Ev3Pics
            }
            x if x == CrDriveMode_CrDrive_Continuous_Bracket_17Ev_5pics => {
                Self::ContinuousBracket17Ev5Pics
            }
            x if x == CrDriveMode_CrDrive_Continuous_Bracket_17Ev_7pics => {
                Self::ContinuousBracket17Ev7Pics
            }
            x if x == CrDriveMode_CrDrive_Continuous_Bracket_20Ev_2pics_Plus => {
                Self::ContinuousBracket20Ev2PicsPlus
            }
            x if x == CrDriveMode_CrDrive_Continuous_Bracket_20Ev_2pics_Minus => {
                Self::ContinuousBracket20Ev2PicsMinus
            }
            x if x == CrDriveMode_CrDrive_Continuous_Bracket_20Ev_7pics => {
                Self::ContinuousBracket20Ev7Pics
            }
            x if x == CrDriveMode_CrDrive_Continuous_Bracket_23Ev_2pics_Plus => {
                Self::ContinuousBracket23Ev2PicsPlus
            }
            x if x == CrDriveMode_CrDrive_Continuous_Bracket_23Ev_2pics_Minus => {
                Self::ContinuousBracket23Ev2PicsMinus
            }
            x if x == CrDriveMode_CrDrive_Continuous_Bracket_23Ev_3pics => {
                Self::ContinuousBracket23Ev3Pics
            }
            x if x == CrDriveMode_CrDrive_Continuous_Bracket_23Ev_5pics => {
                Self::ContinuousBracket23Ev5Pics
            }
            x if x == CrDriveMode_CrDrive_Continuous_Bracket_25Ev_2pics_Plus => {
                Self::ContinuousBracket25Ev2PicsPlus
            }
            x if x == CrDriveMode_CrDrive_Continuous_Bracket_25Ev_2pics_Minus => {
                Self::ContinuousBracket25Ev2PicsMinus
            }
            x if x == CrDriveMode_CrDrive_Continuous_Bracket_25Ev_3pics => {
                Self::ContinuousBracket25Ev3Pics
            }
            x if x == CrDriveMode_CrDrive_Continuous_Bracket_25Ev_5pics => {
                Self::ContinuousBracket25Ev5Pics
            }
            x if x == CrDriveMode_CrDrive_Continuous_Bracket_27Ev_2pics_Plus => {
                Self::ContinuousBracket27Ev2PicsPlus
            }
            x if x == CrDriveMode_CrDrive_Continuous_Bracket_27Ev_2pics_Minus => {
                Self::ContinuousBracket27Ev2PicsMinus
            }
            x if x == CrDriveMode_CrDrive_Continuous_Bracket_27Ev_3pics => {
                Self::ContinuousBracket27Ev3Pics
            }
            x if x == CrDriveMode_CrDrive_Continuous_Bracket_27Ev_5pics => {
                Self::ContinuousBracket27Ev5Pics
            }
            x if x == CrDriveMode_CrDrive_Continuous_Bracket_30Ev_2pics_Plus => {
                Self::ContinuousBracket30Ev2PicsPlus
            }
            x if x == CrDriveMode_CrDrive_Continuous_Bracket_30Ev_2pics_Minus => {
                Self::ContinuousBracket30Ev2PicsMinus
            }
            x if x == CrDriveMode_CrDrive_Single_Bracket_03Ev_3pics => Self::SingleBracket03Ev3Pics,
            x if x == CrDriveMode_CrDrive_Single_Bracket_03Ev_5pics => Self::SingleBracket03Ev5Pics,
            x if x == CrDriveMode_CrDrive_Single_Bracket_03Ev_9pics => Self::SingleBracket03Ev9Pics,
            x if x == CrDriveMode_CrDrive_Single_Bracket_05Ev_3pics => Self::SingleBracket05Ev3Pics,
            x if x == CrDriveMode_CrDrive_Single_Bracket_05Ev_5pics => Self::SingleBracket05Ev5Pics,
            x if x == CrDriveMode_CrDrive_Single_Bracket_05Ev_9pics => Self::SingleBracket05Ev9Pics,
            x if x == CrDriveMode_CrDrive_Single_Bracket_07Ev_3pics => Self::SingleBracket07Ev3Pics,
            x if x == CrDriveMode_CrDrive_Single_Bracket_07Ev_5pics => Self::SingleBracket07Ev5Pics,
            x if x == CrDriveMode_CrDrive_Single_Bracket_07Ev_9pics => Self::SingleBracket07Ev9Pics,
            x if x == CrDriveMode_CrDrive_Single_Bracket_10Ev_3pics => Self::SingleBracket10Ev3Pics,
            x if x == CrDriveMode_CrDrive_Single_Bracket_10Ev_5pics => Self::SingleBracket10Ev5Pics,
            x if x == CrDriveMode_CrDrive_Single_Bracket_10Ev_9pics => Self::SingleBracket10Ev9Pics,
            x if x == CrDriveMode_CrDrive_Single_Bracket_20Ev_3pics => Self::SingleBracket20Ev3Pics,
            x if x == CrDriveMode_CrDrive_Single_Bracket_20Ev_5pics => Self::SingleBracket20Ev5Pics,
            x if x == CrDriveMode_CrDrive_Single_Bracket_30Ev_3pics => Self::SingleBracket30Ev3Pics,
            x if x == CrDriveMode_CrDrive_Single_Bracket_30Ev_5pics => Self::SingleBracket30Ev5Pics,
            x if x == CrDriveMode_CrDrive_Single_Bracket_03Ev_2pics_Plus => {
                Self::SingleBracket03Ev2PicsPlus
            }
            x if x == CrDriveMode_CrDrive_Single_Bracket_03Ev_2pics_Minus => {
                Self::SingleBracket03Ev2PicsMinus
            }
            x if x == CrDriveMode_CrDrive_Single_Bracket_03Ev_7pics => Self::SingleBracket03Ev7Pics,
            x if x == CrDriveMode_CrDrive_Single_Bracket_05Ev_2pics_Plus => {
                Self::SingleBracket05Ev2PicsPlus
            }
            x if x == CrDriveMode_CrDrive_Single_Bracket_05Ev_2pics_Minus => {
                Self::SingleBracket05Ev2PicsMinus
            }
            x if x == CrDriveMode_CrDrive_Single_Bracket_05Ev_7pics => Self::SingleBracket05Ev7Pics,
            x if x == CrDriveMode_CrDrive_Single_Bracket_07Ev_2pics_Plus => {
                Self::SingleBracket07Ev2PicsPlus
            }
            x if x == CrDriveMode_CrDrive_Single_Bracket_07Ev_2pics_Minus => {
                Self::SingleBracket07Ev2PicsMinus
            }
            x if x == CrDriveMode_CrDrive_Single_Bracket_07Ev_7pics => Self::SingleBracket07Ev7Pics,
            x if x == CrDriveMode_CrDrive_Single_Bracket_10Ev_2pics_Plus => {
                Self::SingleBracket10Ev2PicsPlus
            }
            x if x == CrDriveMode_CrDrive_Single_Bracket_10Ev_2pics_Minus => {
                Self::SingleBracket10Ev2PicsMinus
            }
            x if x == CrDriveMode_CrDrive_Single_Bracket_10Ev_7pics => Self::SingleBracket10Ev7Pics,
            x if x == CrDriveMode_CrDrive_Single_Bracket_13Ev_2pics_Plus => {
                Self::SingleBracket13Ev2PicsPlus
            }
            x if x == CrDriveMode_CrDrive_Single_Bracket_13Ev_2pics_Minus => {
                Self::SingleBracket13Ev2PicsMinus
            }
            x if x == CrDriveMode_CrDrive_Single_Bracket_13Ev_3pics => Self::SingleBracket13Ev3Pics,
            x if x == CrDriveMode_CrDrive_Single_Bracket_13Ev_5pics => Self::SingleBracket13Ev5Pics,
            x if x == CrDriveMode_CrDrive_Single_Bracket_13Ev_7pics => Self::SingleBracket13Ev7Pics,
            x if x == CrDriveMode_CrDrive_Single_Bracket_15Ev_2pics_Plus => {
                Self::SingleBracket15Ev2PicsPlus
            }
            x if x == CrDriveMode_CrDrive_Single_Bracket_15Ev_2pics_Minus => {
                Self::SingleBracket15Ev2PicsMinus
            }
            x if x == CrDriveMode_CrDrive_Single_Bracket_15Ev_3pics => Self::SingleBracket15Ev3Pics,
            x if x == CrDriveMode_CrDrive_Single_Bracket_15Ev_5pics => Self::SingleBracket15Ev5Pics,
            x if x == CrDriveMode_CrDrive_Single_Bracket_15Ev_7pics => Self::SingleBracket15Ev7Pics,
            x if x == CrDriveMode_CrDrive_Single_Bracket_17Ev_2pics_Plus => {
                Self::SingleBracket17Ev2PicsPlus
            }
            x if x == CrDriveMode_CrDrive_Single_Bracket_17Ev_2pics_Minus => {
                Self::SingleBracket17Ev2PicsMinus
            }
            x if x == CrDriveMode_CrDrive_Single_Bracket_17Ev_3pics => Self::SingleBracket17Ev3Pics,
            x if x == CrDriveMode_CrDrive_Single_Bracket_17Ev_5pics => Self::SingleBracket17Ev5Pics,
            x if x == CrDriveMode_CrDrive_Single_Bracket_17Ev_7pics => Self::SingleBracket17Ev7Pics,
            x if x == CrDriveMode_CrDrive_Single_Bracket_20Ev_2pics_Plus => {
                Self::SingleBracket20Ev2PicsPlus
            }
            x if x == CrDriveMode_CrDrive_Single_Bracket_20Ev_2pics_Minus => {
                Self::SingleBracket20Ev2PicsMinus
            }
            x if x == CrDriveMode_CrDrive_Single_Bracket_20Ev_7pics => Self::SingleBracket20Ev7Pics,
            x if x == CrDriveMode_CrDrive_Single_Bracket_23Ev_2pics_Plus => {
                Self::SingleBracket23Ev2PicsPlus
            }
            x if x == CrDriveMode_CrDrive_Single_Bracket_23Ev_2pics_Minus => {
                Self::SingleBracket23Ev2PicsMinus
            }
            x if x == CrDriveMode_CrDrive_Single_Bracket_23Ev_3pics => Self::SingleBracket23Ev3Pics,
            x if x == CrDriveMode_CrDrive_Single_Bracket_23Ev_5pics => Self::SingleBracket23Ev5Pics,
            x if x == CrDriveMode_CrDrive_Single_Bracket_25Ev_2pics_Plus => {
                Self::SingleBracket25Ev2PicsPlus
            }
            x if x == CrDriveMode_CrDrive_Single_Bracket_25Ev_2pics_Minus => {
                Self::SingleBracket25Ev2PicsMinus
            }
            x if x == CrDriveMode_CrDrive_Single_Bracket_25Ev_3pics => Self::SingleBracket25Ev3Pics,
            x if x == CrDriveMode_CrDrive_Single_Bracket_25Ev_5pics => Self::SingleBracket25Ev5Pics,
            x if x == CrDriveMode_CrDrive_Single_Bracket_27Ev_2pics_Plus => {
                Self::SingleBracket27Ev2PicsPlus
            }
            x if x == CrDriveMode_CrDrive_Single_Bracket_27Ev_2pics_Minus => {
                Self::SingleBracket27Ev2PicsMinus
            }
            x if x == CrDriveMode_CrDrive_Single_Bracket_27Ev_3pics => Self::SingleBracket27Ev3Pics,
            x if x == CrDriveMode_CrDrive_Single_Bracket_27Ev_5pics => Self::SingleBracket27Ev5Pics,
            x if x == CrDriveMode_CrDrive_Single_Bracket_30Ev_2pics_Plus => {
                Self::SingleBracket30Ev2PicsPlus
            }
            x if x == CrDriveMode_CrDrive_Single_Bracket_30Ev_2pics_Minus => {
                Self::SingleBracket30Ev2PicsMinus
            }
            x if x == CrDriveMode_CrDrive_WB_Bracket_Lo => Self::WbBracketLo,
            x if x == CrDriveMode_CrDrive_WB_Bracket_Hi => Self::WbBracketHi,
            x if x == CrDriveMode_CrDrive_DRO_Bracket_Lo => Self::DroBracketLo,
            x if x == CrDriveMode_CrDrive_DRO_Bracket_Hi => Self::DroBracketHi,
            x if x == CrDriveMode_CrDrive_Continuous_Timer_3pics => Self::ContinuousTimer3Pics,
            x if x == CrDriveMode_CrDrive_Continuous_Timer_5pics => Self::ContinuousTimer5Pics,
            x if x == CrDriveMode_CrDrive_Continuous_Timer_2s_3pics => Self::ContinuousTimer2s3Pics,
            x if x == CrDriveMode_CrDrive_Continuous_Timer_2s_5pics => Self::ContinuousTimer2s5Pics,
            x if x == CrDriveMode_CrDrive_Continuous_Timer_5s_3pics => Self::ContinuousTimer5s3Pics,
            x if x == CrDriveMode_CrDrive_Continuous_Timer_5s_5pics => Self::ContinuousTimer5s5Pics,
            x if x == CrDriveMode_CrDrive_LPF_Bracket => Self::LpfBracket,
            x if x == CrDriveMode_CrDrive_RemoteCommander => Self::RemoteCommander,
            x if x == CrDriveMode_CrDrive_MirrorUp => Self::MirrorUp,
            x if x == CrDriveMode_CrDrive_SelfPortrait_1 => Self::SelfPortrait1,
            x if x == CrDriveMode_CrDrive_SelfPortrait_2 => Self::SelfPortrait2,
            _ => return None,
        })
    }
}

impl ToCrsdk<u64> for DriveMode {
    fn to_crsdk(&self) -> u64 {
        *self as u64
    }
}

/// Shutter type for interval recording (timelapse)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum IntervalRecShutterType {
    /// Automatic shutter type selection based on shooting conditions
    Auto = 1,
    /// Mechanical shutter for interval recording
    Mechanical = 2,
    /// Electronic shutter for silent interval recording
    Electronic = 3,
}

impl ToCrsdk<u64> for IntervalRecShutterType {
    fn to_crsdk(&self) -> u64 {
        *self as u64
    }
}

impl PropertyValue for IntervalRecShutterType {
    fn from_raw(raw: u64) -> Option<Self> {
        Some(match raw as u8 {
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

impl std::fmt::Display for DriveMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Single => "Single",
            Self::ContinuousHi => "Cont. Hi",
            Self::ContinuousHiPlus => "Cont. Hi+",
            Self::ContinuousHiLive => "Cont. Hi Live",
            Self::ContinuousLo => "Cont. Lo",
            Self::Continuous => "Continuous",
            Self::ContinuousSpeedPriority => "Cont. Speed",
            Self::ContinuousMid => "Cont. Mid",
            Self::ContinuousMidLive => "Cont. Mid Live",
            Self::ContinuousLoLive => "Cont. Lo Live",
            Self::SingleBurstShootingLo => "Burst Lo",
            Self::SingleBurstShootingMid => "Burst Mid",
            Self::SingleBurstShootingHi => "Burst Hi",
            Self::FocusBracket => "Focus Bracket",
            Self::Timelapse => "Timelapse",
            Self::Timer2s => "Timer 2s",
            Self::Timer5s => "Timer 5s",
            Self::Timer10s => "Timer 10s",
            Self::ContinuousBracket03Ev3Pics => "C.BRK 0.3EV 3",
            Self::ContinuousBracket03Ev5Pics => "C.BRK 0.3EV 5",
            Self::ContinuousBracket03Ev9Pics => "C.BRK 0.3EV 9",
            Self::ContinuousBracket03Ev2PicsPlus => "C.BRK 0.3EV 2+",
            Self::ContinuousBracket03Ev2PicsMinus => "C.BRK 0.3EV 2-",
            Self::ContinuousBracket03Ev7Pics => "C.BRK 0.3EV 7",
            Self::ContinuousBracket05Ev3Pics => "C.BRK 0.5EV 3",
            Self::ContinuousBracket05Ev5Pics => "C.BRK 0.5EV 5",
            Self::ContinuousBracket05Ev9Pics => "C.BRK 0.5EV 9",
            Self::ContinuousBracket05Ev2PicsPlus => "C.BRK 0.5EV 2+",
            Self::ContinuousBracket05Ev2PicsMinus => "C.BRK 0.5EV 2-",
            Self::ContinuousBracket05Ev7Pics => "C.BRK 0.5EV 7",
            Self::ContinuousBracket07Ev3Pics => "C.BRK 0.7EV 3",
            Self::ContinuousBracket07Ev5Pics => "C.BRK 0.7EV 5",
            Self::ContinuousBracket07Ev9Pics => "C.BRK 0.7EV 9",
            Self::ContinuousBracket07Ev2PicsPlus => "C.BRK 0.7EV 2+",
            Self::ContinuousBracket07Ev2PicsMinus => "C.BRK 0.7EV 2-",
            Self::ContinuousBracket07Ev7Pics => "C.BRK 0.7EV 7",
            Self::ContinuousBracket10Ev3Pics => "C.BRK 1.0EV 3",
            Self::ContinuousBracket10Ev5Pics => "C.BRK 1.0EV 5",
            Self::ContinuousBracket10Ev9Pics => "C.BRK 1.0EV 9",
            Self::ContinuousBracket10Ev2PicsPlus => "C.BRK 1.0EV 2+",
            Self::ContinuousBracket10Ev2PicsMinus => "C.BRK 1.0EV 2-",
            Self::ContinuousBracket10Ev7Pics => "C.BRK 1.0EV 7",
            Self::ContinuousBracket13Ev2PicsPlus => "C.BRK 1.3EV 2+",
            Self::ContinuousBracket13Ev2PicsMinus => "C.BRK 1.3EV 2-",
            Self::ContinuousBracket13Ev3Pics => "C.BRK 1.3EV 3",
            Self::ContinuousBracket13Ev5Pics => "C.BRK 1.3EV 5",
            Self::ContinuousBracket13Ev7Pics => "C.BRK 1.3EV 7",
            Self::ContinuousBracket15Ev2PicsPlus => "C.BRK 1.5EV 2+",
            Self::ContinuousBracket15Ev2PicsMinus => "C.BRK 1.5EV 2-",
            Self::ContinuousBracket15Ev3Pics => "C.BRK 1.5EV 3",
            Self::ContinuousBracket15Ev5Pics => "C.BRK 1.5EV 5",
            Self::ContinuousBracket15Ev7Pics => "C.BRK 1.5EV 7",
            Self::ContinuousBracket17Ev2PicsPlus => "C.BRK 1.7EV 2+",
            Self::ContinuousBracket17Ev2PicsMinus => "C.BRK 1.7EV 2-",
            Self::ContinuousBracket17Ev3Pics => "C.BRK 1.7EV 3",
            Self::ContinuousBracket17Ev5Pics => "C.BRK 1.7EV 5",
            Self::ContinuousBracket17Ev7Pics => "C.BRK 1.7EV 7",
            Self::ContinuousBracket20Ev3Pics => "C.BRK 2.0EV 3",
            Self::ContinuousBracket20Ev5Pics => "C.BRK 2.0EV 5",
            Self::ContinuousBracket20Ev2PicsPlus => "C.BRK 2.0EV 2+",
            Self::ContinuousBracket20Ev2PicsMinus => "C.BRK 2.0EV 2-",
            Self::ContinuousBracket20Ev7Pics => "C.BRK 2.0EV 7",
            Self::ContinuousBracket23Ev2PicsPlus => "C.BRK 2.3EV 2+",
            Self::ContinuousBracket23Ev2PicsMinus => "C.BRK 2.3EV 2-",
            Self::ContinuousBracket23Ev3Pics => "C.BRK 2.3EV 3",
            Self::ContinuousBracket23Ev5Pics => "C.BRK 2.3EV 5",
            Self::ContinuousBracket25Ev2PicsPlus => "C.BRK 2.5EV 2+",
            Self::ContinuousBracket25Ev2PicsMinus => "C.BRK 2.5EV 2-",
            Self::ContinuousBracket25Ev3Pics => "C.BRK 2.5EV 3",
            Self::ContinuousBracket25Ev5Pics => "C.BRK 2.5EV 5",
            Self::ContinuousBracket27Ev2PicsPlus => "C.BRK 2.7EV 2+",
            Self::ContinuousBracket27Ev2PicsMinus => "C.BRK 2.7EV 2-",
            Self::ContinuousBracket27Ev3Pics => "C.BRK 2.7EV 3",
            Self::ContinuousBracket27Ev5Pics => "C.BRK 2.7EV 5",
            Self::ContinuousBracket30Ev3Pics => "C.BRK 3.0EV 3",
            Self::ContinuousBracket30Ev5Pics => "C.BRK 3.0EV 5",
            Self::ContinuousBracket30Ev2PicsPlus => "C.BRK 3.0EV 2+",
            Self::ContinuousBracket30Ev2PicsMinus => "C.BRK 3.0EV 2-",
            Self::SingleBracket03Ev3Pics => "S.BRK 0.3EV 3",
            Self::SingleBracket03Ev5Pics => "S.BRK 0.3EV 5",
            Self::SingleBracket03Ev9Pics => "S.BRK 0.3EV 9",
            Self::SingleBracket03Ev2PicsPlus => "S.BRK 0.3EV 2+",
            Self::SingleBracket03Ev2PicsMinus => "S.BRK 0.3EV 2-",
            Self::SingleBracket03Ev7Pics => "S.BRK 0.3EV 7",
            Self::SingleBracket05Ev3Pics => "S.BRK 0.5EV 3",
            Self::SingleBracket05Ev5Pics => "S.BRK 0.5EV 5",
            Self::SingleBracket05Ev9Pics => "S.BRK 0.5EV 9",
            Self::SingleBracket05Ev2PicsPlus => "S.BRK 0.5EV 2+",
            Self::SingleBracket05Ev2PicsMinus => "S.BRK 0.5EV 2-",
            Self::SingleBracket05Ev7Pics => "S.BRK 0.5EV 7",
            Self::SingleBracket07Ev3Pics => "S.BRK 0.7EV 3",
            Self::SingleBracket07Ev5Pics => "S.BRK 0.7EV 5",
            Self::SingleBracket07Ev9Pics => "S.BRK 0.7EV 9",
            Self::SingleBracket07Ev2PicsPlus => "S.BRK 0.7EV 2+",
            Self::SingleBracket07Ev2PicsMinus => "S.BRK 0.7EV 2-",
            Self::SingleBracket07Ev7Pics => "S.BRK 0.7EV 7",
            Self::SingleBracket10Ev3Pics => "S.BRK 1.0EV 3",
            Self::SingleBracket10Ev5Pics => "S.BRK 1.0EV 5",
            Self::SingleBracket10Ev9Pics => "S.BRK 1.0EV 9",
            Self::SingleBracket10Ev2PicsPlus => "S.BRK 1.0EV 2+",
            Self::SingleBracket10Ev2PicsMinus => "S.BRK 1.0EV 2-",
            Self::SingleBracket10Ev7Pics => "S.BRK 1.0EV 7",
            Self::SingleBracket13Ev2PicsPlus => "S.BRK 1.3EV 2+",
            Self::SingleBracket13Ev2PicsMinus => "S.BRK 1.3EV 2-",
            Self::SingleBracket13Ev3Pics => "S.BRK 1.3EV 3",
            Self::SingleBracket13Ev5Pics => "S.BRK 1.3EV 5",
            Self::SingleBracket13Ev7Pics => "S.BRK 1.3EV 7",
            Self::SingleBracket15Ev2PicsPlus => "S.BRK 1.5EV 2+",
            Self::SingleBracket15Ev2PicsMinus => "S.BRK 1.5EV 2-",
            Self::SingleBracket15Ev3Pics => "S.BRK 1.5EV 3",
            Self::SingleBracket15Ev5Pics => "S.BRK 1.5EV 5",
            Self::SingleBracket15Ev7Pics => "S.BRK 1.5EV 7",
            Self::SingleBracket17Ev2PicsPlus => "S.BRK 1.7EV 2+",
            Self::SingleBracket17Ev2PicsMinus => "S.BRK 1.7EV 2-",
            Self::SingleBracket17Ev3Pics => "S.BRK 1.7EV 3",
            Self::SingleBracket17Ev5Pics => "S.BRK 1.7EV 5",
            Self::SingleBracket17Ev7Pics => "S.BRK 1.7EV 7",
            Self::SingleBracket20Ev3Pics => "S.BRK 2.0EV 3",
            Self::SingleBracket20Ev5Pics => "S.BRK 2.0EV 5",
            Self::SingleBracket20Ev2PicsPlus => "S.BRK 2.0EV 2+",
            Self::SingleBracket20Ev2PicsMinus => "S.BRK 2.0EV 2-",
            Self::SingleBracket20Ev7Pics => "S.BRK 2.0EV 7",
            Self::SingleBracket23Ev2PicsPlus => "S.BRK 2.3EV 2+",
            Self::SingleBracket23Ev2PicsMinus => "S.BRK 2.3EV 2-",
            Self::SingleBracket23Ev3Pics => "S.BRK 2.3EV 3",
            Self::SingleBracket23Ev5Pics => "S.BRK 2.3EV 5",
            Self::SingleBracket25Ev2PicsPlus => "S.BRK 2.5EV 2+",
            Self::SingleBracket25Ev2PicsMinus => "S.BRK 2.5EV 2-",
            Self::SingleBracket25Ev3Pics => "S.BRK 2.5EV 3",
            Self::SingleBracket25Ev5Pics => "S.BRK 2.5EV 5",
            Self::SingleBracket27Ev2PicsPlus => "S.BRK 2.7EV 2+",
            Self::SingleBracket27Ev2PicsMinus => "S.BRK 2.7EV 2-",
            Self::SingleBracket27Ev3Pics => "S.BRK 2.7EV 3",
            Self::SingleBracket27Ev5Pics => "S.BRK 2.7EV 5",
            Self::SingleBracket30Ev3Pics => "S.BRK 3.0EV 3",
            Self::SingleBracket30Ev5Pics => "S.BRK 3.0EV 5",
            Self::SingleBracket30Ev2PicsPlus => "S.BRK 3.0EV 2+",
            Self::SingleBracket30Ev2PicsMinus => "S.BRK 3.0EV 2-",
            Self::WbBracketLo => "WB BRK Lo",
            Self::WbBracketHi => "WB BRK Hi",
            Self::DroBracketLo => "DRO BRK Lo",
            Self::DroBracketHi => "DRO BRK Hi",
            Self::ContinuousTimer3Pics => "Cont. Timer 3",
            Self::ContinuousTimer5Pics => "Cont. Timer 5",
            Self::ContinuousTimer2s3Pics => "Timer 2s 3pics",
            Self::ContinuousTimer2s5Pics => "Timer 2s 5pics",
            Self::ContinuousTimer5s3Pics => "Timer 5s 3pics",
            Self::ContinuousTimer5s5Pics => "Timer 5s 5pics",
            Self::LpfBracket => "LPF Bracket",
            Self::RemoteCommander => "Remote",
            Self::MirrorUp => "Mirror Up",
            Self::SelfPortrait1 => "Self Portrait 1",
            Self::SelfPortrait2 => "Self Portrait 2",
        };
        write!(f, "{}", s)
    }
}
