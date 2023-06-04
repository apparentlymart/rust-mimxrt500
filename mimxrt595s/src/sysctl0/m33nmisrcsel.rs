#[doc = "Register `M33NMISRCSEL` reader"]
pub struct R(crate::R<M33NMISRCSEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<M33NMISRCSEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<M33NMISRCSEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<M33NMISRCSEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `M33NMISRCSEL` writer"]
pub struct W(crate::W<M33NMISRCSEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<M33NMISRCSEL_SPEC>;
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
impl From<crate::W<M33NMISRCSEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<M33NMISRCSEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NMISRCSEL` reader - Selects one of the M33 interrupt sources as the NMI source interrupt."]
pub type NMISRCSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `NMISRCSEL` writer - Selects one of the M33 interrupt sources as the NMI source interrupt."]
pub type NMISRCSEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, M33NMISRCSEL_SPEC, u8, u8, 7, O>;
#[doc = "Field `NMI_Enable` reader - Self Write Disable"]
pub type NMI_ENABLE_R = crate::BitReader<NMI_ENABLE_A>;
#[doc = "Self Write Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NMI_ENABLE_A {
    #[doc = "0: Disable NMI interrupt"]
    NMIENABLE_0 = 0,
    #[doc = "1: Enable NMI interrupt"]
    NMIENABLE_1 = 1,
}
impl From<NMI_ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: NMI_ENABLE_A) -> Self {
        variant as u8 != 0
    }
}
impl NMI_ENABLE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NMI_ENABLE_A {
        match self.bits {
            false => NMI_ENABLE_A::NMIENABLE_0,
            true => NMI_ENABLE_A::NMIENABLE_1,
        }
    }
    #[doc = "Checks if the value of the field is `NMIENABLE_0`"]
    #[inline(always)]
    pub fn is_nmienable_0(&self) -> bool {
        *self == NMI_ENABLE_A::NMIENABLE_0
    }
    #[doc = "Checks if the value of the field is `NMIENABLE_1`"]
    #[inline(always)]
    pub fn is_nmienable_1(&self) -> bool {
        *self == NMI_ENABLE_A::NMIENABLE_1
    }
}
#[doc = "Field `NMI_Enable` writer - Self Write Disable"]
pub type NMI_ENABLE_W<'a, const O: u8> =
    crate::BitWriter1C<'a, u32, M33NMISRCSEL_SPEC, NMI_ENABLE_A, O>;
impl<'a, const O: u8> NMI_ENABLE_W<'a, O> {
    #[doc = "Disable NMI interrupt"]
    #[inline(always)]
    pub fn nmienable_0(self) -> &'a mut W {
        self.variant(NMI_ENABLE_A::NMIENABLE_0)
    }
    #[doc = "Enable NMI interrupt"]
    #[inline(always)]
    pub fn nmienable_1(self) -> &'a mut W {
        self.variant(NMI_ENABLE_A::NMIENABLE_1)
    }
}
impl R {
    #[doc = "Bits 0:6 - Selects one of the M33 interrupt sources as the NMI source interrupt."]
    #[inline(always)]
    pub fn nmisrcsel(&self) -> NMISRCSEL_R {
        NMISRCSEL_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 31 - Self Write Disable"]
    #[inline(always)]
    pub fn nmi_enable(&self) -> NMI_ENABLE_R {
        NMI_ENABLE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - Selects one of the M33 interrupt sources as the NMI source interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn nmisrcsel(&mut self) -> NMISRCSEL_W<0> {
        NMISRCSEL_W::new(self)
    }
    #[doc = "Bit 31 - Self Write Disable"]
    #[inline(always)]
    #[must_use]
    pub fn nmi_enable(&mut self) -> NMI_ENABLE_W<31> {
        NMI_ENABLE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "M33 NMI source selection\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [m33nmisrcsel](index.html) module"]
pub struct M33NMISRCSEL_SPEC;
impl crate::RegisterSpec for M33NMISRCSEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [m33nmisrcsel::R](R) reader structure"]
impl crate::Readable for M33NMISRCSEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [m33nmisrcsel::W](W) writer structure"]
impl crate::Writable for M33NMISRCSEL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x8000_0000;
}
#[doc = "`reset()` method sets M33NMISRCSEL to value 0"]
impl crate::Resettable for M33NMISRCSEL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
