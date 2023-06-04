#[doc = "Register `PRSTCTL1_CLR` writer"]
pub struct W(crate::W<PRSTCTL1_CLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRSTCTL1_CLR_SPEC>;
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
impl From<crate::W<PRSTCTL1_CLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRSTCTL1_CLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "HSGPIO0 reset clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSGPIO0_AW {
    #[doc = "0: No effect"]
    HSGPIO_CLR = 0,
    #[doc = "1: Clears the PRSTCTL1 Bit"]
    HSGPIO_SET = 1,
}
impl From<HSGPIO0_AW> for bool {
    #[inline(always)]
    fn from(variant: HSGPIO0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSGPIO0` writer - HSGPIO0 reset clear"]
pub type HSGPIO0_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRSTCTL1_CLR_SPEC, HSGPIO0_AW, O>;
impl<'a, const O: u8> HSGPIO0_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn hsgpio_clr(self) -> &'a mut W {
        self.variant(HSGPIO0_AW::HSGPIO_CLR)
    }
    #[doc = "Clears the PRSTCTL1 Bit"]
    #[inline(always)]
    pub fn hsgpio_set(self) -> &'a mut W {
        self.variant(HSGPIO0_AW::HSGPIO_SET)
    }
}
#[doc = "HSGPIO1 reset clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSGPIO1_AW {
    #[doc = "0: No effect"]
    HSGPIO_CLR = 0,
    #[doc = "1: Clears the PRSTCTL1 Bit"]
    HSGPIO_SET = 1,
}
impl From<HSGPIO1_AW> for bool {
    #[inline(always)]
    fn from(variant: HSGPIO1_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSGPIO1` writer - HSGPIO1 reset clear"]
pub type HSGPIO1_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRSTCTL1_CLR_SPEC, HSGPIO1_AW, O>;
impl<'a, const O: u8> HSGPIO1_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn hsgpio_clr(self) -> &'a mut W {
        self.variant(HSGPIO1_AW::HSGPIO_CLR)
    }
    #[doc = "Clears the PRSTCTL1 Bit"]
    #[inline(always)]
    pub fn hsgpio_set(self) -> &'a mut W {
        self.variant(HSGPIO1_AW::HSGPIO_SET)
    }
}
#[doc = "HSGPIO2 reset clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSGPIO2_AW {
    #[doc = "0: No effect"]
    HSGPIO_CLR = 0,
    #[doc = "1: Clears the PRSTCTL1 Bit"]
    HSGPIO_SET = 1,
}
impl From<HSGPIO2_AW> for bool {
    #[inline(always)]
    fn from(variant: HSGPIO2_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSGPIO2` writer - HSGPIO2 reset clear"]
pub type HSGPIO2_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRSTCTL1_CLR_SPEC, HSGPIO2_AW, O>;
impl<'a, const O: u8> HSGPIO2_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn hsgpio_clr(self) -> &'a mut W {
        self.variant(HSGPIO2_AW::HSGPIO_CLR)
    }
    #[doc = "Clears the PRSTCTL1 Bit"]
    #[inline(always)]
    pub fn hsgpio_set(self) -> &'a mut W {
        self.variant(HSGPIO2_AW::HSGPIO_SET)
    }
}
#[doc = "HSGPIO3 reset clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSGPIO3_AW {
    #[doc = "0: No effect"]
    HSGPIO_CLR = 0,
    #[doc = "1: Clears the PRSTCTL1 Bit"]
    HSGPIO_SET = 1,
}
impl From<HSGPIO3_AW> for bool {
    #[inline(always)]
    fn from(variant: HSGPIO3_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSGPIO3` writer - HSGPIO3 reset clear"]
pub type HSGPIO3_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRSTCTL1_CLR_SPEC, HSGPIO3_AW, O>;
impl<'a, const O: u8> HSGPIO3_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn hsgpio_clr(self) -> &'a mut W {
        self.variant(HSGPIO3_AW::HSGPIO_CLR)
    }
    #[doc = "Clears the PRSTCTL1 Bit"]
    #[inline(always)]
    pub fn hsgpio_set(self) -> &'a mut W {
        self.variant(HSGPIO3_AW::HSGPIO_SET)
    }
}
#[doc = "HSGPIO4 reset clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSGPIO4_AW {
    #[doc = "0: No effect"]
    HSGPIO_CLR = 0,
    #[doc = "1: Clears the PRSTCTL1 Bit"]
    HSGPIO_SET = 1,
}
impl From<HSGPIO4_AW> for bool {
    #[inline(always)]
    fn from(variant: HSGPIO4_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSGPIO4` writer - HSGPIO4 reset clear"]
pub type HSGPIO4_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRSTCTL1_CLR_SPEC, HSGPIO4_AW, O>;
impl<'a, const O: u8> HSGPIO4_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn hsgpio_clr(self) -> &'a mut W {
        self.variant(HSGPIO4_AW::HSGPIO_CLR)
    }
    #[doc = "Clears the PRSTCTL1 Bit"]
    #[inline(always)]
    pub fn hsgpio_set(self) -> &'a mut W {
        self.variant(HSGPIO4_AW::HSGPIO_SET)
    }
}
#[doc = "HSGPIO5 reset clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSGPIO5_AW {
    #[doc = "0: No effect"]
    HSGPIO_CLR = 0,
    #[doc = "1: Clears the PRSTCTL1 Bit"]
    HSGPIO_SET = 1,
}
impl From<HSGPIO5_AW> for bool {
    #[inline(always)]
    fn from(variant: HSGPIO5_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSGPIO5` writer - HSGPIO5 reset clear"]
pub type HSGPIO5_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRSTCTL1_CLR_SPEC, HSGPIO5_AW, O>;
impl<'a, const O: u8> HSGPIO5_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn hsgpio_clr(self) -> &'a mut W {
        self.variant(HSGPIO5_AW::HSGPIO_CLR)
    }
    #[doc = "Clears the PRSTCTL1 Bit"]
    #[inline(always)]
    pub fn hsgpio_set(self) -> &'a mut W {
        self.variant(HSGPIO5_AW::HSGPIO_SET)
    }
}
#[doc = "HSGPIO6 reset clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSGPIO6_AW {
    #[doc = "0: No effect"]
    HSGPIO_CLR = 0,
    #[doc = "1: Clears the PRSTCTL1 Bit"]
    HSGPIO_SET = 1,
}
impl From<HSGPIO6_AW> for bool {
    #[inline(always)]
    fn from(variant: HSGPIO6_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSGPIO6` writer - HSGPIO6 reset clear"]
pub type HSGPIO6_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRSTCTL1_CLR_SPEC, HSGPIO6_AW, O>;
impl<'a, const O: u8> HSGPIO6_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn hsgpio_clr(self) -> &'a mut W {
        self.variant(HSGPIO6_AW::HSGPIO_CLR)
    }
    #[doc = "Clears the PRSTCTL1 Bit"]
    #[inline(always)]
    pub fn hsgpio_set(self) -> &'a mut W {
        self.variant(HSGPIO6_AW::HSGPIO_SET)
    }
}
#[doc = "HSGPIO7 reset clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSGPIO7_AW {
    #[doc = "0: No effect"]
    HSGPIO_CLR = 0,
    #[doc = "1: Clears the PRSTCTL1 Bit"]
    HSGPIO_SET = 1,
}
impl From<HSGPIO7_AW> for bool {
    #[inline(always)]
    fn from(variant: HSGPIO7_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSGPIO7` writer - HSGPIO7 reset clear"]
pub type HSGPIO7_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRSTCTL1_CLR_SPEC, HSGPIO7_AW, O>;
impl<'a, const O: u8> HSGPIO7_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn hsgpio_clr(self) -> &'a mut W {
        self.variant(HSGPIO7_AW::HSGPIO_CLR)
    }
    #[doc = "Clears the PRSTCTL1 Bit"]
    #[inline(always)]
    pub fn hsgpio_set(self) -> &'a mut W {
        self.variant(HSGPIO7_AW::HSGPIO_SET)
    }
}
#[doc = "CRC reset clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CRC_AW {
    #[doc = "0: No effect"]
    CRC_CLR = 0,
    #[doc = "1: Clears the PRSTCTL1 Bit"]
    CRC_SET = 1,
}
impl From<CRC_AW> for bool {
    #[inline(always)]
    fn from(variant: CRC_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CRC` writer - CRC reset clear"]
pub type CRC_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRSTCTL1_CLR_SPEC, CRC_AW, O>;
impl<'a, const O: u8> CRC_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn crc_clr(self) -> &'a mut W {
        self.variant(CRC_AW::CRC_CLR)
    }
    #[doc = "Clears the PRSTCTL1 Bit"]
    #[inline(always)]
    pub fn crc_set(self) -> &'a mut W {
        self.variant(CRC_AW::CRC_SET)
    }
}
#[doc = "DMAC0 reset clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMAC0_AW {
    #[doc = "0: No effect"]
    DMAC_CLR = 0,
    #[doc = "1: Clears the PRSTCTL1 Bit"]
    DMAC_SET = 1,
}
impl From<DMAC0_AW> for bool {
    #[inline(always)]
    fn from(variant: DMAC0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAC0` writer - DMAC0 reset clear"]
pub type DMAC0_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRSTCTL1_CLR_SPEC, DMAC0_AW, O>;
impl<'a, const O: u8> DMAC0_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn dmac_clr(self) -> &'a mut W {
        self.variant(DMAC0_AW::DMAC_CLR)
    }
    #[doc = "Clears the PRSTCTL1 Bit"]
    #[inline(always)]
    pub fn dmac_set(self) -> &'a mut W {
        self.variant(DMAC0_AW::DMAC_SET)
    }
}
#[doc = "DMAC1 reset clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMAC1_AW {
    #[doc = "0: No effect"]
    DMAC_CLR = 0,
    #[doc = "1: Clears the PRSTCTL1 Bit"]
    DMAC_SET = 1,
}
impl From<DMAC1_AW> for bool {
    #[inline(always)]
    fn from(variant: DMAC1_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAC1` writer - DMAC1 reset clear"]
pub type DMAC1_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRSTCTL1_CLR_SPEC, DMAC1_AW, O>;
impl<'a, const O: u8> DMAC1_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn dmac_clr(self) -> &'a mut W {
        self.variant(DMAC1_AW::DMAC_CLR)
    }
    #[doc = "Clears the PRSTCTL1 Bit"]
    #[inline(always)]
    pub fn dmac_set(self) -> &'a mut W {
        self.variant(DMAC1_AW::DMAC_SET)
    }
}
#[doc = "MU reset clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MU_AW {
    #[doc = "0: No effect"]
    MU_CLR = 0,
    #[doc = "1: Clears the PRSTCTL1 Bit"]
    MU_SET = 1,
}
impl From<MU_AW> for bool {
    #[inline(always)]
    fn from(variant: MU_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MU` writer - MU reset clear"]
pub type MU_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRSTCTL1_CLR_SPEC, MU_AW, O>;
impl<'a, const O: u8> MU_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn mu_clr(self) -> &'a mut W {
        self.variant(MU_AW::MU_CLR)
    }
    #[doc = "Clears the PRSTCTL1 Bit"]
    #[inline(always)]
    pub fn mu_set(self) -> &'a mut W {
        self.variant(MU_AW::MU_SET)
    }
}
#[doc = "SMEA reset clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SEMA_AW {
    #[doc = "0: No effect"]
    SEMA_CLR = 0,
    #[doc = "1: Clears the PRSTCTL1 Bit"]
    SEMA_SET = 1,
}
impl From<SEMA_AW> for bool {
    #[inline(always)]
    fn from(variant: SEMA_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SEMA` writer - SMEA reset clear"]
pub type SEMA_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRSTCTL1_CLR_SPEC, SEMA_AW, O>;
impl<'a, const O: u8> SEMA_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn sema_clr(self) -> &'a mut W {
        self.variant(SEMA_AW::SEMA_CLR)
    }
    #[doc = "Clears the PRSTCTL1 Bit"]
    #[inline(always)]
    pub fn sema_set(self) -> &'a mut W {
        self.variant(SEMA_AW::SEMA_SET)
    }
}
#[doc = "FREQME reset clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FREQME_AW {
    #[doc = "0: No effect"]
    FREQME_CLR = 0,
    #[doc = "1: Clears the PRSTCTL1 Bit"]
    FREQME_SET = 1,
}
impl From<FREQME_AW> for bool {
    #[inline(always)]
    fn from(variant: FREQME_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FREQME` writer - FREQME reset clear"]
pub type FREQME_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRSTCTL1_CLR_SPEC, FREQME_AW, O>;
impl<'a, const O: u8> FREQME_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn freqme_clr(self) -> &'a mut W {
        self.variant(FREQME_AW::FREQME_CLR)
    }
    #[doc = "Clears the PRSTCTL1 Bit"]
    #[inline(always)]
    pub fn freqme_set(self) -> &'a mut W {
        self.variant(FREQME_AW::FREQME_SET)
    }
}
impl W {
    #[doc = "Bit 0 - HSGPIO0 reset clear"]
    #[inline(always)]
    #[must_use]
    pub fn hsgpio0(&mut self) -> HSGPIO0_W<0> {
        HSGPIO0_W::new(self)
    }
    #[doc = "Bit 1 - HSGPIO1 reset clear"]
    #[inline(always)]
    #[must_use]
    pub fn hsgpio1(&mut self) -> HSGPIO1_W<1> {
        HSGPIO1_W::new(self)
    }
    #[doc = "Bit 2 - HSGPIO2 reset clear"]
    #[inline(always)]
    #[must_use]
    pub fn hsgpio2(&mut self) -> HSGPIO2_W<2> {
        HSGPIO2_W::new(self)
    }
    #[doc = "Bit 3 - HSGPIO3 reset clear"]
    #[inline(always)]
    #[must_use]
    pub fn hsgpio3(&mut self) -> HSGPIO3_W<3> {
        HSGPIO3_W::new(self)
    }
    #[doc = "Bit 4 - HSGPIO4 reset clear"]
    #[inline(always)]
    #[must_use]
    pub fn hsgpio4(&mut self) -> HSGPIO4_W<4> {
        HSGPIO4_W::new(self)
    }
    #[doc = "Bit 5 - HSGPIO5 reset clear"]
    #[inline(always)]
    #[must_use]
    pub fn hsgpio5(&mut self) -> HSGPIO5_W<5> {
        HSGPIO5_W::new(self)
    }
    #[doc = "Bit 6 - HSGPIO6 reset clear"]
    #[inline(always)]
    #[must_use]
    pub fn hsgpio6(&mut self) -> HSGPIO6_W<6> {
        HSGPIO6_W::new(self)
    }
    #[doc = "Bit 7 - HSGPIO7 reset clear"]
    #[inline(always)]
    #[must_use]
    pub fn hsgpio7(&mut self) -> HSGPIO7_W<7> {
        HSGPIO7_W::new(self)
    }
    #[doc = "Bit 16 - CRC reset clear"]
    #[inline(always)]
    #[must_use]
    pub fn crc(&mut self) -> CRC_W<16> {
        CRC_W::new(self)
    }
    #[doc = "Bit 23 - DMAC0 reset clear"]
    #[inline(always)]
    #[must_use]
    pub fn dmac0(&mut self) -> DMAC0_W<23> {
        DMAC0_W::new(self)
    }
    #[doc = "Bit 24 - DMAC1 reset clear"]
    #[inline(always)]
    #[must_use]
    pub fn dmac1(&mut self) -> DMAC1_W<24> {
        DMAC1_W::new(self)
    }
    #[doc = "Bit 28 - MU reset clear"]
    #[inline(always)]
    #[must_use]
    pub fn mu(&mut self) -> MU_W<28> {
        MU_W::new(self)
    }
    #[doc = "Bit 29 - SMEA reset clear"]
    #[inline(always)]
    #[must_use]
    pub fn sema(&mut self) -> SEMA_W<29> {
        SEMA_W::new(self)
    }
    #[doc = "Bit 31 - FREQME reset clear"]
    #[inline(always)]
    #[must_use]
    pub fn freqme(&mut self) -> FREQME_W<31> {
        FREQME_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Peripheral Reset Control Register 1 CLR\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prstctl1_clr](index.html) module"]
pub struct PRSTCTL1_CLR_SPEC;
impl crate::RegisterSpec for PRSTCTL1_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [prstctl1_clr::W](W) writer structure"]
impl crate::Writable for PRSTCTL1_CLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PRSTCTL1_CLR to value 0"]
impl crate::Resettable for PRSTCTL1_CLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
