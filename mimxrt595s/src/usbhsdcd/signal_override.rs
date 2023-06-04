#[doc = "Register `SIGNAL_OVERRIDE` reader"]
pub struct R(crate::R<SIGNAL_OVERRIDE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SIGNAL_OVERRIDE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SIGNAL_OVERRIDE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SIGNAL_OVERRIDE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SIGNAL_OVERRIDE` writer"]
pub struct W(crate::W<SIGNAL_OVERRIDE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SIGNAL_OVERRIDE_SPEC>;
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
impl From<crate::W<SIGNAL_OVERRIDE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SIGNAL_OVERRIDE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PS` reader - Phase Selection"]
pub type PS_R = crate::FieldReader<u8, PS_A>;
#[doc = "Phase Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PS_A {
    #[doc = "0: No overrides. Bit field must remain at this value during normal USB data communication to prevent unexpected conditions on USB_DP and USB_DM pins. (Default)"]
    NO_OVERRIDE = 0,
    #[doc = "2: Enables VDP_SRC voltage source for the USB_DP pin and IDM_SINK current source for the USB_DM pin."]
    PRI_DET_OVERRIDE = 2,
}
impl From<PS_A> for u8 {
    #[inline(always)]
    fn from(variant: PS_A) -> Self {
        variant as _
    }
}
impl PS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PS_A> {
        match self.bits {
            0 => Some(PS_A::NO_OVERRIDE),
            2 => Some(PS_A::PRI_DET_OVERRIDE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NO_OVERRIDE`"]
    #[inline(always)]
    pub fn is_no_override(&self) -> bool {
        *self == PS_A::NO_OVERRIDE
    }
    #[doc = "Checks if the value of the field is `PRI_DET_OVERRIDE`"]
    #[inline(always)]
    pub fn is_pri_det_override(&self) -> bool {
        *self == PS_A::PRI_DET_OVERRIDE
    }
}
#[doc = "Field `PS` writer - Phase Selection"]
pub type PS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SIGNAL_OVERRIDE_SPEC, u8, PS_A, 2, O>;
impl<'a, const O: u8> PS_W<'a, O> {
    #[doc = "No overrides. Bit field must remain at this value during normal USB data communication to prevent unexpected conditions on USB_DP and USB_DM pins. (Default)"]
    #[inline(always)]
    pub fn no_override(self) -> &'a mut W {
        self.variant(PS_A::NO_OVERRIDE)
    }
    #[doc = "Enables VDP_SRC voltage source for the USB_DP pin and IDM_SINK current source for the USB_DM pin."]
    #[inline(always)]
    pub fn pri_det_override(self) -> &'a mut W {
        self.variant(PS_A::PRI_DET_OVERRIDE)
    }
}
impl R {
    #[doc = "Bits 0:1 - Phase Selection"]
    #[inline(always)]
    pub fn ps(&self) -> PS_R {
        PS_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Phase Selection"]
    #[inline(always)]
    #[must_use]
    pub fn ps(&mut self) -> PS_W<0> {
        PS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Signal Override\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [signal_override](index.html) module"]
pub struct SIGNAL_OVERRIDE_SPEC;
impl crate::RegisterSpec for SIGNAL_OVERRIDE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [signal_override::R](R) reader structure"]
impl crate::Readable for SIGNAL_OVERRIDE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [signal_override::W](W) writer structure"]
impl crate::Writable for SIGNAL_OVERRIDE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SIGNAL_OVERRIDE to value 0"]
impl crate::Resettable for SIGNAL_OVERRIDE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
