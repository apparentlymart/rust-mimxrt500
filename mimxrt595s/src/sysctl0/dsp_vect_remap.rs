#[doc = "Register `DSP_VECT_REMAP` reader"]
pub struct R(crate::R<DSP_VECT_REMAP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DSP_VECT_REMAP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DSP_VECT_REMAP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DSP_VECT_REMAP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DSP_VECT_REMAP` writer"]
pub struct W(crate::W<DSP_VECT_REMAP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DSP_VECT_REMAP_SPEC>;
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
impl From<crate::W<DSP_VECT_REMAP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DSP_VECT_REMAP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DSP_VECT_REMAP` reader - DSP_VECT_REMAP"]
pub type DSP_VECT_REMAP_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DSP_VECT_REMAP` writer - DSP_VECT_REMAP"]
pub type DSP_VECT_REMAP_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DSP_VECT_REMAP_SPEC, u16, u16, 12, O>;
#[doc = "Field `STATVECSELECT` reader - Static Vector Select"]
pub type STATVECSELECT_R = crate::BitReader<STATVECSELECT_A>;
#[doc = "Static Vector Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STATVECSELECT_A {
    #[doc = "0: Selects the primary static vector base address on Fusion DSP (0x0000_0000)"]
    STAT_VEC_SELECT_0 = 0,
    #[doc = "1: Selects the alternate static vector base address on Fusion DSP (0x0040_0000)"]
    STAT_VEC_SELECT_1 = 1,
}
impl From<STATVECSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: STATVECSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl STATVECSELECT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STATVECSELECT_A {
        match self.bits {
            false => STATVECSELECT_A::STAT_VEC_SELECT_0,
            true => STATVECSELECT_A::STAT_VEC_SELECT_1,
        }
    }
    #[doc = "Checks if the value of the field is `STAT_VEC_SELECT_0`"]
    #[inline(always)]
    pub fn is_stat_vec_select_0(&self) -> bool {
        *self == STATVECSELECT_A::STAT_VEC_SELECT_0
    }
    #[doc = "Checks if the value of the field is `STAT_VEC_SELECT_1`"]
    #[inline(always)]
    pub fn is_stat_vec_select_1(&self) -> bool {
        *self == STATVECSELECT_A::STAT_VEC_SELECT_1
    }
}
#[doc = "Field `STATVECSELECT` writer - Static Vector Select"]
pub type STATVECSELECT_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DSP_VECT_REMAP_SPEC, STATVECSELECT_A, O>;
impl<'a, const O: u8> STATVECSELECT_W<'a, O> {
    #[doc = "Selects the primary static vector base address on Fusion DSP (0x0000_0000)"]
    #[inline(always)]
    pub fn stat_vec_select_0(self) -> &'a mut W {
        self.variant(STATVECSELECT_A::STAT_VEC_SELECT_0)
    }
    #[doc = "Selects the alternate static vector base address on Fusion DSP (0x0040_0000)"]
    #[inline(always)]
    pub fn stat_vec_select_1(self) -> &'a mut W {
        self.variant(STATVECSELECT_A::STAT_VEC_SELECT_1)
    }
}
impl R {
    #[doc = "Bits 0:11 - DSP_VECT_REMAP"]
    #[inline(always)]
    pub fn dsp_vect_remap(&self) -> DSP_VECT_REMAP_R {
        DSP_VECT_REMAP_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bit 12 - Static Vector Select"]
    #[inline(always)]
    pub fn statvecselect(&self) -> STATVECSELECT_R {
        STATVECSELECT_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11 - DSP_VECT_REMAP"]
    #[inline(always)]
    #[must_use]
    pub fn dsp_vect_remap(&mut self) -> DSP_VECT_REMAP_W<0> {
        DSP_VECT_REMAP_W::new(self)
    }
    #[doc = "Bit 12 - Static Vector Select"]
    #[inline(always)]
    #[must_use]
    pub fn statvecselect(&mut self) -> STATVECSELECT_W<12> {
        STATVECSELECT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DSP Vector Remap\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dsp_vect_remap](index.html) module"]
pub struct DSP_VECT_REMAP_SPEC;
impl crate::RegisterSpec for DSP_VECT_REMAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dsp_vect_remap::R](R) reader structure"]
impl crate::Readable for DSP_VECT_REMAP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dsp_vect_remap::W](W) writer structure"]
impl crate::Writable for DSP_VECT_REMAP_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DSP_VECT_REMAP to value 0"]
impl crate::Resettable for DSP_VECT_REMAP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
