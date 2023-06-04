#[doc = "Register `AIPS_BRIDGE1_PER_RULE1` reader"]
pub struct R(crate::R<AIPS_BRIDGE1_PER_RULE1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AIPS_BRIDGE1_PER_RULE1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AIPS_BRIDGE1_PER_RULE1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AIPS_BRIDGE1_PER_RULE1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AIPS_BRIDGE1_PER_RULE1` writer"]
pub struct W(crate::W<AIPS_BRIDGE1_PER_RULE1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AIPS_BRIDGE1_PER_RULE1_SPEC>;
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
impl From<crate::W<AIPS_BRIDGE1_PER_RULE1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AIPS_BRIDGE1_PER_RULE1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RNG` reader - RNG (Random Number Generator)"]
pub type RNG_R = crate::FieldReader<u8, RNG_A>;
#[doc = "RNG (Random Number Generator)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RNG_A {
    #[doc = "0: Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0,
    #[doc = "1: Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 1,
    #[doc = "2: Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 2,
    #[doc = "3: Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 3,
}
impl From<RNG_A> for u8 {
    #[inline(always)]
    fn from(variant: RNG_A) -> Self {
        variant as _
    }
}
impl RNG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RNG_A {
        match self.bits {
            0 => RNG_A::NONSECURE_NONPRIV_USER_ALLOWED,
            1 => RNG_A::NONSECURE_PRIV_USER_ALLOWED,
            2 => RNG_A::SECURE_NONPRIV_USER_ALLOWED,
            3 => RNG_A::SECURE_PRIV_USER_ALLOWED,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONSECURE_NONPRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_nonsecure_nonpriv_user_allowed(&self) -> bool {
        *self == RNG_A::NONSECURE_NONPRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `NONSECURE_PRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_nonsecure_priv_user_allowed(&self) -> bool {
        *self == RNG_A::NONSECURE_PRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `SECURE_NONPRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_secure_nonpriv_user_allowed(&self) -> bool {
        *self == RNG_A::SECURE_NONPRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `SECURE_PRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_secure_priv_user_allowed(&self) -> bool {
        *self == RNG_A::SECURE_PRIV_USER_ALLOWED
    }
}
#[doc = "Field `RNG` writer - RNG (Random Number Generator)"]
pub type RNG_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, AIPS_BRIDGE1_PER_RULE1_SPEC, u8, RNG_A, 2, O>;
impl<'a, const O: u8> RNG_W<'a, O> {
    #[doc = "Non-secure and non-privilege user access allowed"]
    #[inline(always)]
    pub fn nonsecure_nonpriv_user_allowed(self) -> &'a mut W {
        self.variant(RNG_A::NONSECURE_NONPRIV_USER_ALLOWED)
    }
    #[doc = "Non-secure and privilege access allowed"]
    #[inline(always)]
    pub fn nonsecure_priv_user_allowed(self) -> &'a mut W {
        self.variant(RNG_A::NONSECURE_PRIV_USER_ALLOWED)
    }
    #[doc = "Secure and non-privilege user access allowed"]
    #[inline(always)]
    pub fn secure_nonpriv_user_allowed(self) -> &'a mut W {
        self.variant(RNG_A::SECURE_NONPRIV_USER_ALLOWED)
    }
    #[doc = "Secure and privilege user access allowed"]
    #[inline(always)]
    pub fn secure_priv_user_allowed(self) -> &'a mut W {
        self.variant(RNG_A::SECURE_PRIV_USER_ALLOWED)
    }
}
#[doc = "Field `ACMP0` reader - ACMP 0"]
pub type ACMP0_R = crate::FieldReader<u8, ACMP0_A>;
#[doc = "ACMP 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ACMP0_A {
    #[doc = "0: Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0,
    #[doc = "1: Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 1,
    #[doc = "2: Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 2,
    #[doc = "3: Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 3,
}
impl From<ACMP0_A> for u8 {
    #[inline(always)]
    fn from(variant: ACMP0_A) -> Self {
        variant as _
    }
}
impl ACMP0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACMP0_A {
        match self.bits {
            0 => ACMP0_A::NONSECURE_NONPRIV_USER_ALLOWED,
            1 => ACMP0_A::NONSECURE_PRIV_USER_ALLOWED,
            2 => ACMP0_A::SECURE_NONPRIV_USER_ALLOWED,
            3 => ACMP0_A::SECURE_PRIV_USER_ALLOWED,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONSECURE_NONPRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_nonsecure_nonpriv_user_allowed(&self) -> bool {
        *self == ACMP0_A::NONSECURE_NONPRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `NONSECURE_PRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_nonsecure_priv_user_allowed(&self) -> bool {
        *self == ACMP0_A::NONSECURE_PRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `SECURE_NONPRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_secure_nonpriv_user_allowed(&self) -> bool {
        *self == ACMP0_A::SECURE_NONPRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `SECURE_PRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_secure_priv_user_allowed(&self) -> bool {
        *self == ACMP0_A::SECURE_PRIV_USER_ALLOWED
    }
}
#[doc = "Field `ACMP0` writer - ACMP 0"]
pub type ACMP0_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, AIPS_BRIDGE1_PER_RULE1_SPEC, u8, ACMP0_A, 2, O>;
impl<'a, const O: u8> ACMP0_W<'a, O> {
    #[doc = "Non-secure and non-privilege user access allowed"]
    #[inline(always)]
    pub fn nonsecure_nonpriv_user_allowed(self) -> &'a mut W {
        self.variant(ACMP0_A::NONSECURE_NONPRIV_USER_ALLOWED)
    }
    #[doc = "Non-secure and privilege access allowed"]
    #[inline(always)]
    pub fn nonsecure_priv_user_allowed(self) -> &'a mut W {
        self.variant(ACMP0_A::NONSECURE_PRIV_USER_ALLOWED)
    }
    #[doc = "Secure and non-privilege user access allowed"]
    #[inline(always)]
    pub fn secure_nonpriv_user_allowed(self) -> &'a mut W {
        self.variant(ACMP0_A::SECURE_NONPRIV_USER_ALLOWED)
    }
    #[doc = "Secure and privilege user access allowed"]
    #[inline(always)]
    pub fn secure_priv_user_allowed(self) -> &'a mut W {
        self.variant(ACMP0_A::SECURE_PRIV_USER_ALLOWED)
    }
}
#[doc = "Field `ADC0` reader - ADC 0"]
pub type ADC0_R = crate::FieldReader<u8, ADC0_A>;
#[doc = "ADC 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ADC0_A {
    #[doc = "0: Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0,
    #[doc = "1: Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 1,
    #[doc = "2: Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 2,
    #[doc = "3: Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 3,
}
impl From<ADC0_A> for u8 {
    #[inline(always)]
    fn from(variant: ADC0_A) -> Self {
        variant as _
    }
}
impl ADC0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC0_A {
        match self.bits {
            0 => ADC0_A::NONSECURE_NONPRIV_USER_ALLOWED,
            1 => ADC0_A::NONSECURE_PRIV_USER_ALLOWED,
            2 => ADC0_A::SECURE_NONPRIV_USER_ALLOWED,
            3 => ADC0_A::SECURE_PRIV_USER_ALLOWED,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONSECURE_NONPRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_nonsecure_nonpriv_user_allowed(&self) -> bool {
        *self == ADC0_A::NONSECURE_NONPRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `NONSECURE_PRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_nonsecure_priv_user_allowed(&self) -> bool {
        *self == ADC0_A::NONSECURE_PRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `SECURE_NONPRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_secure_nonpriv_user_allowed(&self) -> bool {
        *self == ADC0_A::SECURE_NONPRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `SECURE_PRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_secure_priv_user_allowed(&self) -> bool {
        *self == ADC0_A::SECURE_PRIV_USER_ALLOWED
    }
}
#[doc = "Field `ADC0` writer - ADC 0"]
pub type ADC0_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, AIPS_BRIDGE1_PER_RULE1_SPEC, u8, ADC0_A, 2, O>;
impl<'a, const O: u8> ADC0_W<'a, O> {
    #[doc = "Non-secure and non-privilege user access allowed"]
    #[inline(always)]
    pub fn nonsecure_nonpriv_user_allowed(self) -> &'a mut W {
        self.variant(ADC0_A::NONSECURE_NONPRIV_USER_ALLOWED)
    }
    #[doc = "Non-secure and privilege access allowed"]
    #[inline(always)]
    pub fn nonsecure_priv_user_allowed(self) -> &'a mut W {
        self.variant(ADC0_A::NONSECURE_PRIV_USER_ALLOWED)
    }
    #[doc = "Secure and non-privilege user access allowed"]
    #[inline(always)]
    pub fn secure_nonpriv_user_allowed(self) -> &'a mut W {
        self.variant(ADC0_A::SECURE_NONPRIV_USER_ALLOWED)
    }
    #[doc = "Secure and privilege user access allowed"]
    #[inline(always)]
    pub fn secure_priv_user_allowed(self) -> &'a mut W {
        self.variant(ADC0_A::SECURE_PRIV_USER_ALLOWED)
    }
}
#[doc = "Field `HS_USB_PHY` reader - HS USB PHY"]
pub type HS_USB_PHY_R = crate::FieldReader<u8, HS_USB_PHY_A>;
#[doc = "HS USB PHY\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum HS_USB_PHY_A {
    #[doc = "0: Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0,
    #[doc = "1: Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 1,
    #[doc = "2: Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 2,
    #[doc = "3: Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 3,
}
impl From<HS_USB_PHY_A> for u8 {
    #[inline(always)]
    fn from(variant: HS_USB_PHY_A) -> Self {
        variant as _
    }
}
impl HS_USB_PHY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HS_USB_PHY_A {
        match self.bits {
            0 => HS_USB_PHY_A::NONSECURE_NONPRIV_USER_ALLOWED,
            1 => HS_USB_PHY_A::NONSECURE_PRIV_USER_ALLOWED,
            2 => HS_USB_PHY_A::SECURE_NONPRIV_USER_ALLOWED,
            3 => HS_USB_PHY_A::SECURE_PRIV_USER_ALLOWED,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONSECURE_NONPRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_nonsecure_nonpriv_user_allowed(&self) -> bool {
        *self == HS_USB_PHY_A::NONSECURE_NONPRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `NONSECURE_PRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_nonsecure_priv_user_allowed(&self) -> bool {
        *self == HS_USB_PHY_A::NONSECURE_PRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `SECURE_NONPRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_secure_nonpriv_user_allowed(&self) -> bool {
        *self == HS_USB_PHY_A::SECURE_NONPRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `SECURE_PRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_secure_priv_user_allowed(&self) -> bool {
        *self == HS_USB_PHY_A::SECURE_PRIV_USER_ALLOWED
    }
}
#[doc = "Field `HS_USB_PHY` writer - HS USB PHY"]
pub type HS_USB_PHY_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, AIPS_BRIDGE1_PER_RULE1_SPEC, u8, HS_USB_PHY_A, 2, O>;
impl<'a, const O: u8> HS_USB_PHY_W<'a, O> {
    #[doc = "Non-secure and non-privilege user access allowed"]
    #[inline(always)]
    pub fn nonsecure_nonpriv_user_allowed(self) -> &'a mut W {
        self.variant(HS_USB_PHY_A::NONSECURE_NONPRIV_USER_ALLOWED)
    }
    #[doc = "Non-secure and privilege access allowed"]
    #[inline(always)]
    pub fn nonsecure_priv_user_allowed(self) -> &'a mut W {
        self.variant(HS_USB_PHY_A::NONSECURE_PRIV_USER_ALLOWED)
    }
    #[doc = "Secure and non-privilege user access allowed"]
    #[inline(always)]
    pub fn secure_nonpriv_user_allowed(self) -> &'a mut W {
        self.variant(HS_USB_PHY_A::SECURE_NONPRIV_USER_ALLOWED)
    }
    #[doc = "Secure and privilege user access allowed"]
    #[inline(always)]
    pub fn secure_priv_user_allowed(self) -> &'a mut W {
        self.variant(HS_USB_PHY_A::SECURE_PRIV_USER_ALLOWED)
    }
}
#[doc = "Field `FLEXSPI1_REGISTERS` reader - FLEXSPI1 Registers"]
pub type FLEXSPI1_REGISTERS_R = crate::FieldReader<u8, FLEXSPI1_REGISTERS_A>;
#[doc = "FLEXSPI1 Registers\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FLEXSPI1_REGISTERS_A {
    #[doc = "0: Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0,
    #[doc = "1: Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 1,
    #[doc = "2: Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 2,
    #[doc = "3: Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 3,
}
impl From<FLEXSPI1_REGISTERS_A> for u8 {
    #[inline(always)]
    fn from(variant: FLEXSPI1_REGISTERS_A) -> Self {
        variant as _
    }
}
impl FLEXSPI1_REGISTERS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLEXSPI1_REGISTERS_A {
        match self.bits {
            0 => FLEXSPI1_REGISTERS_A::NONSECURE_NONPRIV_USER_ALLOWED,
            1 => FLEXSPI1_REGISTERS_A::NONSECURE_PRIV_USER_ALLOWED,
            2 => FLEXSPI1_REGISTERS_A::SECURE_NONPRIV_USER_ALLOWED,
            3 => FLEXSPI1_REGISTERS_A::SECURE_PRIV_USER_ALLOWED,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONSECURE_NONPRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_nonsecure_nonpriv_user_allowed(&self) -> bool {
        *self == FLEXSPI1_REGISTERS_A::NONSECURE_NONPRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `NONSECURE_PRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_nonsecure_priv_user_allowed(&self) -> bool {
        *self == FLEXSPI1_REGISTERS_A::NONSECURE_PRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `SECURE_NONPRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_secure_nonpriv_user_allowed(&self) -> bool {
        *self == FLEXSPI1_REGISTERS_A::SECURE_NONPRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `SECURE_PRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_secure_priv_user_allowed(&self) -> bool {
        *self == FLEXSPI1_REGISTERS_A::SECURE_PRIV_USER_ALLOWED
    }
}
#[doc = "Field `FLEXSPI1_REGISTERS` writer - FLEXSPI1 Registers"]
pub type FLEXSPI1_REGISTERS_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, AIPS_BRIDGE1_PER_RULE1_SPEC, u8, FLEXSPI1_REGISTERS_A, 2, O>;
impl<'a, const O: u8> FLEXSPI1_REGISTERS_W<'a, O> {
    #[doc = "Non-secure and non-privilege user access allowed"]
    #[inline(always)]
    pub fn nonsecure_nonpriv_user_allowed(self) -> &'a mut W {
        self.variant(FLEXSPI1_REGISTERS_A::NONSECURE_NONPRIV_USER_ALLOWED)
    }
    #[doc = "Non-secure and privilege access allowed"]
    #[inline(always)]
    pub fn nonsecure_priv_user_allowed(self) -> &'a mut W {
        self.variant(FLEXSPI1_REGISTERS_A::NONSECURE_PRIV_USER_ALLOWED)
    }
    #[doc = "Secure and non-privilege user access allowed"]
    #[inline(always)]
    pub fn secure_nonpriv_user_allowed(self) -> &'a mut W {
        self.variant(FLEXSPI1_REGISTERS_A::SECURE_NONPRIV_USER_ALLOWED)
    }
    #[doc = "Secure and privilege user access allowed"]
    #[inline(always)]
    pub fn secure_priv_user_allowed(self) -> &'a mut W {
        self.variant(FLEXSPI1_REGISTERS_A::SECURE_PRIV_USER_ALLOWED)
    }
}
impl R {
    #[doc = "Bits 0:1 - RNG (Random Number Generator)"]
    #[inline(always)]
    pub fn rng(&self) -> RNG_R {
        RNG_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:5 - ACMP 0"]
    #[inline(always)]
    pub fn acmp0(&self) -> ACMP0_R {
        ACMP0_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:9 - ADC 0"]
    #[inline(always)]
    pub fn adc0(&self) -> ADC0_R {
        ADC0_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 12:13 - HS USB PHY"]
    #[inline(always)]
    pub fn hs_usb_phy(&self) -> HS_USB_PHY_R {
        HS_USB_PHY_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 16:17 - FLEXSPI1 Registers"]
    #[inline(always)]
    pub fn flexspi1_registers(&self) -> FLEXSPI1_REGISTERS_R {
        FLEXSPI1_REGISTERS_R::new(((self.bits >> 16) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - RNG (Random Number Generator)"]
    #[inline(always)]
    #[must_use]
    pub fn rng(&mut self) -> RNG_W<0> {
        RNG_W::new(self)
    }
    #[doc = "Bits 4:5 - ACMP 0"]
    #[inline(always)]
    #[must_use]
    pub fn acmp0(&mut self) -> ACMP0_W<4> {
        ACMP0_W::new(self)
    }
    #[doc = "Bits 8:9 - ADC 0"]
    #[inline(always)]
    #[must_use]
    pub fn adc0(&mut self) -> ADC0_W<8> {
        ADC0_W::new(self)
    }
    #[doc = "Bits 12:13 - HS USB PHY"]
    #[inline(always)]
    #[must_use]
    pub fn hs_usb_phy(&mut self) -> HS_USB_PHY_W<12> {
        HS_USB_PHY_W::new(self)
    }
    #[doc = "Bits 16:17 - FLEXSPI1 Registers"]
    #[inline(always)]
    #[must_use]
    pub fn flexspi1_registers(&mut self) -> FLEXSPI1_REGISTERS_W<16> {
        FLEXSPI1_REGISTERS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AIPS Bridge Peripheral 1 Rule 1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aips_bridge1_per_rule1](index.html) module"]
pub struct AIPS_BRIDGE1_PER_RULE1_SPEC;
impl crate::RegisterSpec for AIPS_BRIDGE1_PER_RULE1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [aips_bridge1_per_rule1::R](R) reader structure"]
impl crate::Readable for AIPS_BRIDGE1_PER_RULE1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [aips_bridge1_per_rule1::W](W) writer structure"]
impl crate::Writable for AIPS_BRIDGE1_PER_RULE1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AIPS_BRIDGE1_PER_RULE1 to value 0"]
impl crate::Resettable for AIPS_BRIDGE1_PER_RULE1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
