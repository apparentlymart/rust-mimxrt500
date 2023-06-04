#[doc = "Register `INTSTAT` reader"]
pub struct R(crate::R<INTSTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTSTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTSTAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTSTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTSTAT` writer"]
pub struct W(crate::W<INTSTAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTSTAT_SPEC>;
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
impl From<crate::W<INTSTAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTSTAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `READY` reader - Ready"]
pub type READY_R = crate::BitReader<bool>;
#[doc = "Field `READY` writer - Ready"]
pub type READY_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, INTSTAT_SPEC, bool, O>;
#[doc = "Field `SUCCESS` reader - Success"]
pub type SUCCESS_R = crate::BitReader<bool>;
#[doc = "Field `SUCCESS` writer - Success"]
pub type SUCCESS_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTSTAT_SPEC, bool, O>;
#[doc = "Field `ERROR` reader - Error"]
pub type ERROR_R = crate::BitReader<bool>;
#[doc = "Field `ERROR` writer - Error"]
pub type ERROR_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTSTAT_SPEC, bool, O>;
#[doc = "Field `KEYINREQ` reader - Key In Request"]
pub type KEYINREQ_R = crate::BitReader<bool>;
#[doc = "Field `KEYINREQ` writer - Key In Request"]
pub type KEYINREQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTSTAT_SPEC, bool, O>;
#[doc = "Field `KEYOUTAVAIL` reader - Key Out Available"]
pub type KEYOUTAVAIL_R = crate::BitReader<bool>;
#[doc = "Field `KEYOUTAVAIL` writer - Key Out Available"]
pub type KEYOUTAVAIL_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTSTAT_SPEC, bool, O>;
#[doc = "Field `CODEINREQ` reader - Code In Request"]
pub type CODEINREQ_R = crate::BitReader<bool>;
#[doc = "Field `CODEINREQ` writer - Code In Request"]
pub type CODEINREQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTSTAT_SPEC, bool, O>;
#[doc = "Field `CODEOUTAVAIL` reader - Code Out Available"]
pub type CODEOUTAVAIL_R = crate::BitReader<bool>;
#[doc = "Field `CODEOUTAVAIL` writer - Code Out Available"]
pub type CODEOUTAVAIL_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTSTAT_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Ready"]
    #[inline(always)]
    pub fn ready(&self) -> READY_R {
        READY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Success"]
    #[inline(always)]
    pub fn success(&self) -> SUCCESS_R {
        SUCCESS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Error"]
    #[inline(always)]
    pub fn error(&self) -> ERROR_R {
        ERROR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Key In Request"]
    #[inline(always)]
    pub fn keyinreq(&self) -> KEYINREQ_R {
        KEYINREQ_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Key Out Available"]
    #[inline(always)]
    pub fn keyoutavail(&self) -> KEYOUTAVAIL_R {
        KEYOUTAVAIL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Code In Request"]
    #[inline(always)]
    pub fn codeinreq(&self) -> CODEINREQ_R {
        CODEINREQ_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Code Out Available"]
    #[inline(always)]
    pub fn codeoutavail(&self) -> CODEOUTAVAIL_R {
        CODEOUTAVAIL_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Ready"]
    #[inline(always)]
    #[must_use]
    pub fn ready(&mut self) -> READY_W<0> {
        READY_W::new(self)
    }
    #[doc = "Bit 1 - Success"]
    #[inline(always)]
    #[must_use]
    pub fn success(&mut self) -> SUCCESS_W<1> {
        SUCCESS_W::new(self)
    }
    #[doc = "Bit 2 - Error"]
    #[inline(always)]
    #[must_use]
    pub fn error(&mut self) -> ERROR_W<2> {
        ERROR_W::new(self)
    }
    #[doc = "Bit 4 - Key In Request"]
    #[inline(always)]
    #[must_use]
    pub fn keyinreq(&mut self) -> KEYINREQ_W<4> {
        KEYINREQ_W::new(self)
    }
    #[doc = "Bit 5 - Key Out Available"]
    #[inline(always)]
    #[must_use]
    pub fn keyoutavail(&mut self) -> KEYOUTAVAIL_W<5> {
        KEYOUTAVAIL_W::new(self)
    }
    #[doc = "Bit 6 - Code In Request"]
    #[inline(always)]
    #[must_use]
    pub fn codeinreq(&mut self) -> CODEINREQ_W<6> {
        CODEINREQ_W::new(self)
    }
    #[doc = "Bit 7 - Code Out Available"]
    #[inline(always)]
    #[must_use]
    pub fn codeoutavail(&mut self) -> CODEOUTAVAIL_W<7> {
        CODEOUTAVAIL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PUF Interrupt Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intstat](index.html) module"]
pub struct INTSTAT_SPEC;
impl crate::RegisterSpec for INTSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intstat::R](R) reader structure"]
impl crate::Readable for INTSTAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intstat::W](W) writer structure"]
impl crate::Writable for INTSTAT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x01;
}
#[doc = "`reset()` method sets INTSTAT to value 0"]
impl crate::Resettable for INTSTAT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
