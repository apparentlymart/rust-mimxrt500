#[doc = "Register `STAT` reader"]
pub struct R(crate::R<STAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STAT` writer"]
pub struct W(crate::W<STAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STAT_SPEC>;
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
impl From<crate::W<STAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BUSY` reader - Busy Status"]
pub type BUSY_R = crate::BitReader<BUSY_A>;
#[doc = "Busy Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUSY_A {
    #[doc = "0: Idle"]
    IDLE = 0,
    #[doc = "1: Busy"]
    BUSY = 1,
}
impl From<BUSY_A> for bool {
    #[inline(always)]
    fn from(variant: BUSY_A) -> Self {
        variant as u8 != 0
    }
}
impl BUSY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUSY_A {
        match self.bits {
            false => BUSY_A::IDLE,
            true => BUSY_A::BUSY,
        }
    }
    #[doc = "Checks if the value of the field is `IDLE`"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == BUSY_A::IDLE
    }
    #[doc = "Checks if the value of the field is `BUSY`"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == BUSY_A::BUSY
    }
}
#[doc = "Field `SLVFRMERR` reader - Slave Frame Error"]
pub type SLVFRMERR_R = crate::BitReader<SLVFRMERR_A>;
#[doc = "Slave Frame Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SLVFRMERR_A {
    #[doc = "0: No error"]
    NO_ERROR = 0,
    #[doc = "1: Error"]
    ERROR = 1,
}
impl From<SLVFRMERR_A> for bool {
    #[inline(always)]
    fn from(variant: SLVFRMERR_A) -> Self {
        variant as u8 != 0
    }
}
impl SLVFRMERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLVFRMERR_A {
        match self.bits {
            false => SLVFRMERR_A::NO_ERROR,
            true => SLVFRMERR_A::ERROR,
        }
    }
    #[doc = "Checks if the value of the field is `NO_ERROR`"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == SLVFRMERR_A::NO_ERROR
    }
    #[doc = "Checks if the value of the field is `ERROR`"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == SLVFRMERR_A::ERROR
    }
}
#[doc = "Field `SLVFRMERR` writer - Slave Frame Error"]
pub type SLVFRMERR_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, STAT_SPEC, SLVFRMERR_A, O>;
impl<'a, const O: u8> SLVFRMERR_W<'a, O> {
    #[doc = "No error"]
    #[inline(always)]
    pub fn no_error(self) -> &'a mut W {
        self.variant(SLVFRMERR_A::NO_ERROR)
    }
    #[doc = "Error"]
    #[inline(always)]
    pub fn error(self) -> &'a mut W {
        self.variant(SLVFRMERR_A::ERROR)
    }
}
#[doc = "Field `LR` reader - Left/Right Indication"]
pub type LR_R = crate::BitReader<LR_A>;
#[doc = "Left/Right Indication\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LR_A {
    #[doc = "0: Left channel"]
    LEFT_CHANNEL = 0,
    #[doc = "1: Right channel"]
    RIGHT_CHANNEL = 1,
}
impl From<LR_A> for bool {
    #[inline(always)]
    fn from(variant: LR_A) -> Self {
        variant as u8 != 0
    }
}
impl LR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LR_A {
        match self.bits {
            false => LR_A::LEFT_CHANNEL,
            true => LR_A::RIGHT_CHANNEL,
        }
    }
    #[doc = "Checks if the value of the field is `LEFT_CHANNEL`"]
    #[inline(always)]
    pub fn is_left_channel(&self) -> bool {
        *self == LR_A::LEFT_CHANNEL
    }
    #[doc = "Checks if the value of the field is `RIGHT_CHANNEL`"]
    #[inline(always)]
    pub fn is_right_channel(&self) -> bool {
        *self == LR_A::RIGHT_CHANNEL
    }
}
#[doc = "Field `DATAPAUSED` reader - Data Paused"]
pub type DATAPAUSED_R = crate::BitReader<DATAPAUSED_A>;
#[doc = "Data Paused\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DATAPAUSED_A {
    #[doc = "0: Not Paused"]
    NOT_PAUSED = 0,
    #[doc = "1: Paused"]
    PAUSED = 1,
}
impl From<DATAPAUSED_A> for bool {
    #[inline(always)]
    fn from(variant: DATAPAUSED_A) -> Self {
        variant as u8 != 0
    }
}
impl DATAPAUSED_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DATAPAUSED_A {
        match self.bits {
            false => DATAPAUSED_A::NOT_PAUSED,
            true => DATAPAUSED_A::PAUSED,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_PAUSED`"]
    #[inline(always)]
    pub fn is_not_paused(&self) -> bool {
        *self == DATAPAUSED_A::NOT_PAUSED
    }
    #[doc = "Checks if the value of the field is `PAUSED`"]
    #[inline(always)]
    pub fn is_paused(&self) -> bool {
        *self == DATAPAUSED_A::PAUSED
    }
}
impl R {
    #[doc = "Bit 0 - Busy Status"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Slave Frame Error"]
    #[inline(always)]
    pub fn slvfrmerr(&self) -> SLVFRMERR_R {
        SLVFRMERR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Left/Right Indication"]
    #[inline(always)]
    pub fn lr(&self) -> LR_R {
        LR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Data Paused"]
    #[inline(always)]
    pub fn datapaused(&self) -> DATAPAUSED_R {
        DATAPAUSED_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Slave Frame Error"]
    #[inline(always)]
    #[must_use]
    pub fn slvfrmerr(&mut self) -> SLVFRMERR_W<1> {
        SLVFRMERR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Status Register for the Primary Channel Pair\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stat](index.html) module"]
pub struct STAT_SPEC;
impl crate::RegisterSpec for STAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [stat::R](R) reader structure"]
impl crate::Readable for STAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [stat::W](W) writer structure"]
impl crate::Writable for STAT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x02;
}
#[doc = "`reset()` method sets STAT to value 0"]
impl crate::Resettable for STAT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
