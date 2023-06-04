#[doc = "Register `SWDATAB` writer"]
pub struct W(crate::W<SWDATAB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SWDATAB_SPEC>;
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
impl From<crate::W<SWDATAB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SWDATAB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATA` writer - The data byte to send to the master"]
pub type DATA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SWDATAB_SPEC, u8, u8, 8, O>;
#[doc = "Field `END` writer - End"]
pub type END_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWDATAB_SPEC, bool, O>;
#[doc = "Field `END_ALSO` writer - End also"]
pub type END_ALSO_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWDATAB_SPEC, bool, O>;
impl W {
    #[doc = "Bits 0:7 - The data byte to send to the master"]
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DATA_W<0> {
        DATA_W::new(self)
    }
    #[doc = "Bit 8 - End"]
    #[inline(always)]
    #[must_use]
    pub fn end(&mut self) -> END_W<8> {
        END_W::new(self)
    }
    #[doc = "Bit 16 - End also"]
    #[inline(always)]
    #[must_use]
    pub fn end_also(&mut self) -> END_ALSO_W<16> {
        END_ALSO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Slave Write Data Byte Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [swdatab](index.html) module"]
pub struct SWDATAB_SPEC;
impl crate::RegisterSpec for SWDATAB_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [swdatab::W](W) writer structure"]
impl crate::Writable for SWDATAB_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SWDATAB to value 0"]
impl crate::Resettable for SWDATAB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
