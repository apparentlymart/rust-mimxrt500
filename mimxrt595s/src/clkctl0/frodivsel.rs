#[doc = "Register `FRODIVSEL` reader"]
pub struct R(crate::R<FRODIVSEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FRODIVSEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FRODIVSEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FRODIVSEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FRODIVSEL` writer"]
pub struct W(crate::W<FRODIVSEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FRODIVSEL_SPEC>;
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
impl From<crate::W<FRODIVSEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FRODIVSEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEL` reader - Select clock"]
pub type SEL_R = crate::FieldReader<u8, SEL_A>;
#[doc = "Select clock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SEL_A {
    #[doc = "0: FRO_DIV2"]
    DIVIDEBY2 = 0,
    #[doc = "1: FRO_DIV4"]
    DIVIDEBY4 = 1,
    #[doc = "2: FRO_DIV8"]
    DIVIDEBY8 = 2,
    #[doc = "3: FRO_DIV16"]
    DIVIDEBY16 = 3,
}
impl From<SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: SEL_A) -> Self {
        variant as _
    }
}
impl SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SEL_A {
        match self.bits {
            0 => SEL_A::DIVIDEBY2,
            1 => SEL_A::DIVIDEBY4,
            2 => SEL_A::DIVIDEBY8,
            3 => SEL_A::DIVIDEBY16,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIVIDEBY2`"]
    #[inline(always)]
    pub fn is_divideby2(&self) -> bool {
        *self == SEL_A::DIVIDEBY2
    }
    #[doc = "Checks if the value of the field is `DIVIDEBY4`"]
    #[inline(always)]
    pub fn is_divideby4(&self) -> bool {
        *self == SEL_A::DIVIDEBY4
    }
    #[doc = "Checks if the value of the field is `DIVIDEBY8`"]
    #[inline(always)]
    pub fn is_divideby8(&self) -> bool {
        *self == SEL_A::DIVIDEBY8
    }
    #[doc = "Checks if the value of the field is `DIVIDEBY16`"]
    #[inline(always)]
    pub fn is_divideby16(&self) -> bool {
        *self == SEL_A::DIVIDEBY16
    }
}
#[doc = "Field `SEL` writer - Select clock"]
pub type SEL_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, FRODIVSEL_SPEC, u8, SEL_A, 2, O>;
impl<'a, const O: u8> SEL_W<'a, O> {
    #[doc = "FRO_DIV2"]
    #[inline(always)]
    pub fn divideby2(self) -> &'a mut W {
        self.variant(SEL_A::DIVIDEBY2)
    }
    #[doc = "FRO_DIV4"]
    #[inline(always)]
    pub fn divideby4(self) -> &'a mut W {
        self.variant(SEL_A::DIVIDEBY4)
    }
    #[doc = "FRO_DIV8"]
    #[inline(always)]
    pub fn divideby8(self) -> &'a mut W {
        self.variant(SEL_A::DIVIDEBY8)
    }
    #[doc = "FRO_DIV16"]
    #[inline(always)]
    pub fn divideby16(self) -> &'a mut W {
        self.variant(SEL_A::DIVIDEBY16)
    }
}
impl R {
    #[doc = "Bits 0:1 - Select clock"]
    #[inline(always)]
    pub fn sel(&self) -> SEL_R {
        SEL_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Select clock"]
    #[inline(always)]
    #[must_use]
    pub fn sel(&mut self) -> SEL_W<0> {
        SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FRO Clock Divider\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [frodivsel](index.html) module"]
pub struct FRODIVSEL_SPEC;
impl crate::RegisterSpec for FRODIVSEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [frodivsel::R](R) reader structure"]
impl crate::Readable for FRODIVSEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [frodivsel::W](W) writer structure"]
impl crate::Writable for FRODIVSEL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FRODIVSEL to value 0"]
impl crate::Resettable for FRODIVSEL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
