#[doc = "Register `INTENCLR` writer"]
pub struct W(crate::W<INTENCLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTENCLR_SPEC>;
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
impl From<crate::W<INTENCLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTENCLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TXIDLECLR` writer - Transmit Idle Clear"]
pub type TXIDLECLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENCLR_SPEC, bool, O>;
#[doc = "Field `DELTACTSCLR` writer - Delta CTS Clear"]
pub type DELTACTSCLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENCLR_SPEC, bool, O>;
#[doc = "Field `TXDISCLR` writer - Transmit Disable Clear"]
pub type TXDISCLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENCLR_SPEC, bool, O>;
#[doc = "Field `DELTARXBRKCLR` writer - Delta Receive Break Clear"]
pub type DELTARXBRKCLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENCLR_SPEC, bool, O>;
#[doc = "Field `STARTCLR` writer - Start Clear"]
pub type STARTCLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENCLR_SPEC, bool, O>;
#[doc = "Field `FRAMERRCLR` writer - Frame Error Clear"]
pub type FRAMERRCLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENCLR_SPEC, bool, O>;
#[doc = "Field `PARITYERRCLR` writer - Parity Error Clear"]
pub type PARITYERRCLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENCLR_SPEC, bool, O>;
#[doc = "Field `RXNOISECLR` writer - Receive Noise Clear"]
pub type RXNOISECLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENCLR_SPEC, bool, O>;
#[doc = "Field `ABERRCLR` writer - Auto Baud Error Clear"]
pub type ABERRCLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENCLR_SPEC, bool, O>;
impl W {
    #[doc = "Bit 3 - Transmit Idle Clear"]
    #[inline(always)]
    #[must_use]
    pub fn txidleclr(&mut self) -> TXIDLECLR_W<3> {
        TXIDLECLR_W::new(self)
    }
    #[doc = "Bit 5 - Delta CTS Clear"]
    #[inline(always)]
    #[must_use]
    pub fn deltactsclr(&mut self) -> DELTACTSCLR_W<5> {
        DELTACTSCLR_W::new(self)
    }
    #[doc = "Bit 6 - Transmit Disable Clear"]
    #[inline(always)]
    #[must_use]
    pub fn txdisclr(&mut self) -> TXDISCLR_W<6> {
        TXDISCLR_W::new(self)
    }
    #[doc = "Bit 11 - Delta Receive Break Clear"]
    #[inline(always)]
    #[must_use]
    pub fn deltarxbrkclr(&mut self) -> DELTARXBRKCLR_W<11> {
        DELTARXBRKCLR_W::new(self)
    }
    #[doc = "Bit 12 - Start Clear"]
    #[inline(always)]
    #[must_use]
    pub fn startclr(&mut self) -> STARTCLR_W<12> {
        STARTCLR_W::new(self)
    }
    #[doc = "Bit 13 - Frame Error Clear"]
    #[inline(always)]
    #[must_use]
    pub fn framerrclr(&mut self) -> FRAMERRCLR_W<13> {
        FRAMERRCLR_W::new(self)
    }
    #[doc = "Bit 14 - Parity Error Clear"]
    #[inline(always)]
    #[must_use]
    pub fn parityerrclr(&mut self) -> PARITYERRCLR_W<14> {
        PARITYERRCLR_W::new(self)
    }
    #[doc = "Bit 15 - Receive Noise Clear"]
    #[inline(always)]
    #[must_use]
    pub fn rxnoiseclr(&mut self) -> RXNOISECLR_W<15> {
        RXNOISECLR_W::new(self)
    }
    #[doc = "Bit 16 - Auto Baud Error Clear"]
    #[inline(always)]
    #[must_use]
    pub fn aberrclr(&mut self) -> ABERRCLR_W<16> {
        ABERRCLR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Enable Clear\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intenclr](index.html) module"]
pub struct INTENCLR_SPEC;
impl crate::RegisterSpec for INTENCLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [intenclr::W](W) writer structure"]
impl crate::Writable for INTENCLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTENCLR to value 0"]
impl crate::Resettable for INTENCLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
