#[doc = "Register `MWDATAH` writer"]
pub struct W(crate::W<MWDATAH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MWDATAH_SPEC>;
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
impl From<crate::W<MWDATAH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MWDATAH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATA0` writer - Data byte 0"]
pub type DATA0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MWDATAH_SPEC, u8, u8, 8, O>;
#[doc = "Field `DATA1` writer - Data byte 1"]
pub type DATA1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MWDATAH_SPEC, u8, u8, 8, O>;
#[doc = "Field `END` writer - End of message"]
pub type END_W<'a, const O: u8> = crate::BitWriter<'a, u32, MWDATAH_SPEC, bool, O>;
impl W {
    #[doc = "Bits 0:7 - Data byte 0"]
    #[inline(always)]
    #[must_use]
    pub fn data0(&mut self) -> DATA0_W<0> {
        DATA0_W::new(self)
    }
    #[doc = "Bits 8:15 - Data byte 1"]
    #[inline(always)]
    #[must_use]
    pub fn data1(&mut self) -> DATA1_W<8> {
        DATA1_W::new(self)
    }
    #[doc = "Bit 16 - End of message"]
    #[inline(always)]
    #[must_use]
    pub fn end(&mut self) -> END_W<16> {
        END_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Master Write Data Half-word Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mwdatah](index.html) module"]
pub struct MWDATAH_SPEC;
impl crate::RegisterSpec for MWDATAH_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [mwdatah::W](W) writer structure"]
impl crate::Writable for MWDATAH_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MWDATAH to value 0"]
impl crate::Resettable for MWDATAH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
