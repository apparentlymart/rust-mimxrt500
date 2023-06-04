#[doc = "Register `FLEXSPI1PADCTL` reader"]
pub struct R(crate::R<FLEXSPI1PADCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLEXSPI1PADCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLEXSPI1PADCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLEXSPI1PADCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FLEXSPI1PADCTL` writer"]
pub struct W(crate::W<FLEXSPI1PADCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FLEXSPI1PADCTL_SPEC>;
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
impl From<crate::W<FLEXSPI1PADCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FLEXSPI1PADCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RASRCN_3_0` reader - Drives FLEXSPI1 Pad Compensation Circuit"]
pub type RASRCN_3_0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RASRCN_3_0` writer - Drives FLEXSPI1 Pad Compensation Circuit"]
pub type RASRCN_3_0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FLEXSPI1PADCTL_SPEC, u8, u8, 4, O>;
#[doc = "Field `RASRCP_3_0` reader - Drives FLEXSPI1 Pad Compensation Circuit"]
pub type RASRCP_3_0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RASRCP_3_0` writer - Drives FLEXSPI1 Pad Compensation Circuit"]
pub type RASRCP_3_0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FLEXSPI1PADCTL_SPEC, u8, u8, 4, O>;
#[doc = "Field `FASTFRZ` reader - Drives FLEXSPI1 Pad Compensation Circuit"]
pub type FASTFRZ_R = crate::BitReader<bool>;
#[doc = "Field `FASTFRZ` writer - Drives FLEXSPI1 Pad Compensation Circuit"]
pub type FASTFRZ_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLEXSPI1PADCTL_SPEC, bool, O>;
#[doc = "Field `FREEZE` reader - Drives FLEXSPI1 Pad Compensation Circuit"]
pub type FREEZE_R = crate::BitReader<bool>;
#[doc = "Field `FREEZE` writer - Drives FLEXSPI1 Pad Compensation Circuit"]
pub type FREEZE_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLEXSPI1PADCTL_SPEC, bool, O>;
#[doc = "Field `COMPTQ` reader - Drives FLEXSPI1 Pad Compensation Circuit"]
pub type COMPTQ_R = crate::BitReader<bool>;
#[doc = "Field `COMPTQ` writer - Drives FLEXSPI1 Pad Compensation Circuit"]
pub type COMPTQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLEXSPI1PADCTL_SPEC, bool, O>;
#[doc = "Field `COMPEN` reader - Drives FLEXSPI1 Pad Compensation Circuit"]
pub type COMPEN_R = crate::BitReader<bool>;
#[doc = "Field `COMPEN` writer - Drives FLEXSPI1 Pad Compensation Circuit"]
pub type COMPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLEXSPI1PADCTL_SPEC, bool, O>;
#[doc = "Field `NASRCN_3_0` reader - FLEXSPI1 Pad Compensation Circuit Status"]
pub type NASRCN_3_0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `NASRCP_3_0` reader - FLEXSPI1 Pad Compensation Circuit Status"]
pub type NASRCP_3_0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COMPOK` reader - FLEXSPI1 Pad Compensation Circuit Status"]
pub type COMPOK_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:3 - Drives FLEXSPI1 Pad Compensation Circuit"]
    #[inline(always)]
    pub fn rasrcn_3_0(&self) -> RASRCN_3_0_R {
        RASRCN_3_0_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Drives FLEXSPI1 Pad Compensation Circuit"]
    #[inline(always)]
    pub fn rasrcp_3_0(&self) -> RASRCP_3_0_R {
        RASRCP_3_0_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - Drives FLEXSPI1 Pad Compensation Circuit"]
    #[inline(always)]
    pub fn fastfrz(&self) -> FASTFRZ_R {
        FASTFRZ_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Drives FLEXSPI1 Pad Compensation Circuit"]
    #[inline(always)]
    pub fn freeze(&self) -> FREEZE_R {
        FREEZE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Drives FLEXSPI1 Pad Compensation Circuit"]
    #[inline(always)]
    pub fn comptq(&self) -> COMPTQ_R {
        COMPTQ_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Drives FLEXSPI1 Pad Compensation Circuit"]
    #[inline(always)]
    pub fn compen(&self) -> COMPEN_R {
        COMPEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 16:19 - FLEXSPI1 Pad Compensation Circuit Status"]
    #[inline(always)]
    pub fn nasrcn_3_0(&self) -> NASRCN_3_0_R {
        NASRCN_3_0_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - FLEXSPI1 Pad Compensation Circuit Status"]
    #[inline(always)]
    pub fn nasrcp_3_0(&self) -> NASRCP_3_0_R {
        NASRCP_3_0_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - FLEXSPI1 Pad Compensation Circuit Status"]
    #[inline(always)]
    pub fn compok(&self) -> COMPOK_R {
        COMPOK_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Drives FLEXSPI1 Pad Compensation Circuit"]
    #[inline(always)]
    #[must_use]
    pub fn rasrcn_3_0(&mut self) -> RASRCN_3_0_W<0> {
        RASRCN_3_0_W::new(self)
    }
    #[doc = "Bits 4:7 - Drives FLEXSPI1 Pad Compensation Circuit"]
    #[inline(always)]
    #[must_use]
    pub fn rasrcp_3_0(&mut self) -> RASRCP_3_0_W<4> {
        RASRCP_3_0_W::new(self)
    }
    #[doc = "Bit 8 - Drives FLEXSPI1 Pad Compensation Circuit"]
    #[inline(always)]
    #[must_use]
    pub fn fastfrz(&mut self) -> FASTFRZ_W<8> {
        FASTFRZ_W::new(self)
    }
    #[doc = "Bit 9 - Drives FLEXSPI1 Pad Compensation Circuit"]
    #[inline(always)]
    #[must_use]
    pub fn freeze(&mut self) -> FREEZE_W<9> {
        FREEZE_W::new(self)
    }
    #[doc = "Bit 10 - Drives FLEXSPI1 Pad Compensation Circuit"]
    #[inline(always)]
    #[must_use]
    pub fn comptq(&mut self) -> COMPTQ_W<10> {
        COMPTQ_W::new(self)
    }
    #[doc = "Bit 11 - Drives FLEXSPI1 Pad Compensation Circuit"]
    #[inline(always)]
    #[must_use]
    pub fn compen(&mut self) -> COMPEN_W<11> {
        COMPEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FLEXSPI1 Pad Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flexspi1padctl](index.html) module"]
pub struct FLEXSPI1PADCTL_SPEC;
impl crate::RegisterSpec for FLEXSPI1PADCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [flexspi1padctl::R](R) reader structure"]
impl crate::Readable for FLEXSPI1PADCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [flexspi1padctl::W](W) writer structure"]
impl crate::Writable for FLEXSPI1PADCTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FLEXSPI1PADCTL to value 0x01a5_0000"]
impl crate::Resettable for FLEXSPI1PADCTL_SPEC {
    const RESET_VALUE: Self::Ux = 0x01a5_0000;
}
