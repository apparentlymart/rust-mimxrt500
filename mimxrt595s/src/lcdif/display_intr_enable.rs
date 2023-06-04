#[doc = "Register `DisplayIntrEnable` reader"]
pub struct R(crate::R<DISPLAY_INTR_ENABLE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DISPLAY_INTR_ENABLE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DISPLAY_INTR_ENABLE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DISPLAY_INTR_ENABLE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DisplayIntrEnable` writer"]
pub struct W(crate::W<DISPLAY_INTR_ENABLE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DISPLAY_INTR_ENABLE_SPEC>;
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
impl From<crate::W<DISPLAY_INTR_ENABLE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DISPLAY_INTR_ENABLE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DISP0` reader - Display0 Interrupt Enable"]
pub type DISP0_R = crate::BitReader<bool>;
#[doc = "Field `DISP0` writer - Display0 Interrupt Enable"]
pub type DISP0_W<'a, const O: u8> = crate::BitWriter<'a, u32, DISPLAY_INTR_ENABLE_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Display0 Interrupt Enable"]
    #[inline(always)]
    pub fn disp0(&self) -> DISP0_R {
        DISP0_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Display0 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn disp0(&mut self) -> DISP0_W<0> {
        DISP0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Enable for Display_0 (and Display_1 if present)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [display_intr_enable](index.html) module"]
pub struct DISPLAY_INTR_ENABLE_SPEC;
impl crate::RegisterSpec for DISPLAY_INTR_ENABLE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [display_intr_enable::R](R) reader structure"]
impl crate::Readable for DISPLAY_INTR_ENABLE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [display_intr_enable::W](W) writer structure"]
impl crate::Writable for DISPLAY_INTR_ENABLE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DisplayIntrEnable to value 0x11"]
impl crate::Resettable for DISPLAY_INTR_ENABLE_SPEC {
    const RESET_VALUE: Self::Ux = 0x11;
}
