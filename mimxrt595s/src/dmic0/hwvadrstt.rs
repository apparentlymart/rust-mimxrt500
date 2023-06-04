#[doc = "Register `HWVADRSTT` reader"]
pub struct R(crate::R<HWVADRSTT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HWVADRSTT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HWVADRSTT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HWVADRSTT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HWVADRSTT` writer"]
pub struct W(crate::W<HWVADRSTT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HWVADRSTT_SPEC>;
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
impl From<crate::W<HWVADRSTT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HWVADRSTT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RSST` reader - Reset HWVAD"]
pub type RSST_R = crate::BitReader<bool>;
#[doc = "Field `RSST` writer - Reset HWVAD"]
pub type RSST_W<'a, const O: u8> = crate::BitWriter<'a, u32, HWVADRSTT_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Reset HWVAD"]
    #[inline(always)]
    pub fn rsst(&self) -> RSST_R {
        RSST_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Reset HWVAD"]
    #[inline(always)]
    #[must_use]
    pub fn rsst(&mut self) -> RSST_W<0> {
        RSST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "HWVAD Filter Reset\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hwvadrstt](index.html) module"]
pub struct HWVADRSTT_SPEC;
impl crate::RegisterSpec for HWVADRSTT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hwvadrstt::R](R) reader structure"]
impl crate::Readable for HWVADRSTT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hwvadrstt::W](W) writer structure"]
impl crate::Writable for HWVADRSTT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HWVADRSTT to value 0"]
impl crate::Resettable for HWVADRSTT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
