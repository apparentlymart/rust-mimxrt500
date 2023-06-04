#[doc = "Register `INTENCLR1` writer"]
pub struct W(crate::W<INTENCLR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTENCLR1_SPEC>;
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
impl From<crate::W<INTENCLR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTENCLR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLR32` writer - Writing ones to this register clears corresponding bits in the DMAIntEnSet1."]
pub type CLR32_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENCLR1_SPEC, bool, O>;
#[doc = "Field `CLR33` writer - Writing ones to this register clears corresponding bits in the DMAIntEnSet1."]
pub type CLR33_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENCLR1_SPEC, bool, O>;
#[doc = "Field `CLR34` writer - Writing ones to this register clears corresponding bits in the DMAIntEnSet1."]
pub type CLR34_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENCLR1_SPEC, bool, O>;
#[doc = "Field `CLR35` writer - Writing ones to this register clears corresponding bits in the DMAIntEnSet1."]
pub type CLR35_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENCLR1_SPEC, bool, O>;
#[doc = "Field `CLR36` writer - Writing ones to this register clears corresponding bits in the DMAIntEnSet1."]
pub type CLR36_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENCLR1_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - Writing ones to this register clears corresponding bits in the DMAIntEnSet1."]
    #[inline(always)]
    #[must_use]
    pub fn clr32(&mut self) -> CLR32_W<0> {
        CLR32_W::new(self)
    }
    #[doc = "Bit 1 - Writing ones to this register clears corresponding bits in the DMAIntEnSet1."]
    #[inline(always)]
    #[must_use]
    pub fn clr33(&mut self) -> CLR33_W<1> {
        CLR33_W::new(self)
    }
    #[doc = "Bit 2 - Writing ones to this register clears corresponding bits in the DMAIntEnSet1."]
    #[inline(always)]
    #[must_use]
    pub fn clr34(&mut self) -> CLR34_W<2> {
        CLR34_W::new(self)
    }
    #[doc = "Bit 3 - Writing ones to this register clears corresponding bits in the DMAIntEnSet1."]
    #[inline(always)]
    #[must_use]
    pub fn clr35(&mut self) -> CLR35_W<3> {
        CLR35_W::new(self)
    }
    #[doc = "Bit 4 - Writing ones to this register clears corresponding bits in the DMAIntEnSet1."]
    #[inline(always)]
    #[must_use]
    pub fn clr36(&mut self) -> CLR36_W<4> {
        CLR36_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Enable Clear for all DMA channels\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intenclr1](index.html) module"]
pub struct INTENCLR1_SPEC;
impl crate::RegisterSpec for INTENCLR1_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [intenclr1::W](W) writer structure"]
impl crate::Writable for INTENCLR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTENCLR1 to value 0"]
impl crate::Resettable for INTENCLR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
