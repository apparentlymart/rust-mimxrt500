#[doc = "Register `PARAM` reader"]
pub struct R(crate::R<PARAM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PARAM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PARAM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PARAM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TRIG_NUM` reader - Trigger Number"]
pub type TRIG_NUM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FIFOSIZE` reader - Result FIFO Depth"]
pub type FIFOSIZE_R = crate::FieldReader<u8, FIFOSIZE_A>;
#[doc = "Result FIFO Depth\n\nValue on reset: 16"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FIFOSIZE_A {
    #[doc = "1: 2"]
    ENTRIES_2 = 1,
    #[doc = "4: 4"]
    ENTRIES_4 = 4,
    #[doc = "8: 8"]
    ENTRIES_8 = 8,
    #[doc = "16: 16"]
    ENTRIES_16 = 16,
    #[doc = "32: 32"]
    ENTRIES_32 = 32,
    #[doc = "64: 64"]
    ENTRIES_64 = 64,
}
impl From<FIFOSIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: FIFOSIZE_A) -> Self {
        variant as _
    }
}
impl FIFOSIZE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<FIFOSIZE_A> {
        match self.bits {
            1 => Some(FIFOSIZE_A::ENTRIES_2),
            4 => Some(FIFOSIZE_A::ENTRIES_4),
            8 => Some(FIFOSIZE_A::ENTRIES_8),
            16 => Some(FIFOSIZE_A::ENTRIES_16),
            32 => Some(FIFOSIZE_A::ENTRIES_32),
            64 => Some(FIFOSIZE_A::ENTRIES_64),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ENTRIES_2`"]
    #[inline(always)]
    pub fn is_entries_2(&self) -> bool {
        *self == FIFOSIZE_A::ENTRIES_2
    }
    #[doc = "Checks if the value of the field is `ENTRIES_4`"]
    #[inline(always)]
    pub fn is_entries_4(&self) -> bool {
        *self == FIFOSIZE_A::ENTRIES_4
    }
    #[doc = "Checks if the value of the field is `ENTRIES_8`"]
    #[inline(always)]
    pub fn is_entries_8(&self) -> bool {
        *self == FIFOSIZE_A::ENTRIES_8
    }
    #[doc = "Checks if the value of the field is `ENTRIES_16`"]
    #[inline(always)]
    pub fn is_entries_16(&self) -> bool {
        *self == FIFOSIZE_A::ENTRIES_16
    }
    #[doc = "Checks if the value of the field is `ENTRIES_32`"]
    #[inline(always)]
    pub fn is_entries_32(&self) -> bool {
        *self == FIFOSIZE_A::ENTRIES_32
    }
    #[doc = "Checks if the value of the field is `ENTRIES_64`"]
    #[inline(always)]
    pub fn is_entries_64(&self) -> bool {
        *self == FIFOSIZE_A::ENTRIES_64
    }
}
#[doc = "Field `CV_NUM` reader - Compare Value Number"]
pub type CV_NUM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CMD_NUM` reader - Command Buffer Number"]
pub type CMD_NUM_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Trigger Number"]
    #[inline(always)]
    pub fn trig_num(&self) -> TRIG_NUM_R {
        TRIG_NUM_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Result FIFO Depth"]
    #[inline(always)]
    pub fn fifosize(&self) -> FIFOSIZE_R {
        FIFOSIZE_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Compare Value Number"]
    #[inline(always)]
    pub fn cv_num(&self) -> CV_NUM_R {
        CV_NUM_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Command Buffer Number"]
    #[inline(always)]
    pub fn cmd_num(&self) -> CMD_NUM_R {
        CMD_NUM_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "Parameter Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [param](index.html) module"]
pub struct PARAM_SPEC;
impl crate::RegisterSpec for PARAM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [param::R](R) reader structure"]
impl crate::Readable for PARAM_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PARAM to value 0x0f04_1010"]
impl crate::Resettable for PARAM_SPEC {
    const RESET_VALUE: Self::Ux = 0x0f04_1010;
}
