#[doc = "Register `CFG2` reader"]
pub struct R(crate::R<CFG2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFG2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFG2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFG2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFG2` writer"]
pub struct W(crate::W<CFG2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFG2_SPEC>;
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
impl From<crate::W<CFG2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFG2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FRAMELEN` reader - Frame Length"]
pub type FRAMELEN_R = crate::FieldReader<u16, FRAMELEN_A>;
#[doc = "Frame Length\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum FRAMELEN_A {
    #[doc = "3: Frame is 4 bits in total length"]
    FRAME_LEN_4 = 3,
    #[doc = "4: Frame is 5 bits in total length"]
    FRAME_LEN_5 = 4,
    #[doc = "511: Frame is 512 bits in total length"]
    FRAME_LEN_512 = 511,
    #[doc = "2047: Frame is 2048 bits in total length"]
    FRAME_LEN_2048 = 2047,
}
impl From<FRAMELEN_A> for u16 {
    #[inline(always)]
    fn from(variant: FRAMELEN_A) -> Self {
        variant as _
    }
}
impl FRAMELEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<FRAMELEN_A> {
        match self.bits {
            3 => Some(FRAMELEN_A::FRAME_LEN_4),
            4 => Some(FRAMELEN_A::FRAME_LEN_5),
            511 => Some(FRAMELEN_A::FRAME_LEN_512),
            2047 => Some(FRAMELEN_A::FRAME_LEN_2048),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `FRAME_LEN_4`"]
    #[inline(always)]
    pub fn is_frame_len_4(&self) -> bool {
        *self == FRAMELEN_A::FRAME_LEN_4
    }
    #[doc = "Checks if the value of the field is `FRAME_LEN_5`"]
    #[inline(always)]
    pub fn is_frame_len_5(&self) -> bool {
        *self == FRAMELEN_A::FRAME_LEN_5
    }
    #[doc = "Checks if the value of the field is `FRAME_LEN_512`"]
    #[inline(always)]
    pub fn is_frame_len_512(&self) -> bool {
        *self == FRAMELEN_A::FRAME_LEN_512
    }
    #[doc = "Checks if the value of the field is `FRAME_LEN_2048`"]
    #[inline(always)]
    pub fn is_frame_len_2048(&self) -> bool {
        *self == FRAMELEN_A::FRAME_LEN_2048
    }
}
#[doc = "Field `FRAMELEN` writer - Frame Length"]
pub type FRAMELEN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CFG2_SPEC, u16, FRAMELEN_A, 11, O>;
impl<'a, const O: u8> FRAMELEN_W<'a, O> {
    #[doc = "Frame is 4 bits in total length"]
    #[inline(always)]
    pub fn frame_len_4(self) -> &'a mut W {
        self.variant(FRAMELEN_A::FRAME_LEN_4)
    }
    #[doc = "Frame is 5 bits in total length"]
    #[inline(always)]
    pub fn frame_len_5(self) -> &'a mut W {
        self.variant(FRAMELEN_A::FRAME_LEN_5)
    }
    #[doc = "Frame is 512 bits in total length"]
    #[inline(always)]
    pub fn frame_len_512(self) -> &'a mut W {
        self.variant(FRAMELEN_A::FRAME_LEN_512)
    }
    #[doc = "Frame is 2048 bits in total length"]
    #[inline(always)]
    pub fn frame_len_2048(self) -> &'a mut W {
        self.variant(FRAMELEN_A::FRAME_LEN_2048)
    }
}
#[doc = "Field `POSITION` reader - Data Position"]
pub type POSITION_R = crate::FieldReader<u16, POSITION_A>;
#[doc = "Data Position\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum POSITION_A {
    #[doc = "0: Data begins at bit position 0 (the first bit position) within the frame or WS phase"]
    POSITION_0 = 0,
    #[doc = "1: Data begins at bit position 1 within the frame or WS phase"]
    POSITION_1 = 1,
    #[doc = "2: Data begins at bit position 2 within the frame or WS phase"]
    POSITION_2 = 2,
}
impl From<POSITION_A> for u16 {
    #[inline(always)]
    fn from(variant: POSITION_A) -> Self {
        variant as _
    }
}
impl POSITION_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<POSITION_A> {
        match self.bits {
            0 => Some(POSITION_A::POSITION_0),
            1 => Some(POSITION_A::POSITION_1),
            2 => Some(POSITION_A::POSITION_2),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `POSITION_0`"]
    #[inline(always)]
    pub fn is_position_0(&self) -> bool {
        *self == POSITION_A::POSITION_0
    }
    #[doc = "Checks if the value of the field is `POSITION_1`"]
    #[inline(always)]
    pub fn is_position_1(&self) -> bool {
        *self == POSITION_A::POSITION_1
    }
    #[doc = "Checks if the value of the field is `POSITION_2`"]
    #[inline(always)]
    pub fn is_position_2(&self) -> bool {
        *self == POSITION_A::POSITION_2
    }
}
#[doc = "Field `POSITION` writer - Data Position"]
pub type POSITION_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CFG2_SPEC, u16, POSITION_A, 11, O>;
impl<'a, const O: u8> POSITION_W<'a, O> {
    #[doc = "Data begins at bit position 0 (the first bit position) within the frame or WS phase"]
    #[inline(always)]
    pub fn position_0(self) -> &'a mut W {
        self.variant(POSITION_A::POSITION_0)
    }
    #[doc = "Data begins at bit position 1 within the frame or WS phase"]
    #[inline(always)]
    pub fn position_1(self) -> &'a mut W {
        self.variant(POSITION_A::POSITION_1)
    }
    #[doc = "Data begins at bit position 2 within the frame or WS phase"]
    #[inline(always)]
    pub fn position_2(self) -> &'a mut W {
        self.variant(POSITION_A::POSITION_2)
    }
}
impl R {
    #[doc = "Bits 0:10 - Frame Length"]
    #[inline(always)]
    pub fn framelen(&self) -> FRAMELEN_R {
        FRAMELEN_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 16:26 - Data Position"]
    #[inline(always)]
    pub fn position(&self) -> POSITION_R {
        POSITION_R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - Frame Length"]
    #[inline(always)]
    #[must_use]
    pub fn framelen(&mut self) -> FRAMELEN_W<0> {
        FRAMELEN_W::new(self)
    }
    #[doc = "Bits 16:26 - Data Position"]
    #[inline(always)]
    #[must_use]
    pub fn position(&mut self) -> POSITION_W<16> {
        POSITION_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configuration Register 2 for the Primary Channel Pair\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfg2](index.html) module"]
pub struct CFG2_SPEC;
impl crate::RegisterSpec for CFG2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfg2::R](R) reader structure"]
impl crate::Readable for CFG2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfg2::W](W) writer structure"]
impl crate::Writable for CFG2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFG2 to value 0"]
impl crate::Resettable for CFG2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
