#[doc = "Register `DLY` reader"]
pub struct R(crate::R<DLY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DLY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DLY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DLY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DLY` writer"]
pub struct W(crate::W<DLY_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DLY_SPEC>;
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
impl From<crate::W<DLY_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DLY_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRE_DELAY` reader - Pre-Delay"]
pub type PRE_DELAY_R = crate::FieldReader<u8, PRE_DELAY_A>;
#[doc = "Pre-Delay\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PRE_DELAY_A {
    #[doc = "0: No additional time is inserted"]
    PRE_DELAY0 = 0,
    #[doc = "1: 1 SPI clock time is inserted"]
    PRE_DELAY1 = 1,
    #[doc = "2: 2 SPI clock times are inserted"]
    PRE_DELAY2 = 2,
    #[doc = "15: 15 SPI clock times are inserted"]
    PRE_DELAY15 = 15,
}
impl From<PRE_DELAY_A> for u8 {
    #[inline(always)]
    fn from(variant: PRE_DELAY_A) -> Self {
        variant as _
    }
}
impl PRE_DELAY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PRE_DELAY_A> {
        match self.bits {
            0 => Some(PRE_DELAY_A::PRE_DELAY0),
            1 => Some(PRE_DELAY_A::PRE_DELAY1),
            2 => Some(PRE_DELAY_A::PRE_DELAY2),
            15 => Some(PRE_DELAY_A::PRE_DELAY15),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `PRE_DELAY0`"]
    #[inline(always)]
    pub fn is_pre_delay0(&self) -> bool {
        *self == PRE_DELAY_A::PRE_DELAY0
    }
    #[doc = "Checks if the value of the field is `PRE_DELAY1`"]
    #[inline(always)]
    pub fn is_pre_delay1(&self) -> bool {
        *self == PRE_DELAY_A::PRE_DELAY1
    }
    #[doc = "Checks if the value of the field is `PRE_DELAY2`"]
    #[inline(always)]
    pub fn is_pre_delay2(&self) -> bool {
        *self == PRE_DELAY_A::PRE_DELAY2
    }
    #[doc = "Checks if the value of the field is `PRE_DELAY15`"]
    #[inline(always)]
    pub fn is_pre_delay15(&self) -> bool {
        *self == PRE_DELAY_A::PRE_DELAY15
    }
}
#[doc = "Field `PRE_DELAY` writer - Pre-Delay"]
pub type PRE_DELAY_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DLY_SPEC, u8, PRE_DELAY_A, 4, O>;
impl<'a, const O: u8> PRE_DELAY_W<'a, O> {
    #[doc = "No additional time is inserted"]
    #[inline(always)]
    pub fn pre_delay0(self) -> &'a mut W {
        self.variant(PRE_DELAY_A::PRE_DELAY0)
    }
    #[doc = "1 SPI clock time is inserted"]
    #[inline(always)]
    pub fn pre_delay1(self) -> &'a mut W {
        self.variant(PRE_DELAY_A::PRE_DELAY1)
    }
    #[doc = "2 SPI clock times are inserted"]
    #[inline(always)]
    pub fn pre_delay2(self) -> &'a mut W {
        self.variant(PRE_DELAY_A::PRE_DELAY2)
    }
    #[doc = "15 SPI clock times are inserted"]
    #[inline(always)]
    pub fn pre_delay15(self) -> &'a mut W {
        self.variant(PRE_DELAY_A::PRE_DELAY15)
    }
}
#[doc = "Field `POST_DELAY` reader - Post-Delay"]
pub type POST_DELAY_R = crate::FieldReader<u8, POST_DELAY_A>;
#[doc = "Post-Delay\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum POST_DELAY_A {
    #[doc = "0: No additional time is inserted"]
    POST_DELAY0 = 0,
    #[doc = "1: 1 SPI clock time is inserted"]
    POST_DELAY1 = 1,
    #[doc = "2: 2 SPI clock times are inserted"]
    POST_DELAY2 = 2,
    #[doc = "15: 15 SPI clock times are inserted"]
    POST_DELAY15 = 15,
}
impl From<POST_DELAY_A> for u8 {
    #[inline(always)]
    fn from(variant: POST_DELAY_A) -> Self {
        variant as _
    }
}
impl POST_DELAY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<POST_DELAY_A> {
        match self.bits {
            0 => Some(POST_DELAY_A::POST_DELAY0),
            1 => Some(POST_DELAY_A::POST_DELAY1),
            2 => Some(POST_DELAY_A::POST_DELAY2),
            15 => Some(POST_DELAY_A::POST_DELAY15),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `POST_DELAY0`"]
    #[inline(always)]
    pub fn is_post_delay0(&self) -> bool {
        *self == POST_DELAY_A::POST_DELAY0
    }
    #[doc = "Checks if the value of the field is `POST_DELAY1`"]
    #[inline(always)]
    pub fn is_post_delay1(&self) -> bool {
        *self == POST_DELAY_A::POST_DELAY1
    }
    #[doc = "Checks if the value of the field is `POST_DELAY2`"]
    #[inline(always)]
    pub fn is_post_delay2(&self) -> bool {
        *self == POST_DELAY_A::POST_DELAY2
    }
    #[doc = "Checks if the value of the field is `POST_DELAY15`"]
    #[inline(always)]
    pub fn is_post_delay15(&self) -> bool {
        *self == POST_DELAY_A::POST_DELAY15
    }
}
#[doc = "Field `POST_DELAY` writer - Post-Delay"]
pub type POST_DELAY_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DLY_SPEC, u8, POST_DELAY_A, 4, O>;
impl<'a, const O: u8> POST_DELAY_W<'a, O> {
    #[doc = "No additional time is inserted"]
    #[inline(always)]
    pub fn post_delay0(self) -> &'a mut W {
        self.variant(POST_DELAY_A::POST_DELAY0)
    }
    #[doc = "1 SPI clock time is inserted"]
    #[inline(always)]
    pub fn post_delay1(self) -> &'a mut W {
        self.variant(POST_DELAY_A::POST_DELAY1)
    }
    #[doc = "2 SPI clock times are inserted"]
    #[inline(always)]
    pub fn post_delay2(self) -> &'a mut W {
        self.variant(POST_DELAY_A::POST_DELAY2)
    }
    #[doc = "15 SPI clock times are inserted"]
    #[inline(always)]
    pub fn post_delay15(self) -> &'a mut W {
        self.variant(POST_DELAY_A::POST_DELAY15)
    }
}
#[doc = "Field `FRAME_DELAY` reader - Frame Delay"]
pub type FRAME_DELAY_R = crate::FieldReader<u8, FRAME_DELAY_A>;
#[doc = "Frame Delay\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FRAME_DELAY_A {
    #[doc = "0: No additional time is inserted"]
    FRAME_DELAY0 = 0,
    #[doc = "1: 1 SPI clock time is inserted"]
    FRAME_DELAY1 = 1,
    #[doc = "2: 2 SPI clock times are inserted"]
    FRAME_DELAY2 = 2,
    #[doc = "15: 15 SPI clock times are inserted"]
    FRAME_DELAY15 = 15,
}
impl From<FRAME_DELAY_A> for u8 {
    #[inline(always)]
    fn from(variant: FRAME_DELAY_A) -> Self {
        variant as _
    }
}
impl FRAME_DELAY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<FRAME_DELAY_A> {
        match self.bits {
            0 => Some(FRAME_DELAY_A::FRAME_DELAY0),
            1 => Some(FRAME_DELAY_A::FRAME_DELAY1),
            2 => Some(FRAME_DELAY_A::FRAME_DELAY2),
            15 => Some(FRAME_DELAY_A::FRAME_DELAY15),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `FRAME_DELAY0`"]
    #[inline(always)]
    pub fn is_frame_delay0(&self) -> bool {
        *self == FRAME_DELAY_A::FRAME_DELAY0
    }
    #[doc = "Checks if the value of the field is `FRAME_DELAY1`"]
    #[inline(always)]
    pub fn is_frame_delay1(&self) -> bool {
        *self == FRAME_DELAY_A::FRAME_DELAY1
    }
    #[doc = "Checks if the value of the field is `FRAME_DELAY2`"]
    #[inline(always)]
    pub fn is_frame_delay2(&self) -> bool {
        *self == FRAME_DELAY_A::FRAME_DELAY2
    }
    #[doc = "Checks if the value of the field is `FRAME_DELAY15`"]
    #[inline(always)]
    pub fn is_frame_delay15(&self) -> bool {
        *self == FRAME_DELAY_A::FRAME_DELAY15
    }
}
#[doc = "Field `FRAME_DELAY` writer - Frame Delay"]
pub type FRAME_DELAY_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DLY_SPEC, u8, FRAME_DELAY_A, 4, O>;
impl<'a, const O: u8> FRAME_DELAY_W<'a, O> {
    #[doc = "No additional time is inserted"]
    #[inline(always)]
    pub fn frame_delay0(self) -> &'a mut W {
        self.variant(FRAME_DELAY_A::FRAME_DELAY0)
    }
    #[doc = "1 SPI clock time is inserted"]
    #[inline(always)]
    pub fn frame_delay1(self) -> &'a mut W {
        self.variant(FRAME_DELAY_A::FRAME_DELAY1)
    }
    #[doc = "2 SPI clock times are inserted"]
    #[inline(always)]
    pub fn frame_delay2(self) -> &'a mut W {
        self.variant(FRAME_DELAY_A::FRAME_DELAY2)
    }
    #[doc = "15 SPI clock times are inserted"]
    #[inline(always)]
    pub fn frame_delay15(self) -> &'a mut W {
        self.variant(FRAME_DELAY_A::FRAME_DELAY15)
    }
}
#[doc = "Field `TRANSFER_DELAY` reader - Transfer Delay"]
pub type TRANSFER_DELAY_R = crate::FieldReader<u8, TRANSFER_DELAY_A>;
#[doc = "Transfer Delay\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TRANSFER_DELAY_A {
    #[doc = "0: The minimum time that SSEL is deasserted is 1 SPI clock time (zero-added time)"]
    TRANSFER_DELAY1 = 0,
    #[doc = "1: The minimum time that SSEL is deasserted is 2 SPI clock times"]
    TRANSFER_DELAY2 = 1,
    #[doc = "2: The minimum time that SSEL is deasserted is 3 SPI clock times"]
    TRANSFER_DELAY3 = 2,
    #[doc = "15: The minimum time that SSEL is deasserted is 16 SPI clock times"]
    TRANSFER_DELAY16 = 15,
}
impl From<TRANSFER_DELAY_A> for u8 {
    #[inline(always)]
    fn from(variant: TRANSFER_DELAY_A) -> Self {
        variant as _
    }
}
impl TRANSFER_DELAY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TRANSFER_DELAY_A> {
        match self.bits {
            0 => Some(TRANSFER_DELAY_A::TRANSFER_DELAY1),
            1 => Some(TRANSFER_DELAY_A::TRANSFER_DELAY2),
            2 => Some(TRANSFER_DELAY_A::TRANSFER_DELAY3),
            15 => Some(TRANSFER_DELAY_A::TRANSFER_DELAY16),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `TRANSFER_DELAY1`"]
    #[inline(always)]
    pub fn is_transfer_delay1(&self) -> bool {
        *self == TRANSFER_DELAY_A::TRANSFER_DELAY1
    }
    #[doc = "Checks if the value of the field is `TRANSFER_DELAY2`"]
    #[inline(always)]
    pub fn is_transfer_delay2(&self) -> bool {
        *self == TRANSFER_DELAY_A::TRANSFER_DELAY2
    }
    #[doc = "Checks if the value of the field is `TRANSFER_DELAY3`"]
    #[inline(always)]
    pub fn is_transfer_delay3(&self) -> bool {
        *self == TRANSFER_DELAY_A::TRANSFER_DELAY3
    }
    #[doc = "Checks if the value of the field is `TRANSFER_DELAY16`"]
    #[inline(always)]
    pub fn is_transfer_delay16(&self) -> bool {
        *self == TRANSFER_DELAY_A::TRANSFER_DELAY16
    }
}
#[doc = "Field `TRANSFER_DELAY` writer - Transfer Delay"]
pub type TRANSFER_DELAY_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DLY_SPEC, u8, TRANSFER_DELAY_A, 4, O>;
impl<'a, const O: u8> TRANSFER_DELAY_W<'a, O> {
    #[doc = "The minimum time that SSEL is deasserted is 1 SPI clock time (zero-added time)"]
    #[inline(always)]
    pub fn transfer_delay1(self) -> &'a mut W {
        self.variant(TRANSFER_DELAY_A::TRANSFER_DELAY1)
    }
    #[doc = "The minimum time that SSEL is deasserted is 2 SPI clock times"]
    #[inline(always)]
    pub fn transfer_delay2(self) -> &'a mut W {
        self.variant(TRANSFER_DELAY_A::TRANSFER_DELAY2)
    }
    #[doc = "The minimum time that SSEL is deasserted is 3 SPI clock times"]
    #[inline(always)]
    pub fn transfer_delay3(self) -> &'a mut W {
        self.variant(TRANSFER_DELAY_A::TRANSFER_DELAY3)
    }
    #[doc = "The minimum time that SSEL is deasserted is 16 SPI clock times"]
    #[inline(always)]
    pub fn transfer_delay16(self) -> &'a mut W {
        self.variant(TRANSFER_DELAY_A::TRANSFER_DELAY16)
    }
}
impl R {
    #[doc = "Bits 0:3 - Pre-Delay"]
    #[inline(always)]
    pub fn pre_delay(&self) -> PRE_DELAY_R {
        PRE_DELAY_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Post-Delay"]
    #[inline(always)]
    pub fn post_delay(&self) -> POST_DELAY_R {
        POST_DELAY_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Frame Delay"]
    #[inline(always)]
    pub fn frame_delay(&self) -> FRAME_DELAY_R {
        FRAME_DELAY_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Transfer Delay"]
    #[inline(always)]
    pub fn transfer_delay(&self) -> TRANSFER_DELAY_R {
        TRANSFER_DELAY_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Pre-Delay"]
    #[inline(always)]
    #[must_use]
    pub fn pre_delay(&mut self) -> PRE_DELAY_W<0> {
        PRE_DELAY_W::new(self)
    }
    #[doc = "Bits 4:7 - Post-Delay"]
    #[inline(always)]
    #[must_use]
    pub fn post_delay(&mut self) -> POST_DELAY_W<4> {
        POST_DELAY_W::new(self)
    }
    #[doc = "Bits 8:11 - Frame Delay"]
    #[inline(always)]
    #[must_use]
    pub fn frame_delay(&mut self) -> FRAME_DELAY_W<8> {
        FRAME_DELAY_W::new(self)
    }
    #[doc = "Bits 12:15 - Transfer Delay"]
    #[inline(always)]
    #[must_use]
    pub fn transfer_delay(&mut self) -> TRANSFER_DELAY_W<12> {
        TRANSFER_DELAY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Delay Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dly](index.html) module"]
pub struct DLY_SPEC;
impl crate::RegisterSpec for DLY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dly::R](R) reader structure"]
impl crate::Readable for DLY_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dly::W](W) writer structure"]
impl crate::Writable for DLY_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DLY to value 0"]
impl crate::Resettable for DLY_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
