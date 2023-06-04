#[doc = "Register `SCAPABILITIES` reader"]
pub struct R(crate::R<SCAPABILITIES_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCAPABILITIES_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCAPABILITIES_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCAPABILITIES_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `IDENA` reader - ID 48b handler"]
pub type IDENA_R = crate::FieldReader<u8, IDENA_A>;
#[doc = "ID 48b handler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IDENA_A {
    #[doc = "0: APPLICATION"]
    APPLICATION = 0,
    #[doc = "1: HW"]
    HW = 1,
    #[doc = "2: HW_BUT"]
    HW_BUT = 2,
    #[doc = "3: PARTNO"]
    PARTNO = 3,
}
impl From<IDENA_A> for u8 {
    #[inline(always)]
    fn from(variant: IDENA_A) -> Self {
        variant as _
    }
}
impl IDENA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IDENA_A {
        match self.bits {
            0 => IDENA_A::APPLICATION,
            1 => IDENA_A::HW,
            2 => IDENA_A::HW_BUT,
            3 => IDENA_A::PARTNO,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `APPLICATION`"]
    #[inline(always)]
    pub fn is_application(&self) -> bool {
        *self == IDENA_A::APPLICATION
    }
    #[doc = "Checks if the value of the field is `HW`"]
    #[inline(always)]
    pub fn is_hw(&self) -> bool {
        *self == IDENA_A::HW
    }
    #[doc = "Checks if the value of the field is `HW_BUT`"]
    #[inline(always)]
    pub fn is_hw_but(&self) -> bool {
        *self == IDENA_A::HW_BUT
    }
    #[doc = "Checks if the value of the field is `PARTNO`"]
    #[inline(always)]
    pub fn is_partno(&self) -> bool {
        *self == IDENA_A::PARTNO
    }
}
#[doc = "Field `IDREG` reader - ID register"]
pub type IDREG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HDRSUPP` reader - HDR support"]
pub type HDRSUPP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MASTER` reader - Master"]
pub type MASTER_R = crate::BitReader<MASTER_A>;
#[doc = "Master\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MASTER_A {
    #[doc = "0: MASTERNOTSUPPORTED"]
    MASTERNOTSUPPORTED = 0,
    #[doc = "1: MASTERSUPPORTED"]
    MASTERSUPPORTED = 1,
}
impl From<MASTER_A> for bool {
    #[inline(always)]
    fn from(variant: MASTER_A) -> Self {
        variant as u8 != 0
    }
}
impl MASTER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MASTER_A {
        match self.bits {
            false => MASTER_A::MASTERNOTSUPPORTED,
            true => MASTER_A::MASTERSUPPORTED,
        }
    }
    #[doc = "Checks if the value of the field is `MASTERNOTSUPPORTED`"]
    #[inline(always)]
    pub fn is_masternotsupported(&self) -> bool {
        *self == MASTER_A::MASTERNOTSUPPORTED
    }
    #[doc = "Checks if the value of the field is `MASTERSUPPORTED`"]
    #[inline(always)]
    pub fn is_mastersupported(&self) -> bool {
        *self == MASTER_A::MASTERSUPPORTED
    }
}
#[doc = "Field `SADDR` reader - Static address"]
pub type SADDR_R = crate::FieldReader<u8, SADDR_A>;
#[doc = "Static address\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SADDR_A {
    #[doc = "0: NO_STATIC"]
    NO_STATIC = 0,
    #[doc = "1: STATIC"]
    STATIC = 1,
    #[doc = "2: HW_CONTROL"]
    HW_CONTROL = 2,
    #[doc = "3: CONFIG"]
    CONFIG = 3,
}
impl From<SADDR_A> for u8 {
    #[inline(always)]
    fn from(variant: SADDR_A) -> Self {
        variant as _
    }
}
impl SADDR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SADDR_A {
        match self.bits {
            0 => SADDR_A::NO_STATIC,
            1 => SADDR_A::STATIC,
            2 => SADDR_A::HW_CONTROL,
            3 => SADDR_A::CONFIG,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NO_STATIC`"]
    #[inline(always)]
    pub fn is_no_static(&self) -> bool {
        *self == SADDR_A::NO_STATIC
    }
    #[doc = "Checks if the value of the field is `STATIC`"]
    #[inline(always)]
    pub fn is_static(&self) -> bool {
        *self == SADDR_A::STATIC
    }
    #[doc = "Checks if the value of the field is `HW_CONTROL`"]
    #[inline(always)]
    pub fn is_hw_control(&self) -> bool {
        *self == SADDR_A::HW_CONTROL
    }
    #[doc = "Checks if the value of the field is `CONFIG`"]
    #[inline(always)]
    pub fn is_config(&self) -> bool {
        *self == SADDR_A::CONFIG
    }
}
#[doc = "Field `CCCHANDLE` reader - Common Command Codes (CCC) handling"]
pub type CCCHANDLE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IBI_MR_HJ` reader - In-Band Interrupts, Master Requests, Hot Join events"]
pub type IBI_MR_HJ_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TIMECTRL` reader - Time control"]
pub type TIMECTRL_R = crate::BitReader<TIMECTRL_A>;
#[doc = "Time control\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIMECTRL_A {
    #[doc = "0: NO_TIME_CONTROL_TYPE"]
    NO_TIME_CONTROL_TYPE = 0,
    #[doc = "1: NO_TIME_CONTROL_TYPE"]
    ATLEAST1_TIME_CONTROL = 1,
}
impl From<TIMECTRL_A> for bool {
    #[inline(always)]
    fn from(variant: TIMECTRL_A) -> Self {
        variant as u8 != 0
    }
}
impl TIMECTRL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIMECTRL_A {
        match self.bits {
            false => TIMECTRL_A::NO_TIME_CONTROL_TYPE,
            true => TIMECTRL_A::ATLEAST1_TIME_CONTROL,
        }
    }
    #[doc = "Checks if the value of the field is `NO_TIME_CONTROL_TYPE`"]
    #[inline(always)]
    pub fn is_no_time_control_type(&self) -> bool {
        *self == TIMECTRL_A::NO_TIME_CONTROL_TYPE
    }
    #[doc = "Checks if the value of the field is `ATLEAST1_TIME_CONTROL`"]
    #[inline(always)]
    pub fn is_atleast1_time_control(&self) -> bool {
        *self == TIMECTRL_A::ATLEAST1_TIME_CONTROL
    }
}
#[doc = "Field `EXTFIFO` reader - External FIFO"]
pub type EXTFIFO_R = crate::FieldReader<u8, EXTFIFO_A>;
#[doc = "External FIFO\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTFIFO_A {
    #[doc = "1: STD_EXT_FIFO:"]
    STD_EXT_FIFO = 1,
}
impl From<EXTFIFO_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTFIFO_A) -> Self {
        variant as _
    }
}
impl EXTFIFO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<EXTFIFO_A> {
        match self.bits {
            1 => Some(EXTFIFO_A::STD_EXT_FIFO),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `STD_EXT_FIFO`"]
    #[inline(always)]
    pub fn is_std_ext_fifo(&self) -> bool {
        *self == EXTFIFO_A::STD_EXT_FIFO
    }
}
#[doc = "Field `FIFOTX` reader - FIFO transmit"]
pub type FIFOTX_R = crate::FieldReader<u8, FIFOTX_A>;
#[doc = "FIFO transmit\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FIFOTX_A {
    #[doc = "0: FIFO_2BYTE"]
    FIFO_2BYTE = 0,
    #[doc = "1: FIFO_4BYTE"]
    FIFO_4BYTE = 1,
    #[doc = "2: FIFO_8BYTE"]
    FIFO_8BYTE = 2,
    #[doc = "3: FIFO_16BYTE"]
    FIFO_16BYTE = 3,
}
impl From<FIFOTX_A> for u8 {
    #[inline(always)]
    fn from(variant: FIFOTX_A) -> Self {
        variant as _
    }
}
impl FIFOTX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FIFOTX_A {
        match self.bits {
            0 => FIFOTX_A::FIFO_2BYTE,
            1 => FIFOTX_A::FIFO_4BYTE,
            2 => FIFOTX_A::FIFO_8BYTE,
            3 => FIFOTX_A::FIFO_16BYTE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `FIFO_2BYTE`"]
    #[inline(always)]
    pub fn is_fifo_2byte(&self) -> bool {
        *self == FIFOTX_A::FIFO_2BYTE
    }
    #[doc = "Checks if the value of the field is `FIFO_4BYTE`"]
    #[inline(always)]
    pub fn is_fifo_4byte(&self) -> bool {
        *self == FIFOTX_A::FIFO_4BYTE
    }
    #[doc = "Checks if the value of the field is `FIFO_8BYTE`"]
    #[inline(always)]
    pub fn is_fifo_8byte(&self) -> bool {
        *self == FIFOTX_A::FIFO_8BYTE
    }
    #[doc = "Checks if the value of the field is `FIFO_16BYTE`"]
    #[inline(always)]
    pub fn is_fifo_16byte(&self) -> bool {
        *self == FIFOTX_A::FIFO_16BYTE
    }
}
#[doc = "Field `FIFORX` reader - FIFO receive"]
pub type FIFORX_R = crate::FieldReader<u8, FIFORX_A>;
#[doc = "FIFO receive\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FIFORX_A {
    #[doc = "0: FIFO_2BYTE"]
    FIFO_2BYTE = 0,
    #[doc = "1: FIFO_4BYTE"]
    FIFO_4BYTE = 1,
    #[doc = "2: FIFO_8BYTE"]
    FIFO_8BYTE = 2,
    #[doc = "3: FIFO_16BYTE"]
    FIFO_16BYTE = 3,
}
impl From<FIFORX_A> for u8 {
    #[inline(always)]
    fn from(variant: FIFORX_A) -> Self {
        variant as _
    }
}
impl FIFORX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FIFORX_A {
        match self.bits {
            0 => FIFORX_A::FIFO_2BYTE,
            1 => FIFORX_A::FIFO_4BYTE,
            2 => FIFORX_A::FIFO_8BYTE,
            3 => FIFORX_A::FIFO_16BYTE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `FIFO_2BYTE`"]
    #[inline(always)]
    pub fn is_fifo_2byte(&self) -> bool {
        *self == FIFORX_A::FIFO_2BYTE
    }
    #[doc = "Checks if the value of the field is `FIFO_4BYTE`"]
    #[inline(always)]
    pub fn is_fifo_4byte(&self) -> bool {
        *self == FIFORX_A::FIFO_4BYTE
    }
    #[doc = "Checks if the value of the field is `FIFO_8BYTE`"]
    #[inline(always)]
    pub fn is_fifo_8byte(&self) -> bool {
        *self == FIFORX_A::FIFO_8BYTE
    }
    #[doc = "Checks if the value of the field is `FIFO_16BYTE`"]
    #[inline(always)]
    pub fn is_fifo_16byte(&self) -> bool {
        *self == FIFORX_A::FIFO_16BYTE
    }
}
#[doc = "Field `INT` reader - Interrupt"]
pub type INT_R = crate::BitReader<INT_A>;
#[doc = "Interrupt\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT_A {
    #[doc = "0: Interrupts are not supported"]
    INTERRUPTSNO = 0,
    #[doc = "1: Interrupts are supported."]
    INTERRUPTSYES = 1,
}
impl From<INT_A> for bool {
    #[inline(always)]
    fn from(variant: INT_A) -> Self {
        variant as u8 != 0
    }
}
impl INT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT_A {
        match self.bits {
            false => INT_A::INTERRUPTSNO,
            true => INT_A::INTERRUPTSYES,
        }
    }
    #[doc = "Checks if the value of the field is `INTERRUPTSNO`"]
    #[inline(always)]
    pub fn is_interruptsno(&self) -> bool {
        *self == INT_A::INTERRUPTSNO
    }
    #[doc = "Checks if the value of the field is `INTERRUPTSYES`"]
    #[inline(always)]
    pub fn is_interruptsyes(&self) -> bool {
        *self == INT_A::INTERRUPTSYES
    }
}
#[doc = "Field `DMA` reader - DMA"]
pub type DMA_R = crate::BitReader<DMA_A>;
#[doc = "DMA\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMA_A {
    #[doc = "0: DMA is not supported"]
    DMANO = 0,
    #[doc = "1: DMA is supported"]
    DMAYES = 1,
}
impl From<DMA_A> for bool {
    #[inline(always)]
    fn from(variant: DMA_A) -> Self {
        variant as u8 != 0
    }
}
impl DMA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMA_A {
        match self.bits {
            false => DMA_A::DMANO,
            true => DMA_A::DMAYES,
        }
    }
    #[doc = "Checks if the value of the field is `DMANO`"]
    #[inline(always)]
    pub fn is_dmano(&self) -> bool {
        *self == DMA_A::DMANO
    }
    #[doc = "Checks if the value of the field is `DMAYES`"]
    #[inline(always)]
    pub fn is_dmayes(&self) -> bool {
        *self == DMA_A::DMAYES
    }
}
impl R {
    #[doc = "Bits 0:1 - ID 48b handler"]
    #[inline(always)]
    pub fn idena(&self) -> IDENA_R {
        IDENA_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:5 - ID register"]
    #[inline(always)]
    pub fn idreg(&self) -> IDREG_R {
        IDREG_R::new(((self.bits >> 2) & 0x0f) as u8)
    }
    #[doc = "Bits 6:8 - HDR support"]
    #[inline(always)]
    pub fn hdrsupp(&self) -> HDRSUPP_R {
        HDRSUPP_R::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bit 9 - Master"]
    #[inline(always)]
    pub fn master(&self) -> MASTER_R {
        MASTER_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:11 - Static address"]
    #[inline(always)]
    pub fn saddr(&self) -> SADDR_R {
        SADDR_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:15 - Common Command Codes (CCC) handling"]
    #[inline(always)]
    pub fn ccchandle(&self) -> CCCHANDLE_R {
        CCCHANDLE_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:20 - In-Band Interrupts, Master Requests, Hot Join events"]
    #[inline(always)]
    pub fn ibi_mr_hj(&self) -> IBI_MR_HJ_R {
        IBI_MR_HJ_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bit 21 - Time control"]
    #[inline(always)]
    pub fn timectrl(&self) -> TIMECTRL_R {
        TIMECTRL_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 23:25 - External FIFO"]
    #[inline(always)]
    pub fn extfifo(&self) -> EXTFIFO_R {
        EXTFIFO_R::new(((self.bits >> 23) & 7) as u8)
    }
    #[doc = "Bits 26:27 - FIFO transmit"]
    #[inline(always)]
    pub fn fifotx(&self) -> FIFOTX_R {
        FIFOTX_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - FIFO receive"]
    #[inline(always)]
    pub fn fiforx(&self) -> FIFORX_R {
        FIFORX_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bit 30 - Interrupt"]
    #[inline(always)]
    pub fn int(&self) -> INT_R {
        INT_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - DMA"]
    #[inline(always)]
    pub fn dma(&self) -> DMA_R {
        DMA_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "Slave Capabilities Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scapabilities](index.html) module"]
pub struct SCAPABILITIES_SPEC;
impl crate::RegisterSpec for SCAPABILITIES_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [scapabilities::R](R) reader structure"]
impl crate::Readable for SCAPABILITIES_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SCAPABILITIES to value 0xe83f_fe78"]
impl crate::Resettable for SCAPABILITIES_SPEC {
    const RESET_VALUE: Self::Ux = 0xe83f_fe78;
}
