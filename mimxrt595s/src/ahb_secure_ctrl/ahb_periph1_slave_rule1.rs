#[doc = "Register `AHB_PERIPH1_SLAVE_RULE1` reader"]
pub struct R(crate::R<AHB_PERIPH1_SLAVE_RULE1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AHB_PERIPH1_SLAVE_RULE1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AHB_PERIPH1_SLAVE_RULE1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AHB_PERIPH1_SLAVE_RULE1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AHB_PERIPH1_SLAVE_RULE1` writer"]
pub struct W(crate::W<AHB_PERIPH1_SLAVE_RULE1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AHB_PERIPH1_SLAVE_RULE1_SPEC>;
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
impl From<crate::W<AHB_PERIPH1_SLAVE_RULE1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AHB_PERIPH1_SLAVE_RULE1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FLEXCOMM16` reader - FLEXCOMM 16"]
pub type FLEXCOMM16_R = crate::FieldReader<u8, FLEXCOMM16_A>;
#[doc = "FLEXCOMM 16\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FLEXCOMM16_A {
    #[doc = "0: Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0,
    #[doc = "1: Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 1,
    #[doc = "2: Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 2,
    #[doc = "3: Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 3,
}
impl From<FLEXCOMM16_A> for u8 {
    #[inline(always)]
    fn from(variant: FLEXCOMM16_A) -> Self {
        variant as _
    }
}
impl FLEXCOMM16_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLEXCOMM16_A {
        match self.bits {
            0 => FLEXCOMM16_A::NONSECURE_NONPRIV_USER_ALLOWED,
            1 => FLEXCOMM16_A::NONSECURE_PRIV_USER_ALLOWED,
            2 => FLEXCOMM16_A::SECURE_NONPRIV_USER_ALLOWED,
            3 => FLEXCOMM16_A::SECURE_PRIV_USER_ALLOWED,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONSECURE_NONPRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_nonsecure_nonpriv_user_allowed(&self) -> bool {
        *self == FLEXCOMM16_A::NONSECURE_NONPRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `NONSECURE_PRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_nonsecure_priv_user_allowed(&self) -> bool {
        *self == FLEXCOMM16_A::NONSECURE_PRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `SECURE_NONPRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_secure_nonpriv_user_allowed(&self) -> bool {
        *self == FLEXCOMM16_A::SECURE_NONPRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `SECURE_PRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_secure_priv_user_allowed(&self) -> bool {
        *self == FLEXCOMM16_A::SECURE_PRIV_USER_ALLOWED
    }
}
#[doc = "Field `FLEXCOMM16` writer - FLEXCOMM 16"]
pub type FLEXCOMM16_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, AHB_PERIPH1_SLAVE_RULE1_SPEC, u8, FLEXCOMM16_A, 2, O>;
impl<'a, const O: u8> FLEXCOMM16_W<'a, O> {
    #[doc = "Non-secure and non-privilege user access allowed"]
    #[inline(always)]
    pub fn nonsecure_nonpriv_user_allowed(self) -> &'a mut W {
        self.variant(FLEXCOMM16_A::NONSECURE_NONPRIV_USER_ALLOWED)
    }
    #[doc = "Non-secure and privilege access allowed"]
    #[inline(always)]
    pub fn nonsecure_priv_user_allowed(self) -> &'a mut W {
        self.variant(FLEXCOMM16_A::NONSECURE_PRIV_USER_ALLOWED)
    }
    #[doc = "Secure and non-privilege user access allowed"]
    #[inline(always)]
    pub fn secure_nonpriv_user_allowed(self) -> &'a mut W {
        self.variant(FLEXCOMM16_A::SECURE_NONPRIV_USER_ALLOWED)
    }
    #[doc = "Secure and privilege user access allowed"]
    #[inline(always)]
    pub fn secure_priv_user_allowed(self) -> &'a mut W {
        self.variant(FLEXCOMM16_A::SECURE_PRIV_USER_ALLOWED)
    }
}
impl R {
    #[doc = "Bits 0:1 - FLEXCOMM 16"]
    #[inline(always)]
    pub fn flexcomm16(&self) -> FLEXCOMM16_R {
        FLEXCOMM16_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - FLEXCOMM 16"]
    #[inline(always)]
    #[must_use]
    pub fn flexcomm16(&mut self) -> FLEXCOMM16_W<0> {
        FLEXCOMM16_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AHB Peripheral 1 Slave Rule 1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ahb_periph1_slave_rule1](index.html) module"]
pub struct AHB_PERIPH1_SLAVE_RULE1_SPEC;
impl crate::RegisterSpec for AHB_PERIPH1_SLAVE_RULE1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ahb_periph1_slave_rule1::R](R) reader structure"]
impl crate::Readable for AHB_PERIPH1_SLAVE_RULE1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ahb_periph1_slave_rule1::W](W) writer structure"]
impl crate::Writable for AHB_PERIPH1_SLAVE_RULE1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AHB_PERIPH1_SLAVE_RULE1 to value 0"]
impl crate::Resettable for AHB_PERIPH1_SLAVE_RULE1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
