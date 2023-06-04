#[doc = "Register `ANACTRL` reader"]
pub struct R(crate::R<ANACTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ANACTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ANACTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ANACTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ANACTRL` writer"]
pub struct W(crate::W<ANACTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ANACTRL_SPEC>;
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
impl From<crate::W<ANACTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ANACTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DEV_PULLDOWN` reader - Device Pull-down"]
pub type DEV_PULLDOWN_R = crate::BitReader<bool>;
#[doc = "Field `DEV_PULLDOWN` writer - Device Pull-down"]
pub type DEV_PULLDOWN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ANACTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 10 - Device Pull-down"]
    #[inline(always)]
    pub fn dev_pulldown(&self) -> DEV_PULLDOWN_R {
        DEV_PULLDOWN_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 10 - Device Pull-down"]
    #[inline(always)]
    #[must_use]
    pub fn dev_pulldown(&mut self) -> DEV_PULLDOWN_W<10> {
        DEV_PULLDOWN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Analog Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [anactrl](index.html) module"]
pub struct ANACTRL_SPEC;
impl crate::RegisterSpec for ANACTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [anactrl::R](R) reader structure"]
impl crate::Readable for ANACTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [anactrl::W](W) writer structure"]
impl crate::Writable for ANACTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ANACTRL to value 0x0400"]
impl crate::Resettable for ANACTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x0400;
}
