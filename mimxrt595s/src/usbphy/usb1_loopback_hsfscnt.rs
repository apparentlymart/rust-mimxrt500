#[doc = "Register `USB1_LOOPBACK_HSFSCNT` reader"]
pub struct R(crate::R<USB1_LOOPBACK_HSFSCNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USB1_LOOPBACK_HSFSCNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USB1_LOOPBACK_HSFSCNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USB1_LOOPBACK_HSFSCNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USB1_LOOPBACK_HSFSCNT` writer"]
pub struct W(crate::W<USB1_LOOPBACK_HSFSCNT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USB1_LOOPBACK_HSFSCNT_SPEC>;
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
impl From<crate::W<USB1_LOOPBACK_HSFSCNT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USB1_LOOPBACK_HSFSCNT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TSTI_HS_NUMBER` reader - USB loopback test HS CNT."]
pub type TSTI_HS_NUMBER_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TSTI_HS_NUMBER` writer - USB loopback test HS CNT."]
pub type TSTI_HS_NUMBER_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, USB1_LOOPBACK_HSFSCNT_SPEC, u16, u16, 16, O>;
#[doc = "Field `TSTI_FS_NUMBER` reader - USB loopback test FS CNT."]
pub type TSTI_FS_NUMBER_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TSTI_FS_NUMBER` writer - USB loopback test FS CNT."]
pub type TSTI_FS_NUMBER_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, USB1_LOOPBACK_HSFSCNT_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - USB loopback test HS CNT."]
    #[inline(always)]
    pub fn tsti_hs_number(&self) -> TSTI_HS_NUMBER_R {
        TSTI_HS_NUMBER_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - USB loopback test FS CNT."]
    #[inline(always)]
    pub fn tsti_fs_number(&self) -> TSTI_FS_NUMBER_R {
        TSTI_FS_NUMBER_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - USB loopback test HS CNT."]
    #[inline(always)]
    #[must_use]
    pub fn tsti_hs_number(&mut self) -> TSTI_HS_NUMBER_W<0> {
        TSTI_HS_NUMBER_W::new(self)
    }
    #[doc = "Bits 16:31 - USB loopback test FS CNT."]
    #[inline(always)]
    #[must_use]
    pub fn tsti_fs_number(&mut self) -> TSTI_FS_NUMBER_W<16> {
        TSTI_FS_NUMBER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Loopback Packet Number Select\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb1_loopback_hsfscnt](index.html) module"]
pub struct USB1_LOOPBACK_HSFSCNT_SPEC;
impl crate::RegisterSpec for USB1_LOOPBACK_HSFSCNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [usb1_loopback_hsfscnt::R](R) reader structure"]
impl crate::Readable for USB1_LOOPBACK_HSFSCNT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usb1_loopback_hsfscnt::W](W) writer structure"]
impl crate::Writable for USB1_LOOPBACK_HSFSCNT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets USB1_LOOPBACK_HSFSCNT to value 0x0004_0010"]
impl crate::Resettable for USB1_LOOPBACK_HSFSCNT_SPEC {
    const RESET_VALUE: Self::Ux = 0x0004_0010;
}
