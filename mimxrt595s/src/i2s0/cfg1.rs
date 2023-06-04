#[doc = "Register `CFG1` reader"]
pub struct R(crate::R<CFG1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFG1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFG1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFG1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFG1` writer"]
pub struct W(crate::W<CFG1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFG1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<CFG1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFG1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MAINENABLE` reader - Main Enable"]
pub type MAINENABLE_R = crate::BitReader<MAINENABLE_A>;
#[doc = "Main Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MAINENABLE_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<MAINENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: MAINENABLE_A) -> Self {
        variant as u8 != 0
    }
}
impl MAINENABLE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MAINENABLE_A {
        match self.bits {
            false => MAINENABLE_A::DISABLED,
            true => MAINENABLE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MAINENABLE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == MAINENABLE_A::ENABLED
    }
}
#[doc = "Field `MAINENABLE` writer - Main Enable"]
pub type MAINENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG1_SPEC, MAINENABLE_A, O>;
impl<'a, const O: u8> MAINENABLE_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MAINENABLE_A::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(MAINENABLE_A::ENABLED)
    }
}
#[doc = "Field `DATAPAUSE` reader - Data Flow Pause"]
pub type DATAPAUSE_R = crate::BitReader<DATAPAUSE_A>;
#[doc = "Data Flow Pause\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DATAPAUSE_A {
    #[doc = "0: Normal operation"]
    NORMAL = 0,
    #[doc = "1: Pause"]
    PAUSE = 1,
}
impl From<DATAPAUSE_A> for bool {
    #[inline(always)]
    fn from(variant: DATAPAUSE_A) -> Self {
        variant as u8 != 0
    }
}
impl DATAPAUSE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DATAPAUSE_A {
        match self.bits {
            false => DATAPAUSE_A::NORMAL,
            true => DATAPAUSE_A::PAUSE,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == DATAPAUSE_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `PAUSE`"]
    #[inline(always)]
    pub fn is_pause(&self) -> bool {
        *self == DATAPAUSE_A::PAUSE
    }
}
#[doc = "Field `DATAPAUSE` writer - Data Flow Pause"]
pub type DATAPAUSE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG1_SPEC, DATAPAUSE_A, O>;
impl<'a, const O: u8> DATAPAUSE_W<'a, O> {
    #[doc = "Normal operation"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(DATAPAUSE_A::NORMAL)
    }
    #[doc = "Pause"]
    #[inline(always)]
    pub fn pause(self) -> &'a mut W {
        self.variant(DATAPAUSE_A::PAUSE)
    }
}
#[doc = "Field `PAIRCOUNT` reader - Pair Count"]
pub type PAIRCOUNT_R = crate::FieldReader<u8, PAIRCOUNT_A>;
#[doc = "Pair Count\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PAIRCOUNT_A {
    #[doc = "0: One Pair"]
    PAIRS_1 = 0,
    #[doc = "1: Two Pairs"]
    PAIRS_2 = 1,
    #[doc = "2: Three Pairs"]
    PAIRS_3 = 2,
    #[doc = "3: Four Pairs"]
    PAIRS_4 = 3,
}
impl From<PAIRCOUNT_A> for u8 {
    #[inline(always)]
    fn from(variant: PAIRCOUNT_A) -> Self {
        variant as _
    }
}
impl PAIRCOUNT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAIRCOUNT_A {
        match self.bits {
            0 => PAIRCOUNT_A::PAIRS_1,
            1 => PAIRCOUNT_A::PAIRS_2,
            2 => PAIRCOUNT_A::PAIRS_3,
            3 => PAIRCOUNT_A::PAIRS_4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PAIRS_1`"]
    #[inline(always)]
    pub fn is_pairs_1(&self) -> bool {
        *self == PAIRCOUNT_A::PAIRS_1
    }
    #[doc = "Checks if the value of the field is `PAIRS_2`"]
    #[inline(always)]
    pub fn is_pairs_2(&self) -> bool {
        *self == PAIRCOUNT_A::PAIRS_2
    }
    #[doc = "Checks if the value of the field is `PAIRS_3`"]
    #[inline(always)]
    pub fn is_pairs_3(&self) -> bool {
        *self == PAIRCOUNT_A::PAIRS_3
    }
    #[doc = "Checks if the value of the field is `PAIRS_4`"]
    #[inline(always)]
    pub fn is_pairs_4(&self) -> bool {
        *self == PAIRCOUNT_A::PAIRS_4
    }
}
#[doc = "Field `PAIRCOUNT` writer - Pair Count"]
pub type PAIRCOUNT_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, CFG1_SPEC, u8, PAIRCOUNT_A, 2, O>;
impl<'a, const O: u8> PAIRCOUNT_W<'a, O> {
    #[doc = "One Pair"]
    #[inline(always)]
    pub fn pairs_1(self) -> &'a mut W {
        self.variant(PAIRCOUNT_A::PAIRS_1)
    }
    #[doc = "Two Pairs"]
    #[inline(always)]
    pub fn pairs_2(self) -> &'a mut W {
        self.variant(PAIRCOUNT_A::PAIRS_2)
    }
    #[doc = "Three Pairs"]
    #[inline(always)]
    pub fn pairs_3(self) -> &'a mut W {
        self.variant(PAIRCOUNT_A::PAIRS_3)
    }
    #[doc = "Four Pairs"]
    #[inline(always)]
    pub fn pairs_4(self) -> &'a mut W {
        self.variant(PAIRCOUNT_A::PAIRS_4)
    }
}
#[doc = "Field `MSTSLVCFG` reader - Master/Slave Configuration Selection"]
pub type MSTSLVCFG_R = crate::FieldReader<u8, MSTSLVCFG_A>;
#[doc = "Master/Slave Configuration Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MSTSLVCFG_A {
    #[doc = "0: Normal Slave Mode"]
    NORMAL_SLAVE_MODE = 0,
    #[doc = "1: WS Synchronized Master Mode"]
    WS_SYNC_MASTER = 1,
    #[doc = "2: Master Using an Existing SCK Mode"]
    MASTER_USING_SCK = 2,
    #[doc = "3: Normal Master Mode"]
    NORMAL_MASTER = 3,
}
impl From<MSTSLVCFG_A> for u8 {
    #[inline(always)]
    fn from(variant: MSTSLVCFG_A) -> Self {
        variant as _
    }
}
impl MSTSLVCFG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSTSLVCFG_A {
        match self.bits {
            0 => MSTSLVCFG_A::NORMAL_SLAVE_MODE,
            1 => MSTSLVCFG_A::WS_SYNC_MASTER,
            2 => MSTSLVCFG_A::MASTER_USING_SCK,
            3 => MSTSLVCFG_A::NORMAL_MASTER,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL_SLAVE_MODE`"]
    #[inline(always)]
    pub fn is_normal_slave_mode(&self) -> bool {
        *self == MSTSLVCFG_A::NORMAL_SLAVE_MODE
    }
    #[doc = "Checks if the value of the field is `WS_SYNC_MASTER`"]
    #[inline(always)]
    pub fn is_ws_sync_master(&self) -> bool {
        *self == MSTSLVCFG_A::WS_SYNC_MASTER
    }
    #[doc = "Checks if the value of the field is `MASTER_USING_SCK`"]
    #[inline(always)]
    pub fn is_master_using_sck(&self) -> bool {
        *self == MSTSLVCFG_A::MASTER_USING_SCK
    }
    #[doc = "Checks if the value of the field is `NORMAL_MASTER`"]
    #[inline(always)]
    pub fn is_normal_master(&self) -> bool {
        *self == MSTSLVCFG_A::NORMAL_MASTER
    }
}
#[doc = "Field `MSTSLVCFG` writer - Master/Slave Configuration Selection"]
pub type MSTSLVCFG_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, CFG1_SPEC, u8, MSTSLVCFG_A, 2, O>;
impl<'a, const O: u8> MSTSLVCFG_W<'a, O> {
    #[doc = "Normal Slave Mode"]
    #[inline(always)]
    pub fn normal_slave_mode(self) -> &'a mut W {
        self.variant(MSTSLVCFG_A::NORMAL_SLAVE_MODE)
    }
    #[doc = "WS Synchronized Master Mode"]
    #[inline(always)]
    pub fn ws_sync_master(self) -> &'a mut W {
        self.variant(MSTSLVCFG_A::WS_SYNC_MASTER)
    }
    #[doc = "Master Using an Existing SCK Mode"]
    #[inline(always)]
    pub fn master_using_sck(self) -> &'a mut W {
        self.variant(MSTSLVCFG_A::MASTER_USING_SCK)
    }
    #[doc = "Normal Master Mode"]
    #[inline(always)]
    pub fn normal_master(self) -> &'a mut W {
        self.variant(MSTSLVCFG_A::NORMAL_MASTER)
    }
}
#[doc = "Field `MODE` reader - Mode"]
pub type MODE_R = crate::FieldReader<u8, MODE_A>;
#[doc = "Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODE_A {
    #[doc = "0: Classic Mode"]
    CLASSIC_MODE = 0,
    #[doc = "1: DSP mode WS 50% duty cycle"]
    DSP_MODE_WS_50_DUTYCYCLE = 1,
    #[doc = "2: DSP mode WS 1 clock"]
    DSP_MODE_WS_1_CLOCK = 2,
    #[doc = "3: DSP mode WS 1 data"]
    DSP_MODE_WS_1_DATA = 3,
}
impl From<MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        variant as _
    }
}
impl MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODE_A {
        match self.bits {
            0 => MODE_A::CLASSIC_MODE,
            1 => MODE_A::DSP_MODE_WS_50_DUTYCYCLE,
            2 => MODE_A::DSP_MODE_WS_1_CLOCK,
            3 => MODE_A::DSP_MODE_WS_1_DATA,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CLASSIC_MODE`"]
    #[inline(always)]
    pub fn is_classic_mode(&self) -> bool {
        *self == MODE_A::CLASSIC_MODE
    }
    #[doc = "Checks if the value of the field is `DSP_MODE_WS_50_DUTYCYCLE`"]
    #[inline(always)]
    pub fn is_dsp_mode_ws_50_dutycycle(&self) -> bool {
        *self == MODE_A::DSP_MODE_WS_50_DUTYCYCLE
    }
    #[doc = "Checks if the value of the field is `DSP_MODE_WS_1_CLOCK`"]
    #[inline(always)]
    pub fn is_dsp_mode_ws_1_clock(&self) -> bool {
        *self == MODE_A::DSP_MODE_WS_1_CLOCK
    }
    #[doc = "Checks if the value of the field is `DSP_MODE_WS_1_DATA`"]
    #[inline(always)]
    pub fn is_dsp_mode_ws_1_data(&self) -> bool {
        *self == MODE_A::DSP_MODE_WS_1_DATA
    }
}
#[doc = "Field `MODE` writer - Mode"]
pub type MODE_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CFG1_SPEC, u8, MODE_A, 2, O>;
impl<'a, const O: u8> MODE_W<'a, O> {
    #[doc = "Classic Mode"]
    #[inline(always)]
    pub fn classic_mode(self) -> &'a mut W {
        self.variant(MODE_A::CLASSIC_MODE)
    }
    #[doc = "DSP mode WS 50% duty cycle"]
    #[inline(always)]
    pub fn dsp_mode_ws_50_dutycycle(self) -> &'a mut W {
        self.variant(MODE_A::DSP_MODE_WS_50_DUTYCYCLE)
    }
    #[doc = "DSP mode WS 1 clock"]
    #[inline(always)]
    pub fn dsp_mode_ws_1_clock(self) -> &'a mut W {
        self.variant(MODE_A::DSP_MODE_WS_1_CLOCK)
    }
    #[doc = "DSP mode WS 1 data"]
    #[inline(always)]
    pub fn dsp_mode_ws_1_data(self) -> &'a mut W {
        self.variant(MODE_A::DSP_MODE_WS_1_DATA)
    }
}
#[doc = "Field `RIGHTLOW` reader - Right Channel Low"]
pub type RIGHTLOW_R = crate::BitReader<RIGHTLOW_A>;
#[doc = "Right Channel Low\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RIGHTLOW_A {
    #[doc = "0: Right high"]
    RIGHT_HIGH = 0,
    #[doc = "1: Right low"]
    RIGHT_LOW = 1,
}
impl From<RIGHTLOW_A> for bool {
    #[inline(always)]
    fn from(variant: RIGHTLOW_A) -> Self {
        variant as u8 != 0
    }
}
impl RIGHTLOW_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RIGHTLOW_A {
        match self.bits {
            false => RIGHTLOW_A::RIGHT_HIGH,
            true => RIGHTLOW_A::RIGHT_LOW,
        }
    }
    #[doc = "Checks if the value of the field is `RIGHT_HIGH`"]
    #[inline(always)]
    pub fn is_right_high(&self) -> bool {
        *self == RIGHTLOW_A::RIGHT_HIGH
    }
    #[doc = "Checks if the value of the field is `RIGHT_LOW`"]
    #[inline(always)]
    pub fn is_right_low(&self) -> bool {
        *self == RIGHTLOW_A::RIGHT_LOW
    }
}
#[doc = "Field `RIGHTLOW` writer - Right Channel Low"]
pub type RIGHTLOW_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG1_SPEC, RIGHTLOW_A, O>;
impl<'a, const O: u8> RIGHTLOW_W<'a, O> {
    #[doc = "Right high"]
    #[inline(always)]
    pub fn right_high(self) -> &'a mut W {
        self.variant(RIGHTLOW_A::RIGHT_HIGH)
    }
    #[doc = "Right low"]
    #[inline(always)]
    pub fn right_low(self) -> &'a mut W {
        self.variant(RIGHTLOW_A::RIGHT_LOW)
    }
}
#[doc = "Field `LEFTJUST` reader - Left-Justify Data"]
pub type LEFTJUST_R = crate::BitReader<LEFTJUST_A>;
#[doc = "Left-Justify Data\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LEFTJUST_A {
    #[doc = "0: Right-justified"]
    RIGHT_JUSTIFIED = 0,
    #[doc = "1: Left-justified"]
    LEFT_JUSTIFIED = 1,
}
impl From<LEFTJUST_A> for bool {
    #[inline(always)]
    fn from(variant: LEFTJUST_A) -> Self {
        variant as u8 != 0
    }
}
impl LEFTJUST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LEFTJUST_A {
        match self.bits {
            false => LEFTJUST_A::RIGHT_JUSTIFIED,
            true => LEFTJUST_A::LEFT_JUSTIFIED,
        }
    }
    #[doc = "Checks if the value of the field is `RIGHT_JUSTIFIED`"]
    #[inline(always)]
    pub fn is_right_justified(&self) -> bool {
        *self == LEFTJUST_A::RIGHT_JUSTIFIED
    }
    #[doc = "Checks if the value of the field is `LEFT_JUSTIFIED`"]
    #[inline(always)]
    pub fn is_left_justified(&self) -> bool {
        *self == LEFTJUST_A::LEFT_JUSTIFIED
    }
}
#[doc = "Field `LEFTJUST` writer - Left-Justify Data"]
pub type LEFTJUST_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG1_SPEC, LEFTJUST_A, O>;
impl<'a, const O: u8> LEFTJUST_W<'a, O> {
    #[doc = "Right-justified"]
    #[inline(always)]
    pub fn right_justified(self) -> &'a mut W {
        self.variant(LEFTJUST_A::RIGHT_JUSTIFIED)
    }
    #[doc = "Left-justified"]
    #[inline(always)]
    pub fn left_justified(self) -> &'a mut W {
        self.variant(LEFTJUST_A::LEFT_JUSTIFIED)
    }
}
#[doc = "Field `ONECHANNEL` reader - Single Channel Mode"]
pub type ONECHANNEL_R = crate::BitReader<ONECHANNEL_A>;
#[doc = "Single Channel Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ONECHANNEL_A {
    #[doc = "0: Dual channel"]
    DUAL_CHANNEL = 0,
    #[doc = "1: Single channel"]
    SINGLE_CHANNEL = 1,
}
impl From<ONECHANNEL_A> for bool {
    #[inline(always)]
    fn from(variant: ONECHANNEL_A) -> Self {
        variant as u8 != 0
    }
}
impl ONECHANNEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ONECHANNEL_A {
        match self.bits {
            false => ONECHANNEL_A::DUAL_CHANNEL,
            true => ONECHANNEL_A::SINGLE_CHANNEL,
        }
    }
    #[doc = "Checks if the value of the field is `DUAL_CHANNEL`"]
    #[inline(always)]
    pub fn is_dual_channel(&self) -> bool {
        *self == ONECHANNEL_A::DUAL_CHANNEL
    }
    #[doc = "Checks if the value of the field is `SINGLE_CHANNEL`"]
    #[inline(always)]
    pub fn is_single_channel(&self) -> bool {
        *self == ONECHANNEL_A::SINGLE_CHANNEL
    }
}
#[doc = "Field `ONECHANNEL` writer - Single Channel Mode"]
pub type ONECHANNEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG1_SPEC, ONECHANNEL_A, O>;
impl<'a, const O: u8> ONECHANNEL_W<'a, O> {
    #[doc = "Dual channel"]
    #[inline(always)]
    pub fn dual_channel(self) -> &'a mut W {
        self.variant(ONECHANNEL_A::DUAL_CHANNEL)
    }
    #[doc = "Single channel"]
    #[inline(always)]
    pub fn single_channel(self) -> &'a mut W {
        self.variant(ONECHANNEL_A::SINGLE_CHANNEL)
    }
}
#[doc = "Field `PDMDATA` reader - PDM Data Selection"]
pub type PDMDATA_R = crate::BitReader<PDMDATA_A>;
#[doc = "PDM Data Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDMDATA_A {
    #[doc = "0: Normal Operation"]
    NORMAL = 0,
    #[doc = "1: DMIC subsystem"]
    DMIC_SUBSYSTEM = 1,
}
impl From<PDMDATA_A> for bool {
    #[inline(always)]
    fn from(variant: PDMDATA_A) -> Self {
        variant as u8 != 0
    }
}
impl PDMDATA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDMDATA_A {
        match self.bits {
            false => PDMDATA_A::NORMAL,
            true => PDMDATA_A::DMIC_SUBSYSTEM,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == PDMDATA_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `DMIC_SUBSYSTEM`"]
    #[inline(always)]
    pub fn is_dmic_subsystem(&self) -> bool {
        *self == PDMDATA_A::DMIC_SUBSYSTEM
    }
}
#[doc = "Field `PDMDATA` writer - PDM Data Selection"]
pub type PDMDATA_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG1_SPEC, PDMDATA_A, O>;
impl<'a, const O: u8> PDMDATA_W<'a, O> {
    #[doc = "Normal Operation"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(PDMDATA_A::NORMAL)
    }
    #[doc = "DMIC subsystem"]
    #[inline(always)]
    pub fn dmic_subsystem(self) -> &'a mut W {
        self.variant(PDMDATA_A::DMIC_SUBSYSTEM)
    }
}
#[doc = "Field `SCK_POL` reader - SCK Polarity"]
pub type SCK_POL_R = crate::BitReader<SCK_POL_A>;
#[doc = "SCK Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SCK_POL_A {
    #[doc = "0: Falling edge"]
    FALLING_EDGE = 0,
    #[doc = "1: Rising edge"]
    RISING_EDGE = 1,
}
impl From<SCK_POL_A> for bool {
    #[inline(always)]
    fn from(variant: SCK_POL_A) -> Self {
        variant as u8 != 0
    }
}
impl SCK_POL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SCK_POL_A {
        match self.bits {
            false => SCK_POL_A::FALLING_EDGE,
            true => SCK_POL_A::RISING_EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `FALLING_EDGE`"]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == SCK_POL_A::FALLING_EDGE
    }
    #[doc = "Checks if the value of the field is `RISING_EDGE`"]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == SCK_POL_A::RISING_EDGE
    }
}
#[doc = "Field `SCK_POL` writer - SCK Polarity"]
pub type SCK_POL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG1_SPEC, SCK_POL_A, O>;
impl<'a, const O: u8> SCK_POL_W<'a, O> {
    #[doc = "Falling edge"]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(SCK_POL_A::FALLING_EDGE)
    }
    #[doc = "Rising edge"]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(SCK_POL_A::RISING_EDGE)
    }
}
#[doc = "Field `WS_POL` reader - WS Polarity"]
pub type WS_POL_R = crate::BitReader<WS_POL_A>;
#[doc = "WS Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WS_POL_A {
    #[doc = "0: Not inverted"]
    NOT_INVERTED = 0,
    #[doc = "1: Inverted"]
    INVERTED = 1,
}
impl From<WS_POL_A> for bool {
    #[inline(always)]
    fn from(variant: WS_POL_A) -> Self {
        variant as u8 != 0
    }
}
impl WS_POL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WS_POL_A {
        match self.bits {
            false => WS_POL_A::NOT_INVERTED,
            true => WS_POL_A::INVERTED,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_INVERTED`"]
    #[inline(always)]
    pub fn is_not_inverted(&self) -> bool {
        *self == WS_POL_A::NOT_INVERTED
    }
    #[doc = "Checks if the value of the field is `INVERTED`"]
    #[inline(always)]
    pub fn is_inverted(&self) -> bool {
        *self == WS_POL_A::INVERTED
    }
}
#[doc = "Field `WS_POL` writer - WS Polarity"]
pub type WS_POL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG1_SPEC, WS_POL_A, O>;
impl<'a, const O: u8> WS_POL_W<'a, O> {
    #[doc = "Not inverted"]
    #[inline(always)]
    pub fn not_inverted(self) -> &'a mut W {
        self.variant(WS_POL_A::NOT_INVERTED)
    }
    #[doc = "Inverted"]
    #[inline(always)]
    pub fn inverted(self) -> &'a mut W {
        self.variant(WS_POL_A::INVERTED)
    }
}
#[doc = "Field `DATALEN` reader - Data Length"]
pub type DATALEN_R = crate::FieldReader<u8, DATALEN_A>;
#[doc = "Data Length\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DATALEN_A {
    #[doc = "3: Data is 4 bits in length."]
    DATA_4 = 3,
    #[doc = "4: Data is 5 bits in length."]
    DATA_5 = 4,
    #[doc = "7: Data is 8 bits in length."]
    DATA_8 = 7,
    #[doc = "30: Data is 31 bits in length."]
    DATA_31 = 30,
    #[doc = "31: Data is 32 bits in length."]
    DATA_32 = 31,
}
impl From<DATALEN_A> for u8 {
    #[inline(always)]
    fn from(variant: DATALEN_A) -> Self {
        variant as _
    }
}
impl DATALEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DATALEN_A> {
        match self.bits {
            3 => Some(DATALEN_A::DATA_4),
            4 => Some(DATALEN_A::DATA_5),
            7 => Some(DATALEN_A::DATA_8),
            30 => Some(DATALEN_A::DATA_31),
            31 => Some(DATALEN_A::DATA_32),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DATA_4`"]
    #[inline(always)]
    pub fn is_data_4(&self) -> bool {
        *self == DATALEN_A::DATA_4
    }
    #[doc = "Checks if the value of the field is `DATA_5`"]
    #[inline(always)]
    pub fn is_data_5(&self) -> bool {
        *self == DATALEN_A::DATA_5
    }
    #[doc = "Checks if the value of the field is `DATA_8`"]
    #[inline(always)]
    pub fn is_data_8(&self) -> bool {
        *self == DATALEN_A::DATA_8
    }
    #[doc = "Checks if the value of the field is `DATA_31`"]
    #[inline(always)]
    pub fn is_data_31(&self) -> bool {
        *self == DATALEN_A::DATA_31
    }
    #[doc = "Checks if the value of the field is `DATA_32`"]
    #[inline(always)]
    pub fn is_data_32(&self) -> bool {
        *self == DATALEN_A::DATA_32
    }
}
#[doc = "Field `DATALEN` writer - Data Length"]
pub type DATALEN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFG1_SPEC, u8, DATALEN_A, 5, O>;
impl<'a, const O: u8> DATALEN_W<'a, O> {
    #[doc = "Data is 4 bits in length."]
    #[inline(always)]
    pub fn data_4(self) -> &'a mut W {
        self.variant(DATALEN_A::DATA_4)
    }
    #[doc = "Data is 5 bits in length."]
    #[inline(always)]
    pub fn data_5(self) -> &'a mut W {
        self.variant(DATALEN_A::DATA_5)
    }
    #[doc = "Data is 8 bits in length."]
    #[inline(always)]
    pub fn data_8(self) -> &'a mut W {
        self.variant(DATALEN_A::DATA_8)
    }
    #[doc = "Data is 31 bits in length."]
    #[inline(always)]
    pub fn data_31(self) -> &'a mut W {
        self.variant(DATALEN_A::DATA_31)
    }
    #[doc = "Data is 32 bits in length."]
    #[inline(always)]
    pub fn data_32(self) -> &'a mut W {
        self.variant(DATALEN_A::DATA_32)
    }
}
impl R {
    #[doc = "Bit 0 - Main Enable"]
    #[inline(always)]
    pub fn mainenable(&self) -> MAINENABLE_R {
        MAINENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Data Flow Pause"]
    #[inline(always)]
    pub fn datapause(&self) -> DATAPAUSE_R {
        DATAPAUSE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Pair Count"]
    #[inline(always)]
    pub fn paircount(&self) -> PAIRCOUNT_R {
        PAIRCOUNT_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Master/Slave Configuration Selection"]
    #[inline(always)]
    pub fn mstslvcfg(&self) -> MSTSLVCFG_R {
        MSTSLVCFG_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Mode"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - Right Channel Low"]
    #[inline(always)]
    pub fn rightlow(&self) -> RIGHTLOW_R {
        RIGHTLOW_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Left-Justify Data"]
    #[inline(always)]
    pub fn leftjust(&self) -> LEFTJUST_R {
        LEFTJUST_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Single Channel Mode"]
    #[inline(always)]
    pub fn onechannel(&self) -> ONECHANNEL_R {
        ONECHANNEL_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PDM Data Selection"]
    #[inline(always)]
    pub fn pdmdata(&self) -> PDMDATA_R {
        PDMDATA_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - SCK Polarity"]
    #[inline(always)]
    pub fn sck_pol(&self) -> SCK_POL_R {
        SCK_POL_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - WS Polarity"]
    #[inline(always)]
    pub fn ws_pol(&self) -> WS_POL_R {
        WS_POL_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 16:20 - Data Length"]
    #[inline(always)]
    pub fn datalen(&self) -> DATALEN_R {
        DATALEN_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Main Enable"]
    #[inline(always)]
    #[must_use]
    pub fn mainenable(&mut self) -> MAINENABLE_W<0> {
        MAINENABLE_W::new(self)
    }
    #[doc = "Bit 1 - Data Flow Pause"]
    #[inline(always)]
    #[must_use]
    pub fn datapause(&mut self) -> DATAPAUSE_W<1> {
        DATAPAUSE_W::new(self)
    }
    #[doc = "Bits 2:3 - Pair Count"]
    #[inline(always)]
    #[must_use]
    pub fn paircount(&mut self) -> PAIRCOUNT_W<2> {
        PAIRCOUNT_W::new(self)
    }
    #[doc = "Bits 4:5 - Master/Slave Configuration Selection"]
    #[inline(always)]
    #[must_use]
    pub fn mstslvcfg(&mut self) -> MSTSLVCFG_W<4> {
        MSTSLVCFG_W::new(self)
    }
    #[doc = "Bits 6:7 - Mode"]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> MODE_W<6> {
        MODE_W::new(self)
    }
    #[doc = "Bit 8 - Right Channel Low"]
    #[inline(always)]
    #[must_use]
    pub fn rightlow(&mut self) -> RIGHTLOW_W<8> {
        RIGHTLOW_W::new(self)
    }
    #[doc = "Bit 9 - Left-Justify Data"]
    #[inline(always)]
    #[must_use]
    pub fn leftjust(&mut self) -> LEFTJUST_W<9> {
        LEFTJUST_W::new(self)
    }
    #[doc = "Bit 10 - Single Channel Mode"]
    #[inline(always)]
    #[must_use]
    pub fn onechannel(&mut self) -> ONECHANNEL_W<10> {
        ONECHANNEL_W::new(self)
    }
    #[doc = "Bit 11 - PDM Data Selection"]
    #[inline(always)]
    #[must_use]
    pub fn pdmdata(&mut self) -> PDMDATA_W<11> {
        PDMDATA_W::new(self)
    }
    #[doc = "Bit 12 - SCK Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn sck_pol(&mut self) -> SCK_POL_W<12> {
        SCK_POL_W::new(self)
    }
    #[doc = "Bit 13 - WS Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn ws_pol(&mut self) -> WS_POL_W<13> {
        WS_POL_W::new(self)
    }
    #[doc = "Bits 16:20 - Data Length"]
    #[inline(always)]
    #[must_use]
    pub fn datalen(&mut self) -> DATALEN_W<16> {
        DATALEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configuration Register 1 for the Primary Channel Pair\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfg1](index.html) module"]
pub struct CFG1_SPEC;
impl crate::RegisterSpec for CFG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfg1::R](R) reader structure"]
impl crate::Readable for CFG1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfg1::W](W) writer structure"]
impl crate::Writable for CFG1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFG1 to value 0"]
impl crate::Resettable for CFG1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
