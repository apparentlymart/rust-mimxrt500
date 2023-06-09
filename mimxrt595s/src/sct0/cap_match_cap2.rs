#[doc = "Register `CAP2` reader"]
pub struct R(crate::R<CAP_MATCH_CAP2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CAP_MATCH_CAP2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CAP_MATCH_CAP2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CAP_MATCH_CAP2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CAP2` writer"]
pub struct W(crate::W<CAP_MATCH_CAP2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CAP_MATCH_CAP2_SPEC>;
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
impl From<crate::W<CAP_MATCH_CAP2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CAP_MATCH_CAP2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CAPn_L` reader - Capture Low"]
pub type CAPN_L_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CAPn_L` writer - Capture Low"]
pub type CAPN_L_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CAP_MATCH_CAP2_SPEC, u16, u16, 16, O>;
#[doc = "Field `CAPn_H` reader - Capture High"]
pub type CAPN_H_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CAPn_H` writer - Capture High"]
pub type CAPN_H_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CAP_MATCH_CAP2_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Capture Low"]
    #[inline(always)]
    pub fn capn_l(&self) -> CAPN_L_R {
        CAPN_L_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Capture High"]
    #[inline(always)]
    pub fn capn_h(&self) -> CAPN_H_R {
        CAPN_H_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Capture Low"]
    #[inline(always)]
    #[must_use]
    pub fn capn_l(&mut self) -> CAPN_L_W<0> {
        CAPN_L_W::new(self)
    }
    #[doc = "Bits 16:31 - Capture High"]
    #[inline(always)]
    #[must_use]
    pub fn capn_h(&mut self) -> CAPN_H_W<16> {
        CAPN_H_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Capture Value\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cap_match_cap2](index.html) module"]
pub struct CAP_MATCH_CAP2_SPEC;
impl crate::RegisterSpec for CAP_MATCH_CAP2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cap_match_cap2::R](R) reader structure"]
impl crate::Readable for CAP_MATCH_CAP2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cap_match_cap2::W](W) writer structure"]
impl crate::Writable for CAP_MATCH_CAP2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CAP2 to value 0"]
impl crate::Resettable for CAP_MATCH_CAP2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
