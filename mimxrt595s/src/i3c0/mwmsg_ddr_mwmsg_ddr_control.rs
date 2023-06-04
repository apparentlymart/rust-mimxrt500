#[doc = "Register `MWMSG_DDR_CONTROL` writer"]
pub struct W(crate::W<MWMSG_DDR_MWMSG_DDR_CONTROL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MWMSG_DDR_MWMSG_DDR_CONTROL_SPEC>;
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
impl From<crate::W<MWMSG_DDR_MWMSG_DDR_CONTROL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MWMSG_DDR_MWMSG_DDR_CONTROL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LEN` writer - Length of message"]
pub type LEN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, MWMSG_DDR_MWMSG_DDR_CONTROL_SPEC, u16, u16, 10, O>;
#[doc = "Field `END` writer - End of message"]
pub type END_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, MWMSG_DDR_MWMSG_DDR_CONTROL_SPEC, bool, O>;
impl W {
    #[doc = "Bits 0:9 - Length of message"]
    #[inline(always)]
    #[must_use]
    pub fn len(&mut self) -> LEN_W<0> {
        LEN_W::new(self)
    }
    #[doc = "Bit 14 - End of message"]
    #[inline(always)]
    #[must_use]
    pub fn end(&mut self) -> END_W<14> {
        END_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Master Write Message in DDR mode\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mwmsg_ddr_mwmsg_ddr_control](index.html) module"]
pub struct MWMSG_DDR_MWMSG_DDR_CONTROL_SPEC;
impl crate::RegisterSpec for MWMSG_DDR_MWMSG_DDR_CONTROL_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [mwmsg_ddr_mwmsg_ddr_control::W](W) writer structure"]
impl crate::Writable for MWMSG_DDR_MWMSG_DDR_CONTROL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MWMSG_DDR_CONTROL to value 0"]
impl crate::Resettable for MWMSG_DDR_MWMSG_DDR_CONTROL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
