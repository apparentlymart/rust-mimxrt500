#[doc = "Register `NSGPIO_PSYNC` reader"]
pub struct R(crate::R<NSGPIO_PSYNC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NSGPIO_PSYNC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NSGPIO_PSYNC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NSGPIO_PSYNC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `NSGPIO_PSYNC` writer"]
pub struct W(crate::W<NSGPIO_PSYNC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NSGPIO_PSYNC_SPEC>;
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
impl From<crate::W<NSGPIO_PSYNC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NSGPIO_PSYNC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PSYNC` reader - Synchronization Stage Setting:"]
pub type PSYNC_R = crate::BitReader<bool>;
#[doc = "Field `PSYNC` writer - Synchronization Stage Setting:"]
pub type PSYNC_W<'a, const O: u8> = crate::BitWriter<'a, u32, NSGPIO_PSYNC_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Synchronization Stage Setting:"]
    #[inline(always)]
    pub fn psync(&self) -> PSYNC_R {
        PSYNC_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Synchronization Stage Setting:"]
    #[inline(always)]
    #[must_use]
    pub fn psync(&mut self) -> PSYNC_W<0> {
        PSYNC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Non-secure GPIO PSYNC\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nsgpio_psync](index.html) module"]
pub struct NSGPIO_PSYNC_SPEC;
impl crate::RegisterSpec for NSGPIO_PSYNC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [nsgpio_psync::R](R) reader structure"]
impl crate::Readable for NSGPIO_PSYNC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [nsgpio_psync::W](W) writer structure"]
impl crate::Writable for NSGPIO_PSYNC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets NSGPIO_PSYNC to value 0"]
impl crate::Resettable for NSGPIO_PSYNC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
