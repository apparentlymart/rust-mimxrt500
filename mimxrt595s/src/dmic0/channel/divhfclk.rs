#[doc = "Register `DIVHFCLK` reader"]
pub struct R(crate::R<DIVHFCLK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DIVHFCLK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DIVHFCLK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DIVHFCLK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DIVHFCLK` writer"]
pub struct W(crate::W<DIVHFCLK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DIVHFCLK_SPEC>;
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
impl From<crate::W<DIVHFCLK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DIVHFCLK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PDMDIV` reader - PDM Clock Divider Value"]
pub type PDMDIV_R = crate::FieldReader<u8, PDMDIV_A>;
#[doc = "PDM Clock Divider Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PDMDIV_A {
    #[doc = "0: Divide by 1"]
    DIV1 = 0,
    #[doc = "1: Divide by 2"]
    DIV2 = 1,
    #[doc = "2: Divide by 3"]
    DIV3 = 2,
    #[doc = "3: Divide by 4"]
    DIV4 = 3,
    #[doc = "4: Divide by 6"]
    DIV6 = 4,
    #[doc = "5: Divide by 8"]
    DIV8 = 5,
    #[doc = "6: Divide by 12"]
    DIV12 = 6,
    #[doc = "7: Divide by 16"]
    DIV16 = 7,
    #[doc = "8: Divide by 24"]
    DIV24 = 8,
    #[doc = "9: Divide by 32"]
    DIV32 = 9,
    #[doc = "10: Divide by 48"]
    DIV48 = 10,
    #[doc = "11: Divide by 64"]
    DIV64 = 11,
    #[doc = "12: Divide by 96"]
    DIV96 = 12,
    #[doc = "13: Divide by 128"]
    DIV128 = 13,
}
impl From<PDMDIV_A> for u8 {
    #[inline(always)]
    fn from(variant: PDMDIV_A) -> Self {
        variant as _
    }
}
impl PDMDIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PDMDIV_A> {
        match self.bits {
            0 => Some(PDMDIV_A::DIV1),
            1 => Some(PDMDIV_A::DIV2),
            2 => Some(PDMDIV_A::DIV3),
            3 => Some(PDMDIV_A::DIV4),
            4 => Some(PDMDIV_A::DIV6),
            5 => Some(PDMDIV_A::DIV8),
            6 => Some(PDMDIV_A::DIV12),
            7 => Some(PDMDIV_A::DIV16),
            8 => Some(PDMDIV_A::DIV24),
            9 => Some(PDMDIV_A::DIV32),
            10 => Some(PDMDIV_A::DIV48),
            11 => Some(PDMDIV_A::DIV64),
            12 => Some(PDMDIV_A::DIV96),
            13 => Some(PDMDIV_A::DIV128),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DIV1`"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == PDMDIV_A::DIV1
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == PDMDIV_A::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV3`"]
    #[inline(always)]
    pub fn is_div3(&self) -> bool {
        *self == PDMDIV_A::DIV3
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == PDMDIV_A::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV6`"]
    #[inline(always)]
    pub fn is_div6(&self) -> bool {
        *self == PDMDIV_A::DIV6
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == PDMDIV_A::DIV8
    }
    #[doc = "Checks if the value of the field is `DIV12`"]
    #[inline(always)]
    pub fn is_div12(&self) -> bool {
        *self == PDMDIV_A::DIV12
    }
    #[doc = "Checks if the value of the field is `DIV16`"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == PDMDIV_A::DIV16
    }
    #[doc = "Checks if the value of the field is `DIV24`"]
    #[inline(always)]
    pub fn is_div24(&self) -> bool {
        *self == PDMDIV_A::DIV24
    }
    #[doc = "Checks if the value of the field is `DIV32`"]
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == PDMDIV_A::DIV32
    }
    #[doc = "Checks if the value of the field is `DIV48`"]
    #[inline(always)]
    pub fn is_div48(&self) -> bool {
        *self == PDMDIV_A::DIV48
    }
    #[doc = "Checks if the value of the field is `DIV64`"]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == PDMDIV_A::DIV64
    }
    #[doc = "Checks if the value of the field is `DIV96`"]
    #[inline(always)]
    pub fn is_div96(&self) -> bool {
        *self == PDMDIV_A::DIV96
    }
    #[doc = "Checks if the value of the field is `DIV128`"]
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == PDMDIV_A::DIV128
    }
}
#[doc = "Field `PDMDIV` writer - PDM Clock Divider Value"]
pub type PDMDIV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DIVHFCLK_SPEC, u8, PDMDIV_A, 4, O>;
impl<'a, const O: u8> PDMDIV_W<'a, O> {
    #[doc = "Divide by 1"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(PDMDIV_A::DIV1)
    }
    #[doc = "Divide by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(PDMDIV_A::DIV2)
    }
    #[doc = "Divide by 3"]
    #[inline(always)]
    pub fn div3(self) -> &'a mut W {
        self.variant(PDMDIV_A::DIV3)
    }
    #[doc = "Divide by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(PDMDIV_A::DIV4)
    }
    #[doc = "Divide by 6"]
    #[inline(always)]
    pub fn div6(self) -> &'a mut W {
        self.variant(PDMDIV_A::DIV6)
    }
    #[doc = "Divide by 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(PDMDIV_A::DIV8)
    }
    #[doc = "Divide by 12"]
    #[inline(always)]
    pub fn div12(self) -> &'a mut W {
        self.variant(PDMDIV_A::DIV12)
    }
    #[doc = "Divide by 16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut W {
        self.variant(PDMDIV_A::DIV16)
    }
    #[doc = "Divide by 24"]
    #[inline(always)]
    pub fn div24(self) -> &'a mut W {
        self.variant(PDMDIV_A::DIV24)
    }
    #[doc = "Divide by 32"]
    #[inline(always)]
    pub fn div32(self) -> &'a mut W {
        self.variant(PDMDIV_A::DIV32)
    }
    #[doc = "Divide by 48"]
    #[inline(always)]
    pub fn div48(self) -> &'a mut W {
        self.variant(PDMDIV_A::DIV48)
    }
    #[doc = "Divide by 64"]
    #[inline(always)]
    pub fn div64(self) -> &'a mut W {
        self.variant(PDMDIV_A::DIV64)
    }
    #[doc = "Divide by 96"]
    #[inline(always)]
    pub fn div96(self) -> &'a mut W {
        self.variant(PDMDIV_A::DIV96)
    }
    #[doc = "Divide by 128"]
    #[inline(always)]
    pub fn div128(self) -> &'a mut W {
        self.variant(PDMDIV_A::DIV128)
    }
}
impl R {
    #[doc = "Bits 0:3 - PDM Clock Divider Value"]
    #[inline(always)]
    pub fn pdmdiv(&self) -> PDMDIV_R {
        PDMDIV_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - PDM Clock Divider Value"]
    #[inline(always)]
    #[must_use]
    pub fn pdmdiv(&mut self) -> PDMDIV_W<0> {
        PDMDIV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMIC Clock\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [divhfclk](index.html) module"]
pub struct DIVHFCLK_SPEC;
impl crate::RegisterSpec for DIVHFCLK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [divhfclk::R](R) reader structure"]
impl crate::Readable for DIVHFCLK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [divhfclk::W](W) writer structure"]
impl crate::Writable for DIVHFCLK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DIVHFCLK to value 0"]
impl crate::Resettable for DIVHFCLK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
