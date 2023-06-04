#[doc = "Register `GLOBAL_SYNC_EN` reader"]
pub struct R(crate::R<GLOBAL_SYNC_EN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GLOBAL_SYNC_EN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GLOBAL_SYNC_EN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GLOBAL_SYNC_EN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GLOBAL_SYNC_EN` writer"]
pub struct W(crate::W<GLOBAL_SYNC_EN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GLOBAL_SYNC_EN_SPEC>;
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
impl From<crate::W<GLOBAL_SYNC_EN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GLOBAL_SYNC_EN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CH_SYNC_EN` reader - Channel synch enable"]
pub type CH_SYNC_EN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CH_SYNC_EN` writer - Channel synch enable"]
pub type CH_SYNC_EN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GLOBAL_SYNC_EN_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Channel synch enable"]
    #[inline(always)]
    pub fn ch_sync_en(&self) -> CH_SYNC_EN_R {
        CH_SYNC_EN_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Channel synch enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch_sync_en(&mut self) -> CH_SYNC_EN_W<0> {
        CH_SYNC_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Global Channel Synchronization Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [global_sync_en](index.html) module"]
pub struct GLOBAL_SYNC_EN_SPEC;
impl crate::RegisterSpec for GLOBAL_SYNC_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [global_sync_en::R](R) reader structure"]
impl crate::Readable for GLOBAL_SYNC_EN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [global_sync_en::W](W) writer structure"]
impl crate::Writable for GLOBAL_SYNC_EN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GLOBAL_SYNC_EN to value 0"]
impl crate::Resettable for GLOBAL_SYNC_EN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
