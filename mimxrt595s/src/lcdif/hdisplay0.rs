#[doc = "Register `HDisplay0` reader"]
pub struct R(crate::R<HDISPLAY0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HDISPLAY0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HDISPLAY0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HDISPLAY0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HDisplay0` writer"]
pub struct W(crate::W<HDISPLAY0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HDISPLAY0_SPEC>;
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
impl From<crate::W<HDISPLAY0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HDISPLAY0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DISPLAY_END` reader - Number of visible horizontal pixels."]
pub type DISPLAY_END_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DISPLAY_END` writer - Number of visible horizontal pixels."]
pub type DISPLAY_END_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, HDISPLAY0_SPEC, u16, u16, 13, O>;
#[doc = "Field `TOTAL` reader - Total number of horizontal pixels."]
pub type TOTAL_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TOTAL` writer - Total number of horizontal pixels."]
pub type TOTAL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HDISPLAY0_SPEC, u16, u16, 13, O>;
impl R {
    #[doc = "Bits 0:12 - Number of visible horizontal pixels."]
    #[inline(always)]
    pub fn display_end(&self) -> DISPLAY_END_R {
        DISPLAY_END_R::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bits 16:28 - Total number of horizontal pixels."]
    #[inline(always)]
    pub fn total(&self) -> TOTAL_R {
        TOTAL_R::new(((self.bits >> 16) & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:12 - Number of visible horizontal pixels."]
    #[inline(always)]
    #[must_use]
    pub fn display_end(&mut self) -> DISPLAY_END_W<0> {
        DISPLAY_END_W::new(self)
    }
    #[doc = "Bits 16:28 - Total number of horizontal pixels."]
    #[inline(always)]
    #[must_use]
    pub fn total(&mut self) -> TOTAL_W<16> {
        TOTAL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Horizontal Total and Display End Counters\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hdisplay0](index.html) module"]
pub struct HDISPLAY0_SPEC;
impl crate::RegisterSpec for HDISPLAY0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hdisplay0::R](R) reader structure"]
impl crate::Readable for HDISPLAY0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hdisplay0::W](W) writer structure"]
impl crate::Writable for HDISPLAY0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HDisplay0 to value 0"]
impl crate::Resettable for HDISPLAY0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
