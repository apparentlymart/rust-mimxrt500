#[doc = "Register `PDWAKECFG` reader"]
pub struct R(crate::R<PDWAKECFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PDWAKECFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PDWAKECFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PDWAKECFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PDWAKECFG` writer"]
pub struct W(crate::W<PDWAKECFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PDWAKECFG_SPEC>;
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
impl From<crate::W<PDWAKECFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PDWAKECFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RBBKEEPST` reader - RBB mode on wakeup"]
pub type RBBKEEPST_R = crate::BitReader<RBBKEEPST_A>;
#[doc = "RBB mode on wakeup\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RBBKEEPST_A {
    #[doc = "0: Use value of RBB_PD in PDRUNCFG on wakeup."]
    RBBKEEPST_0 = 0,
    #[doc = "1: Copy PDSLEEPCFG RBB_PD value to PDRUNCFG RBB_PD on wakeup to keep RBB state."]
    RBBKEEPST_1 = 1,
}
impl From<RBBKEEPST_A> for bool {
    #[inline(always)]
    fn from(variant: RBBKEEPST_A) -> Self {
        variant as u8 != 0
    }
}
impl RBBKEEPST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RBBKEEPST_A {
        match self.bits {
            false => RBBKEEPST_A::RBBKEEPST_0,
            true => RBBKEEPST_A::RBBKEEPST_1,
        }
    }
    #[doc = "Checks if the value of the field is `RBBKEEPST_0`"]
    #[inline(always)]
    pub fn is_rbbkeepst_0(&self) -> bool {
        *self == RBBKEEPST_A::RBBKEEPST_0
    }
    #[doc = "Checks if the value of the field is `RBBKEEPST_1`"]
    #[inline(always)]
    pub fn is_rbbkeepst_1(&self) -> bool {
        *self == RBBKEEPST_A::RBBKEEPST_1
    }
}
#[doc = "Field `RBBKEEPST` writer - RBB mode on wakeup"]
pub type RBBKEEPST_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDWAKECFG_SPEC, RBBKEEPST_A, O>;
impl<'a, const O: u8> RBBKEEPST_W<'a, O> {
    #[doc = "Use value of RBB_PD in PDRUNCFG on wakeup."]
    #[inline(always)]
    pub fn rbbkeepst_0(self) -> &'a mut W {
        self.variant(RBBKEEPST_A::RBBKEEPST_0)
    }
    #[doc = "Copy PDSLEEPCFG RBB_PD value to PDRUNCFG RBB_PD on wakeup to keep RBB state."]
    #[inline(always)]
    pub fn rbbkeepst_1(self) -> &'a mut W {
        self.variant(RBBKEEPST_A::RBBKEEPST_1)
    }
}
#[doc = "Field `FBBKEEPST` reader - FBB mode on wakeup"]
pub type FBBKEEPST_R = crate::BitReader<FBBKEEPST_A>;
#[doc = "FBB mode on wakeup\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FBBKEEPST_A {
    #[doc = "0: Use value of FBB_PD in PDRUNCFG on wakeup"]
    FBBKEEPST_0 = 0,
    #[doc = "1: Copy PDSLEEPCFG FBB_PD value to PDRUNCFG FBB_PD on wakeup to keep FBB state"]
    FBBKEEPST_1 = 1,
}
impl From<FBBKEEPST_A> for bool {
    #[inline(always)]
    fn from(variant: FBBKEEPST_A) -> Self {
        variant as u8 != 0
    }
}
impl FBBKEEPST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FBBKEEPST_A {
        match self.bits {
            false => FBBKEEPST_A::FBBKEEPST_0,
            true => FBBKEEPST_A::FBBKEEPST_1,
        }
    }
    #[doc = "Checks if the value of the field is `FBBKEEPST_0`"]
    #[inline(always)]
    pub fn is_fbbkeepst_0(&self) -> bool {
        *self == FBBKEEPST_A::FBBKEEPST_0
    }
    #[doc = "Checks if the value of the field is `FBBKEEPST_1`"]
    #[inline(always)]
    pub fn is_fbbkeepst_1(&self) -> bool {
        *self == FBBKEEPST_A::FBBKEEPST_1
    }
}
#[doc = "Field `FBBKEEPST` writer - FBB mode on wakeup"]
pub type FBBKEEPST_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDWAKECFG_SPEC, FBBKEEPST_A, O>;
impl<'a, const O: u8> FBBKEEPST_W<'a, O> {
    #[doc = "Use value of FBB_PD in PDRUNCFG on wakeup"]
    #[inline(always)]
    pub fn fbbkeepst_0(self) -> &'a mut W {
        self.variant(FBBKEEPST_A::FBBKEEPST_0)
    }
    #[doc = "Copy PDSLEEPCFG FBB_PD value to PDRUNCFG FBB_PD on wakeup to keep FBB state"]
    #[inline(always)]
    pub fn fbbkeepst_1(self) -> &'a mut W {
        self.variant(FBBKEEPST_A::FBBKEEPST_1)
    }
}
#[doc = "Field `RBBSRAMKEEPST` reader - RBB SRAM mode on wakeup"]
pub type RBBSRAMKEEPST_R = crate::BitReader<RBBSRAMKEEPST_A>;
#[doc = "RBB SRAM mode on wakeup\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RBBSRAMKEEPST_A {
    #[doc = "0: Use value of RBBSRAM_PD in PDRUNCFG on wakeup"]
    RBBSRAMKEEPST_0 = 0,
    #[doc = "1: Copy PDSLEEPCFG RBBSRAM_PD value to PDRUNCFG RBBSRAM_PD on wakeupto keep RBB state"]
    RBBSRAMKEEPST_1 = 1,
}
impl From<RBBSRAMKEEPST_A> for bool {
    #[inline(always)]
    fn from(variant: RBBSRAMKEEPST_A) -> Self {
        variant as u8 != 0
    }
}
impl RBBSRAMKEEPST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RBBSRAMKEEPST_A {
        match self.bits {
            false => RBBSRAMKEEPST_A::RBBSRAMKEEPST_0,
            true => RBBSRAMKEEPST_A::RBBSRAMKEEPST_1,
        }
    }
    #[doc = "Checks if the value of the field is `RBBSRAMKEEPST_0`"]
    #[inline(always)]
    pub fn is_rbbsramkeepst_0(&self) -> bool {
        *self == RBBSRAMKEEPST_A::RBBSRAMKEEPST_0
    }
    #[doc = "Checks if the value of the field is `RBBSRAMKEEPST_1`"]
    #[inline(always)]
    pub fn is_rbbsramkeepst_1(&self) -> bool {
        *self == RBBSRAMKEEPST_A::RBBSRAMKEEPST_1
    }
}
#[doc = "Field `RBBSRAMKEEPST` writer - RBB SRAM mode on wakeup"]
pub type RBBSRAMKEEPST_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PDWAKECFG_SPEC, RBBSRAMKEEPST_A, O>;
impl<'a, const O: u8> RBBSRAMKEEPST_W<'a, O> {
    #[doc = "Use value of RBBSRAM_PD in PDRUNCFG on wakeup"]
    #[inline(always)]
    pub fn rbbsramkeepst_0(self) -> &'a mut W {
        self.variant(RBBSRAMKEEPST_A::RBBSRAMKEEPST_0)
    }
    #[doc = "Copy PDSLEEPCFG RBBSRAM_PD value to PDRUNCFG RBBSRAM_PD on wakeupto keep RBB state"]
    #[inline(always)]
    pub fn rbbsramkeepst_1(self) -> &'a mut W {
        self.variant(RBBSRAMKEEPST_A::RBBSRAMKEEPST_1)
    }
}
#[doc = "Field `MIPIPDKEEPST` reader - MIPI_PD value on wakeup"]
pub type MIPIPDKEEPST_R = crate::BitReader<MIPIPDKEEPST_A>;
#[doc = "MIPI_PD value on wakeup\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MIPIPDKEEPST_A {
    #[doc = "0: Use value of MIPI_PD in PDRUNCFG on wakeup"]
    MIPIPDKEEPST_0 = 0,
    #[doc = "1: Copy PDSLEEPCFG MIPI_PD value to PDRUNCFG MIPI_PD on wakeupto keep MIPI state"]
    MIPIPDKEEPST_1 = 1,
}
impl From<MIPIPDKEEPST_A> for bool {
    #[inline(always)]
    fn from(variant: MIPIPDKEEPST_A) -> Self {
        variant as u8 != 0
    }
}
impl MIPIPDKEEPST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MIPIPDKEEPST_A {
        match self.bits {
            false => MIPIPDKEEPST_A::MIPIPDKEEPST_0,
            true => MIPIPDKEEPST_A::MIPIPDKEEPST_1,
        }
    }
    #[doc = "Checks if the value of the field is `MIPIPDKEEPST_0`"]
    #[inline(always)]
    pub fn is_mipipdkeepst_0(&self) -> bool {
        *self == MIPIPDKEEPST_A::MIPIPDKEEPST_0
    }
    #[doc = "Checks if the value of the field is `MIPIPDKEEPST_1`"]
    #[inline(always)]
    pub fn is_mipipdkeepst_1(&self) -> bool {
        *self == MIPIPDKEEPST_A::MIPIPDKEEPST_1
    }
}
#[doc = "Field `MIPIPDKEEPST` writer - MIPI_PD value on wakeup"]
pub type MIPIPDKEEPST_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PDWAKECFG_SPEC, MIPIPDKEEPST_A, O>;
impl<'a, const O: u8> MIPIPDKEEPST_W<'a, O> {
    #[doc = "Use value of MIPI_PD in PDRUNCFG on wakeup"]
    #[inline(always)]
    pub fn mipipdkeepst_0(self) -> &'a mut W {
        self.variant(MIPIPDKEEPST_A::MIPIPDKEEPST_0)
    }
    #[doc = "Copy PDSLEEPCFG MIPI_PD value to PDRUNCFG MIPI_PD on wakeupto keep MIPI state"]
    #[inline(always)]
    pub fn mipipdkeepst_1(self) -> &'a mut W {
        self.variant(MIPIPDKEEPST_A::MIPIPDKEEPST_1)
    }
}
#[doc = "Field `DSPPDKEEPST` reader - DSP_PD value on wakeup"]
pub type DSPPDKEEPST_R = crate::BitReader<DSPPDKEEPST_A>;
#[doc = "DSP_PD value on wakeup\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DSPPDKEEPST_A {
    #[doc = "0: Use value of DSP_PD in PDRUNCFG on wakeup"]
    DSPPDKEEPST_0 = 0,
    #[doc = "1: Copy PDSLEEPCFG DSP_PD value to PDRUNCFG DSP_PD on wakeupto keep DSP state"]
    DSPPDKEEPST_1 = 1,
}
impl From<DSPPDKEEPST_A> for bool {
    #[inline(always)]
    fn from(variant: DSPPDKEEPST_A) -> Self {
        variant as u8 != 0
    }
}
impl DSPPDKEEPST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DSPPDKEEPST_A {
        match self.bits {
            false => DSPPDKEEPST_A::DSPPDKEEPST_0,
            true => DSPPDKEEPST_A::DSPPDKEEPST_1,
        }
    }
    #[doc = "Checks if the value of the field is `DSPPDKEEPST_0`"]
    #[inline(always)]
    pub fn is_dsppdkeepst_0(&self) -> bool {
        *self == DSPPDKEEPST_A::DSPPDKEEPST_0
    }
    #[doc = "Checks if the value of the field is `DSPPDKEEPST_1`"]
    #[inline(always)]
    pub fn is_dsppdkeepst_1(&self) -> bool {
        *self == DSPPDKEEPST_A::DSPPDKEEPST_1
    }
}
#[doc = "Field `DSPPDKEEPST` writer - DSP_PD value on wakeup"]
pub type DSPPDKEEPST_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PDWAKECFG_SPEC, DSPPDKEEPST_A, O>;
impl<'a, const O: u8> DSPPDKEEPST_W<'a, O> {
    #[doc = "Use value of DSP_PD in PDRUNCFG on wakeup"]
    #[inline(always)]
    pub fn dsppdkeepst_0(self) -> &'a mut W {
        self.variant(DSPPDKEEPST_A::DSPPDKEEPST_0)
    }
    #[doc = "Copy PDSLEEPCFG DSP_PD value to PDRUNCFG DSP_PD on wakeupto keep DSP state"]
    #[inline(always)]
    pub fn dsppdkeepst_1(self) -> &'a mut W {
        self.variant(DSPPDKEEPST_A::DSPPDKEEPST_1)
    }
}
#[doc = "Field `OTPPDKEEPST` reader - OTP_PD value on wakeup"]
pub type OTPPDKEEPST_R = crate::BitReader<OTPPDKEEPST_A>;
#[doc = "OTP_PD value on wakeup\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OTPPDKEEPST_A {
    #[doc = "0: Use value of OTP_PD in PDRUNCFG on wakeup"]
    OTPPDKEEPST_0 = 0,
    #[doc = "1: Copy PDSLEEPCFG OTP_PD value to PDRUNCFG OTP_PD on wakeupto keep OTP state"]
    OTPPDKEEPST_1 = 1,
}
impl From<OTPPDKEEPST_A> for bool {
    #[inline(always)]
    fn from(variant: OTPPDKEEPST_A) -> Self {
        variant as u8 != 0
    }
}
impl OTPPDKEEPST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OTPPDKEEPST_A {
        match self.bits {
            false => OTPPDKEEPST_A::OTPPDKEEPST_0,
            true => OTPPDKEEPST_A::OTPPDKEEPST_1,
        }
    }
    #[doc = "Checks if the value of the field is `OTPPDKEEPST_0`"]
    #[inline(always)]
    pub fn is_otppdkeepst_0(&self) -> bool {
        *self == OTPPDKEEPST_A::OTPPDKEEPST_0
    }
    #[doc = "Checks if the value of the field is `OTPPDKEEPST_1`"]
    #[inline(always)]
    pub fn is_otppdkeepst_1(&self) -> bool {
        *self == OTPPDKEEPST_A::OTPPDKEEPST_1
    }
}
#[doc = "Field `OTPPDKEEPST` writer - OTP_PD value on wakeup"]
pub type OTPPDKEEPST_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PDWAKECFG_SPEC, OTPPDKEEPST_A, O>;
impl<'a, const O: u8> OTPPDKEEPST_W<'a, O> {
    #[doc = "Use value of OTP_PD in PDRUNCFG on wakeup"]
    #[inline(always)]
    pub fn otppdkeepst_0(self) -> &'a mut W {
        self.variant(OTPPDKEEPST_A::OTPPDKEEPST_0)
    }
    #[doc = "Copy PDSLEEPCFG OTP_PD value to PDRUNCFG OTP_PD on wakeupto keep OTP state"]
    #[inline(always)]
    pub fn otppdkeepst_1(self) -> &'a mut W {
        self.variant(OTPPDKEEPST_A::OTPPDKEEPST_1)
    }
}
impl R {
    #[doc = "Bit 0 - RBB mode on wakeup"]
    #[inline(always)]
    pub fn rbbkeepst(&self) -> RBBKEEPST_R {
        RBBKEEPST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - FBB mode on wakeup"]
    #[inline(always)]
    pub fn fbbkeepst(&self) -> FBBKEEPST_R {
        FBBKEEPST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RBB SRAM mode on wakeup"]
    #[inline(always)]
    pub fn rbbsramkeepst(&self) -> RBBSRAMKEEPST_R {
        RBBSRAMKEEPST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - MIPI_PD value on wakeup"]
    #[inline(always)]
    pub fn mipipdkeepst(&self) -> MIPIPDKEEPST_R {
        MIPIPDKEEPST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - DSP_PD value on wakeup"]
    #[inline(always)]
    pub fn dsppdkeepst(&self) -> DSPPDKEEPST_R {
        DSPPDKEEPST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - OTP_PD value on wakeup"]
    #[inline(always)]
    pub fn otppdkeepst(&self) -> OTPPDKEEPST_R {
        OTPPDKEEPST_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RBB mode on wakeup"]
    #[inline(always)]
    #[must_use]
    pub fn rbbkeepst(&mut self) -> RBBKEEPST_W<0> {
        RBBKEEPST_W::new(self)
    }
    #[doc = "Bit 1 - FBB mode on wakeup"]
    #[inline(always)]
    #[must_use]
    pub fn fbbkeepst(&mut self) -> FBBKEEPST_W<1> {
        FBBKEEPST_W::new(self)
    }
    #[doc = "Bit 2 - RBB SRAM mode on wakeup"]
    #[inline(always)]
    #[must_use]
    pub fn rbbsramkeepst(&mut self) -> RBBSRAMKEEPST_W<2> {
        RBBSRAMKEEPST_W::new(self)
    }
    #[doc = "Bit 3 - MIPI_PD value on wakeup"]
    #[inline(always)]
    #[must_use]
    pub fn mipipdkeepst(&mut self) -> MIPIPDKEEPST_W<3> {
        MIPIPDKEEPST_W::new(self)
    }
    #[doc = "Bit 4 - DSP_PD value on wakeup"]
    #[inline(always)]
    #[must_use]
    pub fn dsppdkeepst(&mut self) -> DSPPDKEEPST_W<4> {
        DSPPDKEEPST_W::new(self)
    }
    #[doc = "Bit 5 - OTP_PD value on wakeup"]
    #[inline(always)]
    #[must_use]
    pub fn otppdkeepst(&mut self) -> OTPPDKEEPST_W<5> {
        OTPPDKEEPST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PD Wake Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdwakecfg](index.html) module"]
pub struct PDWAKECFG_SPEC;
impl crate::RegisterSpec for PDWAKECFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pdwakecfg::R](R) reader structure"]
impl crate::Readable for PDWAKECFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pdwakecfg::W](W) writer structure"]
impl crate::Writable for PDWAKECFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PDWAKECFG to value 0"]
impl crate::Resettable for PDWAKECFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
