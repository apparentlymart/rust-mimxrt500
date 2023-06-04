#[doc = "Register `PRSTCTL2_SET` writer"]
pub struct W(crate::W<PRSTCTL2_SET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRSTCTL2_SET_SPEC>;
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
impl From<crate::W<PRSTCTL2_SET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRSTCTL2_SET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "CT32BIT0 reset set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CT32BIT0_AW {
    #[doc = "0: No effect"]
    CT32BIT_CLR = 0,
    #[doc = "1: Sets the PRSTCTL2 Bit"]
    CT32BIT_SET = 1,
}
impl From<CT32BIT0_AW> for bool {
    #[inline(always)]
    fn from(variant: CT32BIT0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CT32BIT0` writer - CT32BIT0 reset set"]
pub type CT32BIT0_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRSTCTL2_SET_SPEC, CT32BIT0_AW, O>;
impl<'a, const O: u8> CT32BIT0_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn ct32bit_clr(self) -> &'a mut W {
        self.variant(CT32BIT0_AW::CT32BIT_CLR)
    }
    #[doc = "Sets the PRSTCTL2 Bit"]
    #[inline(always)]
    pub fn ct32bit_set(self) -> &'a mut W {
        self.variant(CT32BIT0_AW::CT32BIT_SET)
    }
}
#[doc = "CT32BIT1 reset set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CT32BIT1_AW {
    #[doc = "0: No effect"]
    CT32BIT_CLR = 0,
    #[doc = "1: Sets the PRSTCTL2 Bit"]
    CT32BIT_SET = 1,
}
impl From<CT32BIT1_AW> for bool {
    #[inline(always)]
    fn from(variant: CT32BIT1_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CT32BIT1` writer - CT32BIT1 reset set"]
pub type CT32BIT1_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRSTCTL2_SET_SPEC, CT32BIT1_AW, O>;
impl<'a, const O: u8> CT32BIT1_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn ct32bit_clr(self) -> &'a mut W {
        self.variant(CT32BIT1_AW::CT32BIT_CLR)
    }
    #[doc = "Sets the PRSTCTL2 Bit"]
    #[inline(always)]
    pub fn ct32bit_set(self) -> &'a mut W {
        self.variant(CT32BIT1_AW::CT32BIT_SET)
    }
}
#[doc = "CT32BIT2 reset set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CT32BIT2_AW {
    #[doc = "0: No effect"]
    CT32BIT_CLR = 0,
    #[doc = "1: Sets the PRSTCTL2 Bit"]
    CT32BIT_SET = 1,
}
impl From<CT32BIT2_AW> for bool {
    #[inline(always)]
    fn from(variant: CT32BIT2_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CT32BIT2` writer - CT32BIT2 reset set"]
pub type CT32BIT2_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRSTCTL2_SET_SPEC, CT32BIT2_AW, O>;
impl<'a, const O: u8> CT32BIT2_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn ct32bit_clr(self) -> &'a mut W {
        self.variant(CT32BIT2_AW::CT32BIT_CLR)
    }
    #[doc = "Sets the PRSTCTL2 Bit"]
    #[inline(always)]
    pub fn ct32bit_set(self) -> &'a mut W {
        self.variant(CT32BIT2_AW::CT32BIT_SET)
    }
}
#[doc = "CT32BIT3 reset set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CT32BIT3_AW {
    #[doc = "0: No effect"]
    CT32BIT_CLR = 0,
    #[doc = "1: Sets the PRSTCTL2 Bit"]
    CT32BIT_SET = 1,
}
impl From<CT32BIT3_AW> for bool {
    #[inline(always)]
    fn from(variant: CT32BIT3_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CT32BIT3` writer - CT32BIT3 reset set"]
pub type CT32BIT3_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRSTCTL2_SET_SPEC, CT32BIT3_AW, O>;
impl<'a, const O: u8> CT32BIT3_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn ct32bit_clr(self) -> &'a mut W {
        self.variant(CT32BIT3_AW::CT32BIT_CLR)
    }
    #[doc = "Sets the PRSTCTL2 Bit"]
    #[inline(always)]
    pub fn ct32bit_set(self) -> &'a mut W {
        self.variant(CT32BIT3_AW::CT32BIT_SET)
    }
}
#[doc = "CT32BIT4 reset set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CT32BIT4_AW {
    #[doc = "0: No effect"]
    CT32BIT_CLR = 0,
    #[doc = "1: Sets the PRSTCTL2 Bit"]
    CT32BIT_SET = 1,
}
impl From<CT32BIT4_AW> for bool {
    #[inline(always)]
    fn from(variant: CT32BIT4_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CT32BIT4` writer - CT32BIT4 reset set"]
pub type CT32BIT4_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRSTCTL2_SET_SPEC, CT32BIT4_AW, O>;
impl<'a, const O: u8> CT32BIT4_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn ct32bit_clr(self) -> &'a mut W {
        self.variant(CT32BIT4_AW::CT32BIT_CLR)
    }
    #[doc = "Sets the PRSTCTL2 Bit"]
    #[inline(always)]
    pub fn ct32bit_set(self) -> &'a mut W {
        self.variant(CT32BIT4_AW::CT32BIT_SET)
    }
}
#[doc = "MRT0 reset set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MRT0_AW {
    #[doc = "0: No effect"]
    MRT0_CLR = 0,
    #[doc = "1: Sets the PRSTCTL2 Bit"]
    MRT0_SET = 1,
}
impl From<MRT0_AW> for bool {
    #[inline(always)]
    fn from(variant: MRT0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MRT0` writer - MRT0 reset set"]
pub type MRT0_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRSTCTL2_SET_SPEC, MRT0_AW, O>;
impl<'a, const O: u8> MRT0_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn mrt0_clr(self) -> &'a mut W {
        self.variant(MRT0_AW::MRT0_CLR)
    }
    #[doc = "Sets the PRSTCTL2 Bit"]
    #[inline(always)]
    pub fn mrt0_set(self) -> &'a mut W {
        self.variant(MRT0_AW::MRT0_SET)
    }
}
#[doc = "WWDT1 reset set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WWDT1_AW {
    #[doc = "0: No effect"]
    WWDT1_CLR = 0,
    #[doc = "1: Sets the PRSTCTL2 Bit"]
    WWDT1_SET = 1,
}
impl From<WWDT1_AW> for bool {
    #[inline(always)]
    fn from(variant: WWDT1_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WWDT1` writer - WWDT1 reset set"]
pub type WWDT1_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRSTCTL2_SET_SPEC, WWDT1_AW, O>;
impl<'a, const O: u8> WWDT1_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn wwdt1_clr(self) -> &'a mut W {
        self.variant(WWDT1_AW::WWDT1_CLR)
    }
    #[doc = "Sets the PRSTCTL2 Bit"]
    #[inline(always)]
    pub fn wwdt1_set(self) -> &'a mut W {
        self.variant(WWDT1_AW::WWDT1_SET)
    }
}
#[doc = "I3C0 reset set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I3C0_AW {
    #[doc = "0: No effect"]
    I3C_CLR = 0,
    #[doc = "1: Sets the PRSTCTL2 Bit"]
    I3C_SET = 1,
}
impl From<I3C0_AW> for bool {
    #[inline(always)]
    fn from(variant: I3C0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `I3C0` writer - I3C0 reset set"]
pub type I3C0_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRSTCTL2_SET_SPEC, I3C0_AW, O>;
impl<'a, const O: u8> I3C0_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn i3c_clr(self) -> &'a mut W {
        self.variant(I3C0_AW::I3C_CLR)
    }
    #[doc = "Sets the PRSTCTL2 Bit"]
    #[inline(always)]
    pub fn i3c_set(self) -> &'a mut W {
        self.variant(I3C0_AW::I3C_SET)
    }
}
#[doc = "I3C1 reset set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I3C1_AW {
    #[doc = "0: No effect"]
    I3C_CLR = 0,
    #[doc = "1: Sets the PRSTCTL2 Bit"]
    I3C_SET = 1,
}
impl From<I3C1_AW> for bool {
    #[inline(always)]
    fn from(variant: I3C1_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `I3C1` writer - I3C1 reset set"]
pub type I3C1_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRSTCTL2_SET_SPEC, I3C1_AW, O>;
impl<'a, const O: u8> I3C1_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn i3c_clr(self) -> &'a mut W {
        self.variant(I3C1_AW::I3C_CLR)
    }
    #[doc = "Sets the PRSTCTL2 Bit"]
    #[inline(always)]
    pub fn i3c_set(self) -> &'a mut W {
        self.variant(I3C1_AW::I3C_SET)
    }
}
#[doc = "GPIOINTCTL reset set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIOINTCTL_AW {
    #[doc = "0: No effect"]
    GPIOINTCTL_CLR = 0,
    #[doc = "1: Sets the PRSTCTL2 Bit"]
    GPIOINTCTL_SET = 1,
}
impl From<GPIOINTCTL_AW> for bool {
    #[inline(always)]
    fn from(variant: GPIOINTCTL_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIOINTCTL` writer - GPIOINTCTL reset set"]
pub type GPIOINTCTL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PRSTCTL2_SET_SPEC, GPIOINTCTL_AW, O>;
impl<'a, const O: u8> GPIOINTCTL_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn gpiointctl_clr(self) -> &'a mut W {
        self.variant(GPIOINTCTL_AW::GPIOINTCTL_CLR)
    }
    #[doc = "Sets the PRSTCTL2 Bit"]
    #[inline(always)]
    pub fn gpiointctl_set(self) -> &'a mut W {
        self.variant(GPIOINTCTL_AW::GPIOINTCTL_SET)
    }
}
#[doc = "PIMCTL reset set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIMCTL_AW {
    #[doc = "0: No effect"]
    PIMCTL_CLR = 0,
    #[doc = "1: Sets the PRSTCTL2 Bit"]
    PIMCTL_SET = 1,
}
impl From<PIMCTL_AW> for bool {
    #[inline(always)]
    fn from(variant: PIMCTL_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIMCTL` writer - PIMCTL reset set"]
pub type PIMCTL_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRSTCTL2_SET_SPEC, PIMCTL_AW, O>;
impl<'a, const O: u8> PIMCTL_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn pimctl_clr(self) -> &'a mut W {
        self.variant(PIMCTL_AW::PIMCTL_CLR)
    }
    #[doc = "Sets the PRSTCTL2 Bit"]
    #[inline(always)]
    pub fn pimctl_set(self) -> &'a mut W {
        self.variant(PIMCTL_AW::PIMCTL_SET)
    }
}
impl W {
    #[doc = "Bit 0 - CT32BIT0 reset set"]
    #[inline(always)]
    #[must_use]
    pub fn ct32bit0(&mut self) -> CT32BIT0_W<0> {
        CT32BIT0_W::new(self)
    }
    #[doc = "Bit 1 - CT32BIT1 reset set"]
    #[inline(always)]
    #[must_use]
    pub fn ct32bit1(&mut self) -> CT32BIT1_W<1> {
        CT32BIT1_W::new(self)
    }
    #[doc = "Bit 2 - CT32BIT2 reset set"]
    #[inline(always)]
    #[must_use]
    pub fn ct32bit2(&mut self) -> CT32BIT2_W<2> {
        CT32BIT2_W::new(self)
    }
    #[doc = "Bit 3 - CT32BIT3 reset set"]
    #[inline(always)]
    #[must_use]
    pub fn ct32bit3(&mut self) -> CT32BIT3_W<3> {
        CT32BIT3_W::new(self)
    }
    #[doc = "Bit 4 - CT32BIT4 reset set"]
    #[inline(always)]
    #[must_use]
    pub fn ct32bit4(&mut self) -> CT32BIT4_W<4> {
        CT32BIT4_W::new(self)
    }
    #[doc = "Bit 8 - MRT0 reset set"]
    #[inline(always)]
    #[must_use]
    pub fn mrt0(&mut self) -> MRT0_W<8> {
        MRT0_W::new(self)
    }
    #[doc = "Bit 10 - WWDT1 reset set"]
    #[inline(always)]
    #[must_use]
    pub fn wwdt1(&mut self) -> WWDT1_W<10> {
        WWDT1_W::new(self)
    }
    #[doc = "Bit 16 - I3C0 reset set"]
    #[inline(always)]
    #[must_use]
    pub fn i3c0(&mut self) -> I3C0_W<16> {
        I3C0_W::new(self)
    }
    #[doc = "Bit 17 - I3C1 reset set"]
    #[inline(always)]
    #[must_use]
    pub fn i3c1(&mut self) -> I3C1_W<17> {
        I3C1_W::new(self)
    }
    #[doc = "Bit 30 - GPIOINTCTL reset set"]
    #[inline(always)]
    #[must_use]
    pub fn gpiointctl(&mut self) -> GPIOINTCTL_W<30> {
        GPIOINTCTL_W::new(self)
    }
    #[doc = "Bit 31 - PIMCTL reset set"]
    #[inline(always)]
    #[must_use]
    pub fn pimctl(&mut self) -> PIMCTL_W<31> {
        PIMCTL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Peripheral Reset Control Register 2 SET\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prstctl2_set](index.html) module"]
pub struct PRSTCTL2_SET_SPEC;
impl crate::RegisterSpec for PRSTCTL2_SET_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [prstctl2_set::W](W) writer structure"]
impl crate::Writable for PRSTCTL2_SET_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PRSTCTL2_SET to value 0"]
impl crate::Resettable for PRSTCTL2_SET_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
