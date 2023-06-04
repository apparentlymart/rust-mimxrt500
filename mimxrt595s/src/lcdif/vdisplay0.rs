#[doc = "Register `VDisplay0` reader"]
pub struct R(crate::R<VDISPLAY0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VDISPLAY0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VDISPLAY0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VDISPLAY0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `VDisplay0` writer"]
pub struct W(crate::W<VDISPLAY0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<VDISPLAY0_SPEC>;
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
impl From<crate::W<VDISPLAY0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<VDISPLAY0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DISPLAY_END` reader - Number of visible vertical lines."]
pub type DISPLAY_END_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DISPLAY_END` writer - Number of visible vertical lines."]
pub type DISPLAY_END_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, VDISPLAY0_SPEC, u16, u16, 12, O>;
#[doc = "Field `TOTAL` reader - Total number of vertical lines."]
pub type TOTAL_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TOTAL` writer - Total number of vertical lines."]
pub type TOTAL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, VDISPLAY0_SPEC, u16, u16, 12, O>;
impl R {
    #[doc = "Bits 0:11 - Number of visible vertical lines."]
    #[inline(always)]
    pub fn display_end(&self) -> DISPLAY_END_R {
        DISPLAY_END_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - Total number of vertical lines."]
    #[inline(always)]
    pub fn total(&self) -> TOTAL_R {
        TOTAL_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Number of visible vertical lines."]
    #[inline(always)]
    #[must_use]
    pub fn display_end(&mut self) -> DISPLAY_END_W<0> {
        DISPLAY_END_W::new(self)
    }
    #[doc = "Bits 16:27 - Total number of vertical lines."]
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
#[doc = "Vertical Total and Display End Counters\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vdisplay0](index.html) module"]
pub struct VDISPLAY0_SPEC;
impl crate::RegisterSpec for VDISPLAY0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [vdisplay0::R](R) reader structure"]
impl crate::Readable for VDISPLAY0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [vdisplay0::W](W) writer structure"]
impl crate::Writable for VDISPLAY0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets VDisplay0 to value 0"]
impl crate::Resettable for VDISPLAY0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
