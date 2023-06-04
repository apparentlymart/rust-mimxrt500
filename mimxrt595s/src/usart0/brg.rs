#[doc = "Register `BRG` reader"]
pub struct R(crate::R<BRG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BRG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BRG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BRG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BRG` writer"]
pub struct W(crate::W<BRG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BRG_SPEC>;
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
impl From<crate::W<BRG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BRG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BRGVAL` reader - Baud Rate Generator Value"]
pub type BRGVAL_R = crate::FieldReader<u16, BRGVAL_A>;
#[doc = "Baud Rate Generator Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum BRGVAL_A {
    #[doc = "0: FCLK is used directly by the USART function."]
    ZERO = 0,
    #[doc = "1: FCLK is divided by 2 before use by the USART function."]
    ONE = 1,
    #[doc = "2: FCLK is divided by 3 before use by the USART function."]
    TWO = 2,
    #[doc = "65535: FCLK is divided by 65,536 before use by the USART function."]
    FFFF = 65535,
}
impl From<BRGVAL_A> for u16 {
    #[inline(always)]
    fn from(variant: BRGVAL_A) -> Self {
        variant as _
    }
}
impl BRGVAL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<BRGVAL_A> {
        match self.bits {
            0 => Some(BRGVAL_A::ZERO),
            1 => Some(BRGVAL_A::ONE),
            2 => Some(BRGVAL_A::TWO),
            65535 => Some(BRGVAL_A::FFFF),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ZERO`"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == BRGVAL_A::ZERO
    }
    #[doc = "Checks if the value of the field is `ONE`"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == BRGVAL_A::ONE
    }
    #[doc = "Checks if the value of the field is `TWO`"]
    #[inline(always)]
    pub fn is_two(&self) -> bool {
        *self == BRGVAL_A::TWO
    }
    #[doc = "Checks if the value of the field is `FFFF`"]
    #[inline(always)]
    pub fn is_ffff(&self) -> bool {
        *self == BRGVAL_A::FFFF
    }
}
#[doc = "Field `BRGVAL` writer - Baud Rate Generator Value"]
pub type BRGVAL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BRG_SPEC, u16, BRGVAL_A, 16, O>;
impl<'a, const O: u8> BRGVAL_W<'a, O> {
    #[doc = "FCLK is used directly by the USART function."]
    #[inline(always)]
    pub fn zero(self) -> &'a mut W {
        self.variant(BRGVAL_A::ZERO)
    }
    #[doc = "FCLK is divided by 2 before use by the USART function."]
    #[inline(always)]
    pub fn one(self) -> &'a mut W {
        self.variant(BRGVAL_A::ONE)
    }
    #[doc = "FCLK is divided by 3 before use by the USART function."]
    #[inline(always)]
    pub fn two(self) -> &'a mut W {
        self.variant(BRGVAL_A::TWO)
    }
    #[doc = "FCLK is divided by 65,536 before use by the USART function."]
    #[inline(always)]
    pub fn ffff(self) -> &'a mut W {
        self.variant(BRGVAL_A::FFFF)
    }
}
impl R {
    #[doc = "Bits 0:15 - Baud Rate Generator Value"]
    #[inline(always)]
    pub fn brgval(&self) -> BRGVAL_R {
        BRGVAL_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Baud Rate Generator Value"]
    #[inline(always)]
    #[must_use]
    pub fn brgval(&mut self) -> BRGVAL_W<0> {
        BRGVAL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Baud Rate Generator\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [brg](index.html) module"]
pub struct BRG_SPEC;
impl crate::RegisterSpec for BRG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [brg::R](R) reader structure"]
impl crate::Readable for BRG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [brg::W](W) writer structure"]
impl crate::Writable for BRG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BRG to value 0"]
impl crate::Resettable for BRG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
