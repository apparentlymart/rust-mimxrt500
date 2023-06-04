#[doc = "Register `FLEXSPI0_REGION_RULE0` reader"]
pub struct R(crate::R<FLEXSPI0_REGION_RULE0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLEXSPI0_REGION_RULE0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLEXSPI0_REGION_RULE0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLEXSPI0_REGION_RULE0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FLEXSPI0_REGION_RULE0` writer"]
pub struct W(crate::W<FLEXSPI0_REGION_RULE0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FLEXSPI0_REGION_RULE0_SPEC>;
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
impl From<crate::W<FLEXSPI0_REGION_RULE0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FLEXSPI0_REGION_RULE0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RULE0` reader - Rule 0"]
pub type RULE0_R = crate::FieldReader<u8, RULE0_A>;
#[doc = "Rule 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RULE0_A {
    #[doc = "0: Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0,
    #[doc = "1: Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 1,
    #[doc = "2: Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 2,
    #[doc = "3: Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 3,
}
impl From<RULE0_A> for u8 {
    #[inline(always)]
    fn from(variant: RULE0_A) -> Self {
        variant as _
    }
}
impl RULE0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RULE0_A {
        match self.bits {
            0 => RULE0_A::NONSECURE_NONPRIV_USER_ALLOWED,
            1 => RULE0_A::NONSECURE_PRIV_USER_ALLOWED,
            2 => RULE0_A::SECURE_NONPRIV_USER_ALLOWED,
            3 => RULE0_A::SECURE_PRIV_USER_ALLOWED,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONSECURE_NONPRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_nonsecure_nonpriv_user_allowed(&self) -> bool {
        *self == RULE0_A::NONSECURE_NONPRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `NONSECURE_PRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_nonsecure_priv_user_allowed(&self) -> bool {
        *self == RULE0_A::NONSECURE_PRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `SECURE_NONPRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_secure_nonpriv_user_allowed(&self) -> bool {
        *self == RULE0_A::SECURE_NONPRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `SECURE_PRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_secure_priv_user_allowed(&self) -> bool {
        *self == RULE0_A::SECURE_PRIV_USER_ALLOWED
    }
}
#[doc = "Field `RULE0` writer - Rule 0"]
pub type RULE0_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, FLEXSPI0_REGION_RULE0_SPEC, u8, RULE0_A, 2, O>;
impl<'a, const O: u8> RULE0_W<'a, O> {
    #[doc = "Non-secure and non-privilege user access allowed"]
    #[inline(always)]
    pub fn nonsecure_nonpriv_user_allowed(self) -> &'a mut W {
        self.variant(RULE0_A::NONSECURE_NONPRIV_USER_ALLOWED)
    }
    #[doc = "Non-secure and privilege access allowed"]
    #[inline(always)]
    pub fn nonsecure_priv_user_allowed(self) -> &'a mut W {
        self.variant(RULE0_A::NONSECURE_PRIV_USER_ALLOWED)
    }
    #[doc = "Secure and non-privilege user access allowed"]
    #[inline(always)]
    pub fn secure_nonpriv_user_allowed(self) -> &'a mut W {
        self.variant(RULE0_A::SECURE_NONPRIV_USER_ALLOWED)
    }
    #[doc = "Secure and privilege user access allowed"]
    #[inline(always)]
    pub fn secure_priv_user_allowed(self) -> &'a mut W {
        self.variant(RULE0_A::SECURE_PRIV_USER_ALLOWED)
    }
}
#[doc = "Field `RULE1` reader - Rule 1"]
pub type RULE1_R = crate::FieldReader<u8, RULE1_A>;
#[doc = "Rule 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RULE1_A {
    #[doc = "0: Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0,
    #[doc = "1: Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 1,
    #[doc = "2: Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 2,
    #[doc = "3: Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 3,
}
impl From<RULE1_A> for u8 {
    #[inline(always)]
    fn from(variant: RULE1_A) -> Self {
        variant as _
    }
}
impl RULE1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RULE1_A {
        match self.bits {
            0 => RULE1_A::NONSECURE_NONPRIV_USER_ALLOWED,
            1 => RULE1_A::NONSECURE_PRIV_USER_ALLOWED,
            2 => RULE1_A::SECURE_NONPRIV_USER_ALLOWED,
            3 => RULE1_A::SECURE_PRIV_USER_ALLOWED,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONSECURE_NONPRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_nonsecure_nonpriv_user_allowed(&self) -> bool {
        *self == RULE1_A::NONSECURE_NONPRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `NONSECURE_PRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_nonsecure_priv_user_allowed(&self) -> bool {
        *self == RULE1_A::NONSECURE_PRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `SECURE_NONPRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_secure_nonpriv_user_allowed(&self) -> bool {
        *self == RULE1_A::SECURE_NONPRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `SECURE_PRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_secure_priv_user_allowed(&self) -> bool {
        *self == RULE1_A::SECURE_PRIV_USER_ALLOWED
    }
}
#[doc = "Field `RULE1` writer - Rule 1"]
pub type RULE1_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, FLEXSPI0_REGION_RULE0_SPEC, u8, RULE1_A, 2, O>;
impl<'a, const O: u8> RULE1_W<'a, O> {
    #[doc = "Non-secure and non-privilege user access allowed"]
    #[inline(always)]
    pub fn nonsecure_nonpriv_user_allowed(self) -> &'a mut W {
        self.variant(RULE1_A::NONSECURE_NONPRIV_USER_ALLOWED)
    }
    #[doc = "Non-secure and privilege access allowed"]
    #[inline(always)]
    pub fn nonsecure_priv_user_allowed(self) -> &'a mut W {
        self.variant(RULE1_A::NONSECURE_PRIV_USER_ALLOWED)
    }
    #[doc = "Secure and non-privilege user access allowed"]
    #[inline(always)]
    pub fn secure_nonpriv_user_allowed(self) -> &'a mut W {
        self.variant(RULE1_A::SECURE_NONPRIV_USER_ALLOWED)
    }
    #[doc = "Secure and privilege user access allowed"]
    #[inline(always)]
    pub fn secure_priv_user_allowed(self) -> &'a mut W {
        self.variant(RULE1_A::SECURE_PRIV_USER_ALLOWED)
    }
}
#[doc = "Field `RULE2` reader - Rule 2"]
pub type RULE2_R = crate::FieldReader<u8, RULE2_A>;
#[doc = "Rule 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RULE2_A {
    #[doc = "0: Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0,
    #[doc = "1: Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 1,
    #[doc = "2: Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 2,
    #[doc = "3: Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 3,
}
impl From<RULE2_A> for u8 {
    #[inline(always)]
    fn from(variant: RULE2_A) -> Self {
        variant as _
    }
}
impl RULE2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RULE2_A {
        match self.bits {
            0 => RULE2_A::NONSECURE_NONPRIV_USER_ALLOWED,
            1 => RULE2_A::NONSECURE_PRIV_USER_ALLOWED,
            2 => RULE2_A::SECURE_NONPRIV_USER_ALLOWED,
            3 => RULE2_A::SECURE_PRIV_USER_ALLOWED,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONSECURE_NONPRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_nonsecure_nonpriv_user_allowed(&self) -> bool {
        *self == RULE2_A::NONSECURE_NONPRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `NONSECURE_PRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_nonsecure_priv_user_allowed(&self) -> bool {
        *self == RULE2_A::NONSECURE_PRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `SECURE_NONPRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_secure_nonpriv_user_allowed(&self) -> bool {
        *self == RULE2_A::SECURE_NONPRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `SECURE_PRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_secure_priv_user_allowed(&self) -> bool {
        *self == RULE2_A::SECURE_PRIV_USER_ALLOWED
    }
}
#[doc = "Field `RULE2` writer - Rule 2"]
pub type RULE2_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, FLEXSPI0_REGION_RULE0_SPEC, u8, RULE2_A, 2, O>;
impl<'a, const O: u8> RULE2_W<'a, O> {
    #[doc = "Non-secure and non-privilege user access allowed"]
    #[inline(always)]
    pub fn nonsecure_nonpriv_user_allowed(self) -> &'a mut W {
        self.variant(RULE2_A::NONSECURE_NONPRIV_USER_ALLOWED)
    }
    #[doc = "Non-secure and privilege access allowed"]
    #[inline(always)]
    pub fn nonsecure_priv_user_allowed(self) -> &'a mut W {
        self.variant(RULE2_A::NONSECURE_PRIV_USER_ALLOWED)
    }
    #[doc = "Secure and non-privilege user access allowed"]
    #[inline(always)]
    pub fn secure_nonpriv_user_allowed(self) -> &'a mut W {
        self.variant(RULE2_A::SECURE_NONPRIV_USER_ALLOWED)
    }
    #[doc = "Secure and privilege user access allowed"]
    #[inline(always)]
    pub fn secure_priv_user_allowed(self) -> &'a mut W {
        self.variant(RULE2_A::SECURE_PRIV_USER_ALLOWED)
    }
}
#[doc = "Field `RULE3` reader - Rule 3"]
pub type RULE3_R = crate::FieldReader<u8, RULE3_A>;
#[doc = "Rule 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RULE3_A {
    #[doc = "0: Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0,
    #[doc = "1: Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 1,
    #[doc = "2: Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 2,
    #[doc = "3: Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 3,
}
impl From<RULE3_A> for u8 {
    #[inline(always)]
    fn from(variant: RULE3_A) -> Self {
        variant as _
    }
}
impl RULE3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RULE3_A {
        match self.bits {
            0 => RULE3_A::NONSECURE_NONPRIV_USER_ALLOWED,
            1 => RULE3_A::NONSECURE_PRIV_USER_ALLOWED,
            2 => RULE3_A::SECURE_NONPRIV_USER_ALLOWED,
            3 => RULE3_A::SECURE_PRIV_USER_ALLOWED,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONSECURE_NONPRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_nonsecure_nonpriv_user_allowed(&self) -> bool {
        *self == RULE3_A::NONSECURE_NONPRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `NONSECURE_PRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_nonsecure_priv_user_allowed(&self) -> bool {
        *self == RULE3_A::NONSECURE_PRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `SECURE_NONPRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_secure_nonpriv_user_allowed(&self) -> bool {
        *self == RULE3_A::SECURE_NONPRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `SECURE_PRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_secure_priv_user_allowed(&self) -> bool {
        *self == RULE3_A::SECURE_PRIV_USER_ALLOWED
    }
}
#[doc = "Field `RULE3` writer - Rule 3"]
pub type RULE3_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, FLEXSPI0_REGION_RULE0_SPEC, u8, RULE3_A, 2, O>;
impl<'a, const O: u8> RULE3_W<'a, O> {
    #[doc = "Non-secure and non-privilege user access allowed"]
    #[inline(always)]
    pub fn nonsecure_nonpriv_user_allowed(self) -> &'a mut W {
        self.variant(RULE3_A::NONSECURE_NONPRIV_USER_ALLOWED)
    }
    #[doc = "Non-secure and privilege access allowed"]
    #[inline(always)]
    pub fn nonsecure_priv_user_allowed(self) -> &'a mut W {
        self.variant(RULE3_A::NONSECURE_PRIV_USER_ALLOWED)
    }
    #[doc = "Secure and non-privilege user access allowed"]
    #[inline(always)]
    pub fn secure_nonpriv_user_allowed(self) -> &'a mut W {
        self.variant(RULE3_A::SECURE_NONPRIV_USER_ALLOWED)
    }
    #[doc = "Secure and privilege user access allowed"]
    #[inline(always)]
    pub fn secure_priv_user_allowed(self) -> &'a mut W {
        self.variant(RULE3_A::SECURE_PRIV_USER_ALLOWED)
    }
}
impl R {
    #[doc = "Bits 0:1 - Rule 0"]
    #[inline(always)]
    pub fn rule0(&self) -> RULE0_R {
        RULE0_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:5 - Rule 1"]
    #[inline(always)]
    pub fn rule1(&self) -> RULE1_R {
        RULE1_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Rule 2"]
    #[inline(always)]
    pub fn rule2(&self) -> RULE2_R {
        RULE2_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Rule 3"]
    #[inline(always)]
    pub fn rule3(&self) -> RULE3_R {
        RULE3_R::new(((self.bits >> 12) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Rule 0"]
    #[inline(always)]
    #[must_use]
    pub fn rule0(&mut self) -> RULE0_W<0> {
        RULE0_W::new(self)
    }
    #[doc = "Bits 4:5 - Rule 1"]
    #[inline(always)]
    #[must_use]
    pub fn rule1(&mut self) -> RULE1_W<4> {
        RULE1_W::new(self)
    }
    #[doc = "Bits 8:9 - Rule 2"]
    #[inline(always)]
    #[must_use]
    pub fn rule2(&mut self) -> RULE2_W<8> {
        RULE2_W::new(self)
    }
    #[doc = "Bits 12:13 - Rule 3"]
    #[inline(always)]
    #[must_use]
    pub fn rule3(&mut self) -> RULE3_W<12> {
        RULE3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FLEXSPI0 Region index Rule 0 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flexspi0_region_rule0](index.html) module"]
pub struct FLEXSPI0_REGION_RULE0_SPEC;
impl crate::RegisterSpec for FLEXSPI0_REGION_RULE0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [flexspi0_region_rule0::R](R) reader structure"]
impl crate::Readable for FLEXSPI0_REGION_RULE0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [flexspi0_region_rule0::W](W) writer structure"]
impl crate::Writable for FLEXSPI0_REGION_RULE0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FLEXSPI0_REGION_RULE0 to value 0"]
impl crate::Resettable for FLEXSPI0_REGION_RULE0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
