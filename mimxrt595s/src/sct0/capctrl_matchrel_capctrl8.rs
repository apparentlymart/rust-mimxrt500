#[doc = "Register `CAPCTRL8` reader"]
pub struct R(crate::R<CAPCTRL_MATCHREL_CAPCTRL8_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CAPCTRL_MATCHREL_CAPCTRL8_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CAPCTRL_MATCHREL_CAPCTRL8_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CAPCTRL_MATCHREL_CAPCTRL8_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CAPCTRL8` writer"]
pub struct W(crate::W<CAPCTRL_MATCHREL_CAPCTRL8_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CAPCTRL_MATCHREL_CAPCTRL8_SPEC>;
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
impl From<crate::W<CAPCTRL_MATCHREL_CAPCTRL8_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CAPCTRL_MATCHREL_CAPCTRL8_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CAPCONn_L` reader - Capture Control Low"]
pub type CAPCONN_L_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CAPCONn_L` writer - Capture Control Low"]
pub type CAPCONN_L_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CAPCTRL_MATCHREL_CAPCTRL8_SPEC, u16, u16, 16, O>;
#[doc = "Field `CAPCONn_H` reader - Capture Control High"]
pub type CAPCONN_H_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CAPCONn_H` writer - Capture Control High"]
pub type CAPCONN_H_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CAPCTRL_MATCHREL_CAPCTRL8_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Capture Control Low"]
    #[inline(always)]
    pub fn capconn_l(&self) -> CAPCONN_L_R {
        CAPCONN_L_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Capture Control High"]
    #[inline(always)]
    pub fn capconn_h(&self) -> CAPCONN_H_R {
        CAPCONN_H_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Capture Control Low"]
    #[inline(always)]
    #[must_use]
    pub fn capconn_l(&mut self) -> CAPCONN_L_W<0> {
        CAPCONN_L_W::new(self)
    }
    #[doc = "Bits 16:31 - Capture Control High"]
    #[inline(always)]
    #[must_use]
    pub fn capconn_h(&mut self) -> CAPCONN_H_W<16> {
        CAPCONN_H_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Capture Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [capctrl_matchrel_capctrl8](index.html) module"]
pub struct CAPCTRL_MATCHREL_CAPCTRL8_SPEC;
impl crate::RegisterSpec for CAPCTRL_MATCHREL_CAPCTRL8_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [capctrl_matchrel_capctrl8::R](R) reader structure"]
impl crate::Readable for CAPCTRL_MATCHREL_CAPCTRL8_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [capctrl_matchrel_capctrl8::W](W) writer structure"]
impl crate::Writable for CAPCTRL_MATCHREL_CAPCTRL8_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CAPCTRL8 to value 0"]
impl crate::Resettable for CAPCTRL_MATCHREL_CAPCTRL8_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}