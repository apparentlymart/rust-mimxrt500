#[doc = "Register `MINTCLR` writer"]
pub struct W(crate::W<MINTCLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MINTCLR_SPEC>;
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
impl From<crate::W<MINTCLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MINTCLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SLVSTART` writer - SLVSTART interrupt enable clear"]
pub type SLVSTART_W<'a, const O: u8> = crate::BitWriter<'a, u32, MINTCLR_SPEC, bool, O>;
#[doc = "Field `MCTRLDONE` writer - MCTRLDONE interrupt enable clear"]
pub type MCTRLDONE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MINTCLR_SPEC, bool, O>;
#[doc = "Field `COMPLETE` writer - COMPLETE interrupt enable clear"]
pub type COMPLETE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MINTCLR_SPEC, bool, O>;
#[doc = "Field `RXPEND` writer - RXPEND interrupt enable clear"]
pub type RXPEND_W<'a, const O: u8> = crate::BitWriter<'a, u32, MINTCLR_SPEC, bool, O>;
#[doc = "Field `TXNOTFULL` writer - TXNOTFULL interrupt enable clear"]
pub type TXNOTFULL_W<'a, const O: u8> = crate::BitWriter<'a, u32, MINTCLR_SPEC, bool, O>;
#[doc = "Field `IBIWON` writer - IBIWON interrupt enable clear"]
pub type IBIWON_W<'a, const O: u8> = crate::BitWriter<'a, u32, MINTCLR_SPEC, bool, O>;
#[doc = "Field `ERRWARN` writer - ERRWARN interrupt enable clear"]
pub type ERRWARN_W<'a, const O: u8> = crate::BitWriter<'a, u32, MINTCLR_SPEC, bool, O>;
#[doc = "Field `NOWMASTER` writer - NOWMASTER interrupt enable clear"]
pub type NOWMASTER_W<'a, const O: u8> = crate::BitWriter<'a, u32, MINTCLR_SPEC, bool, O>;
impl W {
    #[doc = "Bit 8 - SLVSTART interrupt enable clear"]
    #[inline(always)]
    #[must_use]
    pub fn slvstart(&mut self) -> SLVSTART_W<8> {
        SLVSTART_W::new(self)
    }
    #[doc = "Bit 9 - MCTRLDONE interrupt enable clear"]
    #[inline(always)]
    #[must_use]
    pub fn mctrldone(&mut self) -> MCTRLDONE_W<9> {
        MCTRLDONE_W::new(self)
    }
    #[doc = "Bit 10 - COMPLETE interrupt enable clear"]
    #[inline(always)]
    #[must_use]
    pub fn complete(&mut self) -> COMPLETE_W<10> {
        COMPLETE_W::new(self)
    }
    #[doc = "Bit 11 - RXPEND interrupt enable clear"]
    #[inline(always)]
    #[must_use]
    pub fn rxpend(&mut self) -> RXPEND_W<11> {
        RXPEND_W::new(self)
    }
    #[doc = "Bit 12 - TXNOTFULL interrupt enable clear"]
    #[inline(always)]
    #[must_use]
    pub fn txnotfull(&mut self) -> TXNOTFULL_W<12> {
        TXNOTFULL_W::new(self)
    }
    #[doc = "Bit 13 - IBIWON interrupt enable clear"]
    #[inline(always)]
    #[must_use]
    pub fn ibiwon(&mut self) -> IBIWON_W<13> {
        IBIWON_W::new(self)
    }
    #[doc = "Bit 15 - ERRWARN interrupt enable clear"]
    #[inline(always)]
    #[must_use]
    pub fn errwarn(&mut self) -> ERRWARN_W<15> {
        ERRWARN_W::new(self)
    }
    #[doc = "Bit 19 - NOWMASTER interrupt enable clear"]
    #[inline(always)]
    #[must_use]
    pub fn nowmaster(&mut self) -> NOWMASTER_W<19> {
        NOWMASTER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Master Interrupt Clear Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mintclr](index.html) module"]
pub struct MINTCLR_SPEC;
impl crate::RegisterSpec for MINTCLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [mintclr::W](W) writer structure"]
impl crate::Writable for MINTCLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MINTCLR to value 0"]
impl crate::Resettable for MINTCLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
