#[doc = "Register `TIMER2_BC12` reader"]
pub struct R(crate::R<TIMER2_TIMER2_BC12_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMER2_TIMER2_BC12_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMER2_TIMER2_BC12_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMER2_TIMER2_BC12_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIMER2_BC12` writer"]
pub struct W(crate::W<TIMER2_TIMER2_BC12_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIMER2_TIMER2_BC12_SPEC>;
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
impl From<crate::W<TIMER2_TIMER2_BC12_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIMER2_TIMER2_BC12_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TVDMSRC_ON` reader - TVDMSRC_ON"]
pub type TVDMSRC_ON_R = crate::FieldReader<u16, TVDMSRC_ON_A>;
#[doc = "TVDMSRC_ON\n\nValue on reset: 40"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum TVDMSRC_ON_A {
    #[doc = "0: 0ms - 40ms"]
    MS_0 = 0,
    #[doc = "1: 0ms - 40ms"]
    MS_1 = 1,
    #[doc = "2: 0ms - 40ms"]
    MS_2 = 2,
    #[doc = "3: 0ms - 40ms"]
    MS_3 = 3,
    #[doc = "4: 0ms - 40ms"]
    MS_4 = 4,
    #[doc = "5: 0ms - 40ms"]
    MS_5 = 5,
    #[doc = "6: 0ms - 40ms"]
    MS_6 = 6,
    #[doc = "7: 0ms - 40ms"]
    MS_7 = 7,
    #[doc = "8: 0ms - 40ms"]
    MS_8 = 8,
    #[doc = "9: 0ms - 40ms"]
    MS_9 = 9,
}
impl From<TVDMSRC_ON_A> for u16 {
    #[inline(always)]
    fn from(variant: TVDMSRC_ON_A) -> Self {
        variant as _
    }
}
impl TVDMSRC_ON_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TVDMSRC_ON_A> {
        match self.bits {
            0 => Some(TVDMSRC_ON_A::MS_0),
            1 => Some(TVDMSRC_ON_A::MS_1),
            2 => Some(TVDMSRC_ON_A::MS_2),
            3 => Some(TVDMSRC_ON_A::MS_3),
            4 => Some(TVDMSRC_ON_A::MS_4),
            5 => Some(TVDMSRC_ON_A::MS_5),
            6 => Some(TVDMSRC_ON_A::MS_6),
            7 => Some(TVDMSRC_ON_A::MS_7),
            8 => Some(TVDMSRC_ON_A::MS_8),
            9 => Some(TVDMSRC_ON_A::MS_9),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `MS_0`"]
    #[inline(always)]
    pub fn is_ms_0(&self) -> bool {
        *self == TVDMSRC_ON_A::MS_0
    }
    #[doc = "Checks if the value of the field is `MS_1`"]
    #[inline(always)]
    pub fn is_ms_1(&self) -> bool {
        *self == TVDMSRC_ON_A::MS_1
    }
    #[doc = "Checks if the value of the field is `MS_2`"]
    #[inline(always)]
    pub fn is_ms_2(&self) -> bool {
        *self == TVDMSRC_ON_A::MS_2
    }
    #[doc = "Checks if the value of the field is `MS_3`"]
    #[inline(always)]
    pub fn is_ms_3(&self) -> bool {
        *self == TVDMSRC_ON_A::MS_3
    }
    #[doc = "Checks if the value of the field is `MS_4`"]
    #[inline(always)]
    pub fn is_ms_4(&self) -> bool {
        *self == TVDMSRC_ON_A::MS_4
    }
    #[doc = "Checks if the value of the field is `MS_5`"]
    #[inline(always)]
    pub fn is_ms_5(&self) -> bool {
        *self == TVDMSRC_ON_A::MS_5
    }
    #[doc = "Checks if the value of the field is `MS_6`"]
    #[inline(always)]
    pub fn is_ms_6(&self) -> bool {
        *self == TVDMSRC_ON_A::MS_6
    }
    #[doc = "Checks if the value of the field is `MS_7`"]
    #[inline(always)]
    pub fn is_ms_7(&self) -> bool {
        *self == TVDMSRC_ON_A::MS_7
    }
    #[doc = "Checks if the value of the field is `MS_8`"]
    #[inline(always)]
    pub fn is_ms_8(&self) -> bool {
        *self == TVDMSRC_ON_A::MS_8
    }
    #[doc = "Checks if the value of the field is `MS_9`"]
    #[inline(always)]
    pub fn is_ms_9(&self) -> bool {
        *self == TVDMSRC_ON_A::MS_9
    }
}
#[doc = "Field `TVDMSRC_ON` writer - TVDMSRC_ON"]
pub type TVDMSRC_ON_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TIMER2_TIMER2_BC12_SPEC, u16, TVDMSRC_ON_A, 10, O>;
impl<'a, const O: u8> TVDMSRC_ON_W<'a, O> {
    #[doc = "0ms - 40ms"]
    #[inline(always)]
    pub fn ms_0(self) -> &'a mut W {
        self.variant(TVDMSRC_ON_A::MS_0)
    }
    #[doc = "0ms - 40ms"]
    #[inline(always)]
    pub fn ms_1(self) -> &'a mut W {
        self.variant(TVDMSRC_ON_A::MS_1)
    }
    #[doc = "0ms - 40ms"]
    #[inline(always)]
    pub fn ms_2(self) -> &'a mut W {
        self.variant(TVDMSRC_ON_A::MS_2)
    }
    #[doc = "0ms - 40ms"]
    #[inline(always)]
    pub fn ms_3(self) -> &'a mut W {
        self.variant(TVDMSRC_ON_A::MS_3)
    }
    #[doc = "0ms - 40ms"]
    #[inline(always)]
    pub fn ms_4(self) -> &'a mut W {
        self.variant(TVDMSRC_ON_A::MS_4)
    }
    #[doc = "0ms - 40ms"]
    #[inline(always)]
    pub fn ms_5(self) -> &'a mut W {
        self.variant(TVDMSRC_ON_A::MS_5)
    }
    #[doc = "0ms - 40ms"]
    #[inline(always)]
    pub fn ms_6(self) -> &'a mut W {
        self.variant(TVDMSRC_ON_A::MS_6)
    }
    #[doc = "0ms - 40ms"]
    #[inline(always)]
    pub fn ms_7(self) -> &'a mut W {
        self.variant(TVDMSRC_ON_A::MS_7)
    }
    #[doc = "0ms - 40ms"]
    #[inline(always)]
    pub fn ms_8(self) -> &'a mut W {
        self.variant(TVDMSRC_ON_A::MS_8)
    }
    #[doc = "0ms - 40ms"]
    #[inline(always)]
    pub fn ms_9(self) -> &'a mut W {
        self.variant(TVDMSRC_ON_A::MS_9)
    }
}
#[doc = "Field `TWAIT_AFTER_PRD` reader - TWAIT_AFTER_PRD"]
pub type TWAIT_AFTER_PRD_R = crate::FieldReader<u16, TWAIT_AFTER_PRD_A>;
#[doc = "TWAIT_AFTER_PRD\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum TWAIT_AFTER_PRD_A {
    #[doc = "1: 1ms - 1023ms"]
    MS_1 = 1,
    #[doc = "2: 1ms - 1023ms"]
    MS_2 = 2,
    #[doc = "3: 1ms - 1023ms"]
    MS_3 = 3,
    #[doc = "4: 1ms - 1023ms"]
    MS_4 = 4,
    #[doc = "5: 1ms - 1023ms"]
    MS_5 = 5,
    #[doc = "6: 1ms - 1023ms"]
    MS_6 = 6,
    #[doc = "7: 1ms - 1023ms"]
    MS_7 = 7,
    #[doc = "8: 1ms - 1023ms"]
    MS_8 = 8,
    #[doc = "9: 1ms - 1023ms"]
    MS_9 = 9,
    #[doc = "10: 1ms - 1023ms"]
    MS_10 = 10,
}
impl From<TWAIT_AFTER_PRD_A> for u16 {
    #[inline(always)]
    fn from(variant: TWAIT_AFTER_PRD_A) -> Self {
        variant as _
    }
}
impl TWAIT_AFTER_PRD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TWAIT_AFTER_PRD_A> {
        match self.bits {
            1 => Some(TWAIT_AFTER_PRD_A::MS_1),
            2 => Some(TWAIT_AFTER_PRD_A::MS_2),
            3 => Some(TWAIT_AFTER_PRD_A::MS_3),
            4 => Some(TWAIT_AFTER_PRD_A::MS_4),
            5 => Some(TWAIT_AFTER_PRD_A::MS_5),
            6 => Some(TWAIT_AFTER_PRD_A::MS_6),
            7 => Some(TWAIT_AFTER_PRD_A::MS_7),
            8 => Some(TWAIT_AFTER_PRD_A::MS_8),
            9 => Some(TWAIT_AFTER_PRD_A::MS_9),
            10 => Some(TWAIT_AFTER_PRD_A::MS_10),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `MS_1`"]
    #[inline(always)]
    pub fn is_ms_1(&self) -> bool {
        *self == TWAIT_AFTER_PRD_A::MS_1
    }
    #[doc = "Checks if the value of the field is `MS_2`"]
    #[inline(always)]
    pub fn is_ms_2(&self) -> bool {
        *self == TWAIT_AFTER_PRD_A::MS_2
    }
    #[doc = "Checks if the value of the field is `MS_3`"]
    #[inline(always)]
    pub fn is_ms_3(&self) -> bool {
        *self == TWAIT_AFTER_PRD_A::MS_3
    }
    #[doc = "Checks if the value of the field is `MS_4`"]
    #[inline(always)]
    pub fn is_ms_4(&self) -> bool {
        *self == TWAIT_AFTER_PRD_A::MS_4
    }
    #[doc = "Checks if the value of the field is `MS_5`"]
    #[inline(always)]
    pub fn is_ms_5(&self) -> bool {
        *self == TWAIT_AFTER_PRD_A::MS_5
    }
    #[doc = "Checks if the value of the field is `MS_6`"]
    #[inline(always)]
    pub fn is_ms_6(&self) -> bool {
        *self == TWAIT_AFTER_PRD_A::MS_6
    }
    #[doc = "Checks if the value of the field is `MS_7`"]
    #[inline(always)]
    pub fn is_ms_7(&self) -> bool {
        *self == TWAIT_AFTER_PRD_A::MS_7
    }
    #[doc = "Checks if the value of the field is `MS_8`"]
    #[inline(always)]
    pub fn is_ms_8(&self) -> bool {
        *self == TWAIT_AFTER_PRD_A::MS_8
    }
    #[doc = "Checks if the value of the field is `MS_9`"]
    #[inline(always)]
    pub fn is_ms_9(&self) -> bool {
        *self == TWAIT_AFTER_PRD_A::MS_9
    }
    #[doc = "Checks if the value of the field is `MS_10`"]
    #[inline(always)]
    pub fn is_ms_10(&self) -> bool {
        *self == TWAIT_AFTER_PRD_A::MS_10
    }
}
#[doc = "Field `TWAIT_AFTER_PRD` writer - TWAIT_AFTER_PRD"]
pub type TWAIT_AFTER_PRD_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TIMER2_TIMER2_BC12_SPEC, u16, TWAIT_AFTER_PRD_A, 10, O>;
impl<'a, const O: u8> TWAIT_AFTER_PRD_W<'a, O> {
    #[doc = "1ms - 1023ms"]
    #[inline(always)]
    pub fn ms_1(self) -> &'a mut W {
        self.variant(TWAIT_AFTER_PRD_A::MS_1)
    }
    #[doc = "1ms - 1023ms"]
    #[inline(always)]
    pub fn ms_2(self) -> &'a mut W {
        self.variant(TWAIT_AFTER_PRD_A::MS_2)
    }
    #[doc = "1ms - 1023ms"]
    #[inline(always)]
    pub fn ms_3(self) -> &'a mut W {
        self.variant(TWAIT_AFTER_PRD_A::MS_3)
    }
    #[doc = "1ms - 1023ms"]
    #[inline(always)]
    pub fn ms_4(self) -> &'a mut W {
        self.variant(TWAIT_AFTER_PRD_A::MS_4)
    }
    #[doc = "1ms - 1023ms"]
    #[inline(always)]
    pub fn ms_5(self) -> &'a mut W {
        self.variant(TWAIT_AFTER_PRD_A::MS_5)
    }
    #[doc = "1ms - 1023ms"]
    #[inline(always)]
    pub fn ms_6(self) -> &'a mut W {
        self.variant(TWAIT_AFTER_PRD_A::MS_6)
    }
    #[doc = "1ms - 1023ms"]
    #[inline(always)]
    pub fn ms_7(self) -> &'a mut W {
        self.variant(TWAIT_AFTER_PRD_A::MS_7)
    }
    #[doc = "1ms - 1023ms"]
    #[inline(always)]
    pub fn ms_8(self) -> &'a mut W {
        self.variant(TWAIT_AFTER_PRD_A::MS_8)
    }
    #[doc = "1ms - 1023ms"]
    #[inline(always)]
    pub fn ms_9(self) -> &'a mut W {
        self.variant(TWAIT_AFTER_PRD_A::MS_9)
    }
    #[doc = "1ms - 1023ms"]
    #[inline(always)]
    pub fn ms_10(self) -> &'a mut W {
        self.variant(TWAIT_AFTER_PRD_A::MS_10)
    }
}
impl R {
    #[doc = "Bits 0:9 - TVDMSRC_ON"]
    #[inline(always)]
    pub fn tvdmsrc_on(&self) -> TVDMSRC_ON_R {
        TVDMSRC_ON_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:25 - TWAIT_AFTER_PRD"]
    #[inline(always)]
    pub fn twait_after_prd(&self) -> TWAIT_AFTER_PRD_R {
        TWAIT_AFTER_PRD_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - TVDMSRC_ON"]
    #[inline(always)]
    #[must_use]
    pub fn tvdmsrc_on(&mut self) -> TVDMSRC_ON_W<0> {
        TVDMSRC_ON_W::new(self)
    }
    #[doc = "Bits 16:25 - TWAIT_AFTER_PRD"]
    #[inline(always)]
    #[must_use]
    pub fn twait_after_prd(&mut self) -> TWAIT_AFTER_PRD_W<16> {
        TWAIT_AFTER_PRD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TIMER2_BC12\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timer2_timer2_bc12](index.html) module"]
pub struct TIMER2_TIMER2_BC12_SPEC;
impl crate::RegisterSpec for TIMER2_TIMER2_BC12_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [timer2_timer2_bc12::R](R) reader structure"]
impl crate::Readable for TIMER2_TIMER2_BC12_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [timer2_timer2_bc12::W](W) writer structure"]
impl crate::Writable for TIMER2_TIMER2_BC12_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TIMER2_BC12 to value 0x0001_0028"]
impl crate::Resettable for TIMER2_TIMER2_BC12_SPEC {
    const RESET_VALUE: Self::Ux = 0x0001_0028;
}
