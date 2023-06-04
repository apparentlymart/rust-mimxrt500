#[doc = "Register `USB1_LOOPBACK` reader"]
pub struct R(crate::R<USB1_LOOPBACK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USB1_LOOPBACK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USB1_LOOPBACK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USB1_LOOPBACK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USB1_LOOPBACK` writer"]
pub struct W(crate::W<USB1_LOOPBACK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USB1_LOOPBACK_SPEC>;
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
impl From<crate::W<USB1_LOOPBACK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USB1_LOOPBACK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UTMI_TESTSTART` reader - USB loopback test."]
pub type UTMI_TESTSTART_R = crate::BitReader<bool>;
#[doc = "Field `UTMI_TESTSTART` writer - USB loopback test."]
pub type UTMI_TESTSTART_W<'a, const O: u8> = crate::BitWriter<'a, u32, USB1_LOOPBACK_SPEC, bool, O>;
#[doc = "Field `UTMI_DIG_TST0` reader - Mode control for USB loopback test."]
pub type UTMI_DIG_TST0_R = crate::BitReader<bool>;
#[doc = "Field `UTMI_DIG_TST0` writer - Mode control for USB loopback test."]
pub type UTMI_DIG_TST0_W<'a, const O: u8> = crate::BitWriter<'a, u32, USB1_LOOPBACK_SPEC, bool, O>;
#[doc = "Field `UTMI_DIG_TST1` reader - Mode control for USB loopback test."]
pub type UTMI_DIG_TST1_R = crate::BitReader<bool>;
#[doc = "Field `UTMI_DIG_TST1` writer - Mode control for USB loopback test."]
pub type UTMI_DIG_TST1_W<'a, const O: u8> = crate::BitWriter<'a, u32, USB1_LOOPBACK_SPEC, bool, O>;
#[doc = "Field `TSTI_TX_HS_MODE` reader - Select HS or FS mode for USB loopback testing."]
pub type TSTI_TX_HS_MODE_R = crate::BitReader<bool>;
#[doc = "Field `TSTI_TX_HS_MODE` writer - Select HS or FS mode for USB loopback testing."]
pub type TSTI_TX_HS_MODE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, USB1_LOOPBACK_SPEC, bool, O>;
#[doc = "Field `TSTI_TX_LS_MODE` reader - Select HS or FS mode for USB loopback testing."]
pub type TSTI_TX_LS_MODE_R = crate::BitReader<bool>;
#[doc = "Field `TSTI_TX_LS_MODE` writer - Select HS or FS mode for USB loopback testing."]
pub type TSTI_TX_LS_MODE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, USB1_LOOPBACK_SPEC, bool, O>;
#[doc = "Field `TSTI_TX_EN` reader - Enable TX for USB loopback test."]
pub type TSTI_TX_EN_R = crate::BitReader<bool>;
#[doc = "Field `TSTI_TX_EN` writer - Enable TX for USB loopback test."]
pub type TSTI_TX_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, USB1_LOOPBACK_SPEC, bool, O>;
#[doc = "Field `TSTI_TX_HIZ` reader - Sets TX Hi-Z for USB loopback test."]
pub type TSTI_TX_HIZ_R = crate::BitReader<bool>;
#[doc = "Field `TSTI_TX_HIZ` writer - Sets TX Hi-Z for USB loopback test."]
pub type TSTI_TX_HIZ_W<'a, const O: u8> = crate::BitWriter<'a, u32, USB1_LOOPBACK_SPEC, bool, O>;
#[doc = "Field `UTMO_DIG_TST0` reader - Status bit for USB loopback test."]
pub type UTMO_DIG_TST0_R = crate::BitReader<bool>;
#[doc = "Field `UTMO_DIG_TST1` reader - Status bit for USB loopback test."]
pub type UTMO_DIG_TST1_R = crate::BitReader<bool>;
#[doc = "Field `TSTI_HSFS_MODE_EN` reader - Setting this bit field to value 1'b1 will enable the loopback test to dynamically change the packet speed"]
pub type TSTI_HSFS_MODE_EN_R = crate::BitReader<bool>;
#[doc = "Field `TSTI_HSFS_MODE_EN` writer - Setting this bit field to value 1'b1 will enable the loopback test to dynamically change the packet speed"]
pub type TSTI_HSFS_MODE_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, USB1_LOOPBACK_SPEC, bool, O>;
#[doc = "Field `TSTPKT` reader - Test packet"]
pub type TSTPKT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TSTPKT` writer - Test packet"]
pub type TSTPKT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, USB1_LOOPBACK_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bit 0 - USB loopback test."]
    #[inline(always)]
    pub fn utmi_teststart(&self) -> UTMI_TESTSTART_R {
        UTMI_TESTSTART_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Mode control for USB loopback test."]
    #[inline(always)]
    pub fn utmi_dig_tst0(&self) -> UTMI_DIG_TST0_R {
        UTMI_DIG_TST0_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Mode control for USB loopback test."]
    #[inline(always)]
    pub fn utmi_dig_tst1(&self) -> UTMI_DIG_TST1_R {
        UTMI_DIG_TST1_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Select HS or FS mode for USB loopback testing."]
    #[inline(always)]
    pub fn tsti_tx_hs_mode(&self) -> TSTI_TX_HS_MODE_R {
        TSTI_TX_HS_MODE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Select HS or FS mode for USB loopback testing."]
    #[inline(always)]
    pub fn tsti_tx_ls_mode(&self) -> TSTI_TX_LS_MODE_R {
        TSTI_TX_LS_MODE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable TX for USB loopback test."]
    #[inline(always)]
    pub fn tsti_tx_en(&self) -> TSTI_TX_EN_R {
        TSTI_TX_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Sets TX Hi-Z for USB loopback test."]
    #[inline(always)]
    pub fn tsti_tx_hiz(&self) -> TSTI_TX_HIZ_R {
        TSTI_TX_HIZ_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Status bit for USB loopback test."]
    #[inline(always)]
    pub fn utmo_dig_tst0(&self) -> UTMO_DIG_TST0_R {
        UTMO_DIG_TST0_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Status bit for USB loopback test."]
    #[inline(always)]
    pub fn utmo_dig_tst1(&self) -> UTMO_DIG_TST1_R {
        UTMO_DIG_TST1_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 15 - Setting this bit field to value 1'b1 will enable the loopback test to dynamically change the packet speed"]
    #[inline(always)]
    pub fn tsti_hsfs_mode_en(&self) -> TSTI_HSFS_MODE_EN_R {
        TSTI_HSFS_MODE_EN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:23 - Test packet"]
    #[inline(always)]
    pub fn tstpkt(&self) -> TSTPKT_R {
        TSTPKT_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - USB loopback test."]
    #[inline(always)]
    #[must_use]
    pub fn utmi_teststart(&mut self) -> UTMI_TESTSTART_W<0> {
        UTMI_TESTSTART_W::new(self)
    }
    #[doc = "Bit 1 - Mode control for USB loopback test."]
    #[inline(always)]
    #[must_use]
    pub fn utmi_dig_tst0(&mut self) -> UTMI_DIG_TST0_W<1> {
        UTMI_DIG_TST0_W::new(self)
    }
    #[doc = "Bit 2 - Mode control for USB loopback test."]
    #[inline(always)]
    #[must_use]
    pub fn utmi_dig_tst1(&mut self) -> UTMI_DIG_TST1_W<2> {
        UTMI_DIG_TST1_W::new(self)
    }
    #[doc = "Bit 3 - Select HS or FS mode for USB loopback testing."]
    #[inline(always)]
    #[must_use]
    pub fn tsti_tx_hs_mode(&mut self) -> TSTI_TX_HS_MODE_W<3> {
        TSTI_TX_HS_MODE_W::new(self)
    }
    #[doc = "Bit 4 - Select HS or FS mode for USB loopback testing."]
    #[inline(always)]
    #[must_use]
    pub fn tsti_tx_ls_mode(&mut self) -> TSTI_TX_LS_MODE_W<4> {
        TSTI_TX_LS_MODE_W::new(self)
    }
    #[doc = "Bit 5 - Enable TX for USB loopback test."]
    #[inline(always)]
    #[must_use]
    pub fn tsti_tx_en(&mut self) -> TSTI_TX_EN_W<5> {
        TSTI_TX_EN_W::new(self)
    }
    #[doc = "Bit 6 - Sets TX Hi-Z for USB loopback test."]
    #[inline(always)]
    #[must_use]
    pub fn tsti_tx_hiz(&mut self) -> TSTI_TX_HIZ_W<6> {
        TSTI_TX_HIZ_W::new(self)
    }
    #[doc = "Bit 15 - Setting this bit field to value 1'b1 will enable the loopback test to dynamically change the packet speed"]
    #[inline(always)]
    #[must_use]
    pub fn tsti_hsfs_mode_en(&mut self) -> TSTI_HSFS_MODE_EN_W<15> {
        TSTI_HSFS_MODE_EN_W::new(self)
    }
    #[doc = "Bits 16:23 - Test packet"]
    #[inline(always)]
    #[must_use]
    pub fn tstpkt(&mut self) -> TSTPKT_W<16> {
        TSTPKT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB PHY Loopback Control/Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb1_loopback](index.html) module"]
pub struct USB1_LOOPBACK_SPEC;
impl crate::RegisterSpec for USB1_LOOPBACK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [usb1_loopback::R](R) reader structure"]
impl crate::Readable for USB1_LOOPBACK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usb1_loopback::W](W) writer structure"]
impl crate::Writable for USB1_LOOPBACK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets USB1_LOOPBACK to value 0x0055_0000"]
impl crate::Resettable for USB1_LOOPBACK_SPEC {
    const RESET_VALUE: Self::Ux = 0x0055_0000;
}
