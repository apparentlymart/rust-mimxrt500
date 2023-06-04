#[doc = "Register `CTRL` reader"]
pub struct R(crate::R<CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL` writer"]
pub struct W(crate::W<CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL_SPEC>;
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
impl From<crate::W<CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DELAYVAL` reader - Tick interval"]
pub type DELAYVAL_R = crate::FieldReader<u32, u32>;
#[doc = "Field `DELAYVAL` writer - Tick interval"]
pub type DELAYVAL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRL_SPEC, u32, u32, 31, O>;
#[doc = "Field `REPEAT` reader - Repeat delay"]
pub type REPEAT_R = crate::BitReader<REPEAT_A>;
#[doc = "Repeat delay\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REPEAT_A {
    #[doc = "0: One-time delay"]
    DELAYONCE = 0,
    #[doc = "1: Delay repeats continuously"]
    DELAYREPEATS = 1,
}
impl From<REPEAT_A> for bool {
    #[inline(always)]
    fn from(variant: REPEAT_A) -> Self {
        variant as u8 != 0
    }
}
impl REPEAT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REPEAT_A {
        match self.bits {
            false => REPEAT_A::DELAYONCE,
            true => REPEAT_A::DELAYREPEATS,
        }
    }
    #[doc = "Checks if the value of the field is `DELAYONCE`"]
    #[inline(always)]
    pub fn is_delayonce(&self) -> bool {
        *self == REPEAT_A::DELAYONCE
    }
    #[doc = "Checks if the value of the field is `DELAYREPEATS`"]
    #[inline(always)]
    pub fn is_delayrepeats(&self) -> bool {
        *self == REPEAT_A::DELAYREPEATS
    }
}
#[doc = "Field `REPEAT` writer - Repeat delay"]
pub type REPEAT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, REPEAT_A, O>;
impl<'a, const O: u8> REPEAT_W<'a, O> {
    #[doc = "One-time delay"]
    #[inline(always)]
    pub fn delayonce(self) -> &'a mut W {
        self.variant(REPEAT_A::DELAYONCE)
    }
    #[doc = "Delay repeats continuously"]
    #[inline(always)]
    pub fn delayrepeats(self) -> &'a mut W {
        self.variant(REPEAT_A::DELAYREPEATS)
    }
}
impl R {
    #[doc = "Bits 0:30 - Tick interval"]
    #[inline(always)]
    pub fn delayval(&self) -> DELAYVAL_R {
        DELAYVAL_R::new(self.bits & 0x7fff_ffff)
    }
    #[doc = "Bit 31 - Repeat delay"]
    #[inline(always)]
    pub fn repeat(&self) -> REPEAT_R {
        REPEAT_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:30 - Tick interval"]
    #[inline(always)]
    #[must_use]
    pub fn delayval(&mut self) -> DELAYVAL_W<0> {
        DELAYVAL_W::new(self)
    }
    #[doc = "Bit 31 - Repeat delay"]
    #[inline(always)]
    #[must_use]
    pub fn repeat(&mut self) -> REPEAT_W<31> {
        REPEAT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](index.html) module"]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl::R](R) reader structure"]
impl crate::Readable for CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl::W](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
