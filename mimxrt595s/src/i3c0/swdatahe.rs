#[doc = "Register `SWDATAHE` writer"]
pub struct W(crate::W<SWDATAHE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SWDATAHE_SPEC>;
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
impl From<crate::W<SWDATAHE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SWDATAHE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATA0` writer - The 1st byte to send to the master"]
pub type DATA0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SWDATAHE_SPEC, u8, u8, 8, O>;
#[doc = "Field `DATA1` writer - The 2nd byte to send to the master"]
pub type DATA1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SWDATAHE_SPEC, u8, u8, 8, O>;
impl W {
    #[doc = "Bits 0:7 - The 1st byte to send to the master"]
    #[inline(always)]
    #[must_use]
    pub fn data0(&mut self) -> DATA0_W<0> {
        DATA0_W::new(self)
    }
    #[doc = "Bits 8:15 - The 2nd byte to send to the master"]
    #[inline(always)]
    #[must_use]
    pub fn data1(&mut self) -> DATA1_W<8> {
        DATA1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Slave Write Data Half-word End Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [swdatahe](index.html) module"]
pub struct SWDATAHE_SPEC;
impl crate::RegisterSpec for SWDATAHE_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [swdatahe::W](W) writer structure"]
impl crate::Writable for SWDATAHE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SWDATAHE to value 0"]
impl crate::Resettable for SWDATAHE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
