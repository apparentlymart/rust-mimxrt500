#[doc = "Register `AHB_PERIPH2_SLAVE_RULE0` reader"]
pub struct R(crate::R<AHB_PERIPH2_SLAVE_RULE0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AHB_PERIPH2_SLAVE_RULE0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AHB_PERIPH2_SLAVE_RULE0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AHB_PERIPH2_SLAVE_RULE0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AHB_PERIPH2_SLAVE_RULE0` writer"]
pub struct W(crate::W<AHB_PERIPH2_SLAVE_RULE0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AHB_PERIPH2_SLAVE_RULE0_SPEC>;
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
impl From<crate::W<AHB_PERIPH2_SLAVE_RULE0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AHB_PERIPH2_SLAVE_RULE0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USB_HS_RAM` reader - USB HS RAM"]
pub type USB_HS_RAM_R = crate::FieldReader<u8, USB_HS_RAM_A>;
#[doc = "USB HS RAM\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum USB_HS_RAM_A {
    #[doc = "0: Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0,
    #[doc = "1: Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 1,
    #[doc = "2: Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 2,
    #[doc = "3: Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 3,
}
impl From<USB_HS_RAM_A> for u8 {
    #[inline(always)]
    fn from(variant: USB_HS_RAM_A) -> Self {
        variant as _
    }
}
impl USB_HS_RAM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USB_HS_RAM_A {
        match self.bits {
            0 => USB_HS_RAM_A::NONSECURE_NONPRIV_USER_ALLOWED,
            1 => USB_HS_RAM_A::NONSECURE_PRIV_USER_ALLOWED,
            2 => USB_HS_RAM_A::SECURE_NONPRIV_USER_ALLOWED,
            3 => USB_HS_RAM_A::SECURE_PRIV_USER_ALLOWED,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONSECURE_NONPRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_nonsecure_nonpriv_user_allowed(&self) -> bool {
        *self == USB_HS_RAM_A::NONSECURE_NONPRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `NONSECURE_PRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_nonsecure_priv_user_allowed(&self) -> bool {
        *self == USB_HS_RAM_A::NONSECURE_PRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `SECURE_NONPRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_secure_nonpriv_user_allowed(&self) -> bool {
        *self == USB_HS_RAM_A::SECURE_NONPRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `SECURE_PRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_secure_priv_user_allowed(&self) -> bool {
        *self == USB_HS_RAM_A::SECURE_PRIV_USER_ALLOWED
    }
}
#[doc = "Field `USB_HS_RAM` writer - USB HS RAM"]
pub type USB_HS_RAM_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, AHB_PERIPH2_SLAVE_RULE0_SPEC, u8, USB_HS_RAM_A, 2, O>;
impl<'a, const O: u8> USB_HS_RAM_W<'a, O> {
    #[doc = "Non-secure and non-privilege user access allowed"]
    #[inline(always)]
    pub fn nonsecure_nonpriv_user_allowed(self) -> &'a mut W {
        self.variant(USB_HS_RAM_A::NONSECURE_NONPRIV_USER_ALLOWED)
    }
    #[doc = "Non-secure and privilege access allowed"]
    #[inline(always)]
    pub fn nonsecure_priv_user_allowed(self) -> &'a mut W {
        self.variant(USB_HS_RAM_A::NONSECURE_PRIV_USER_ALLOWED)
    }
    #[doc = "Secure and non-privilege user access allowed"]
    #[inline(always)]
    pub fn secure_nonpriv_user_allowed(self) -> &'a mut W {
        self.variant(USB_HS_RAM_A::SECURE_NONPRIV_USER_ALLOWED)
    }
    #[doc = "Secure and privilege user access allowed"]
    #[inline(always)]
    pub fn secure_priv_user_allowed(self) -> &'a mut W {
        self.variant(USB_HS_RAM_A::SECURE_PRIV_USER_ALLOWED)
    }
}
#[doc = "Field `USB_HS_DEV` reader - USB HS DEV"]
pub type USB_HS_DEV_R = crate::FieldReader<u8, USB_HS_DEV_A>;
#[doc = "USB HS DEV\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum USB_HS_DEV_A {
    #[doc = "0: Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0,
    #[doc = "1: Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 1,
    #[doc = "2: Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 2,
    #[doc = "3: Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 3,
}
impl From<USB_HS_DEV_A> for u8 {
    #[inline(always)]
    fn from(variant: USB_HS_DEV_A) -> Self {
        variant as _
    }
}
impl USB_HS_DEV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USB_HS_DEV_A {
        match self.bits {
            0 => USB_HS_DEV_A::NONSECURE_NONPRIV_USER_ALLOWED,
            1 => USB_HS_DEV_A::NONSECURE_PRIV_USER_ALLOWED,
            2 => USB_HS_DEV_A::SECURE_NONPRIV_USER_ALLOWED,
            3 => USB_HS_DEV_A::SECURE_PRIV_USER_ALLOWED,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONSECURE_NONPRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_nonsecure_nonpriv_user_allowed(&self) -> bool {
        *self == USB_HS_DEV_A::NONSECURE_NONPRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `NONSECURE_PRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_nonsecure_priv_user_allowed(&self) -> bool {
        *self == USB_HS_DEV_A::NONSECURE_PRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `SECURE_NONPRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_secure_nonpriv_user_allowed(&self) -> bool {
        *self == USB_HS_DEV_A::SECURE_NONPRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `SECURE_PRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_secure_priv_user_allowed(&self) -> bool {
        *self == USB_HS_DEV_A::SECURE_PRIV_USER_ALLOWED
    }
}
#[doc = "Field `USB_HS_DEV` writer - USB HS DEV"]
pub type USB_HS_DEV_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, AHB_PERIPH2_SLAVE_RULE0_SPEC, u8, USB_HS_DEV_A, 2, O>;
impl<'a, const O: u8> USB_HS_DEV_W<'a, O> {
    #[doc = "Non-secure and non-privilege user access allowed"]
    #[inline(always)]
    pub fn nonsecure_nonpriv_user_allowed(self) -> &'a mut W {
        self.variant(USB_HS_DEV_A::NONSECURE_NONPRIV_USER_ALLOWED)
    }
    #[doc = "Non-secure and privilege access allowed"]
    #[inline(always)]
    pub fn nonsecure_priv_user_allowed(self) -> &'a mut W {
        self.variant(USB_HS_DEV_A::NONSECURE_PRIV_USER_ALLOWED)
    }
    #[doc = "Secure and non-privilege user access allowed"]
    #[inline(always)]
    pub fn secure_nonpriv_user_allowed(self) -> &'a mut W {
        self.variant(USB_HS_DEV_A::SECURE_NONPRIV_USER_ALLOWED)
    }
    #[doc = "Secure and privilege user access allowed"]
    #[inline(always)]
    pub fn secure_priv_user_allowed(self) -> &'a mut W {
        self.variant(USB_HS_DEV_A::SECURE_PRIV_USER_ALLOWED)
    }
}
#[doc = "Field `USB_HS_HOST` reader - USB HS HOST"]
pub type USB_HS_HOST_R = crate::FieldReader<u8, USB_HS_HOST_A>;
#[doc = "USB HS HOST\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum USB_HS_HOST_A {
    #[doc = "0: Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0,
    #[doc = "1: Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 1,
    #[doc = "2: Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 2,
    #[doc = "3: Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 3,
}
impl From<USB_HS_HOST_A> for u8 {
    #[inline(always)]
    fn from(variant: USB_HS_HOST_A) -> Self {
        variant as _
    }
}
impl USB_HS_HOST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USB_HS_HOST_A {
        match self.bits {
            0 => USB_HS_HOST_A::NONSECURE_NONPRIV_USER_ALLOWED,
            1 => USB_HS_HOST_A::NONSECURE_PRIV_USER_ALLOWED,
            2 => USB_HS_HOST_A::SECURE_NONPRIV_USER_ALLOWED,
            3 => USB_HS_HOST_A::SECURE_PRIV_USER_ALLOWED,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONSECURE_NONPRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_nonsecure_nonpriv_user_allowed(&self) -> bool {
        *self == USB_HS_HOST_A::NONSECURE_NONPRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `NONSECURE_PRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_nonsecure_priv_user_allowed(&self) -> bool {
        *self == USB_HS_HOST_A::NONSECURE_PRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `SECURE_NONPRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_secure_nonpriv_user_allowed(&self) -> bool {
        *self == USB_HS_HOST_A::SECURE_NONPRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `SECURE_PRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_secure_priv_user_allowed(&self) -> bool {
        *self == USB_HS_HOST_A::SECURE_PRIV_USER_ALLOWED
    }
}
#[doc = "Field `USB_HS_HOST` writer - USB HS HOST"]
pub type USB_HS_HOST_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, AHB_PERIPH2_SLAVE_RULE0_SPEC, u8, USB_HS_HOST_A, 2, O>;
impl<'a, const O: u8> USB_HS_HOST_W<'a, O> {
    #[doc = "Non-secure and non-privilege user access allowed"]
    #[inline(always)]
    pub fn nonsecure_nonpriv_user_allowed(self) -> &'a mut W {
        self.variant(USB_HS_HOST_A::NONSECURE_NONPRIV_USER_ALLOWED)
    }
    #[doc = "Non-secure and privilege access allowed"]
    #[inline(always)]
    pub fn nonsecure_priv_user_allowed(self) -> &'a mut W {
        self.variant(USB_HS_HOST_A::NONSECURE_PRIV_USER_ALLOWED)
    }
    #[doc = "Secure and non-privilege user access allowed"]
    #[inline(always)]
    pub fn secure_nonpriv_user_allowed(self) -> &'a mut W {
        self.variant(USB_HS_HOST_A::SECURE_NONPRIV_USER_ALLOWED)
    }
    #[doc = "Secure and privilege user access allowed"]
    #[inline(always)]
    pub fn secure_priv_user_allowed(self) -> &'a mut W {
        self.variant(USB_HS_HOST_A::SECURE_PRIV_USER_ALLOWED)
    }
}
#[doc = "Field `SCT` reader - SCT"]
pub type SCT_R = crate::FieldReader<u8, SCT_A>;
#[doc = "SCT\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SCT_A {
    #[doc = "0: Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0,
    #[doc = "1: Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 1,
    #[doc = "2: Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 2,
    #[doc = "3: Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 3,
}
impl From<SCT_A> for u8 {
    #[inline(always)]
    fn from(variant: SCT_A) -> Self {
        variant as _
    }
}
impl SCT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SCT_A {
        match self.bits {
            0 => SCT_A::NONSECURE_NONPRIV_USER_ALLOWED,
            1 => SCT_A::NONSECURE_PRIV_USER_ALLOWED,
            2 => SCT_A::SECURE_NONPRIV_USER_ALLOWED,
            3 => SCT_A::SECURE_PRIV_USER_ALLOWED,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONSECURE_NONPRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_nonsecure_nonpriv_user_allowed(&self) -> bool {
        *self == SCT_A::NONSECURE_NONPRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `NONSECURE_PRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_nonsecure_priv_user_allowed(&self) -> bool {
        *self == SCT_A::NONSECURE_PRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `SECURE_NONPRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_secure_nonpriv_user_allowed(&self) -> bool {
        *self == SCT_A::SECURE_NONPRIV_USER_ALLOWED
    }
    #[doc = "Checks if the value of the field is `SECURE_PRIV_USER_ALLOWED`"]
    #[inline(always)]
    pub fn is_secure_priv_user_allowed(&self) -> bool {
        *self == SCT_A::SECURE_PRIV_USER_ALLOWED
    }
}
#[doc = "Field `SCT` writer - SCT"]
pub type SCT_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, AHB_PERIPH2_SLAVE_RULE0_SPEC, u8, SCT_A, 2, O>;
impl<'a, const O: u8> SCT_W<'a, O> {
    #[doc = "Non-secure and non-privilege user access allowed"]
    #[inline(always)]
    pub fn nonsecure_nonpriv_user_allowed(self) -> &'a mut W {
        self.variant(SCT_A::NONSECURE_NONPRIV_USER_ALLOWED)
    }
    #[doc = "Non-secure and privilege access allowed"]
    #[inline(always)]
    pub fn nonsecure_priv_user_allowed(self) -> &'a mut W {
        self.variant(SCT_A::NONSECURE_PRIV_USER_ALLOWED)
    }
    #[doc = "Secure and non-privilege user access allowed"]
    #[inline(always)]
    pub fn secure_nonpriv_user_allowed(self) -> &'a mut W {
        self.variant(SCT_A::SECURE_NONPRIV_USER_ALLOWED)
    }
    #[doc = "Secure and privilege user access allowed"]
    #[inline(always)]
    pub fn secure_priv_user_allowed(self) -> &'a mut W {
        self.variant(SCT_A::SECURE_PRIV_USER_ALLOWED)
    }
}
impl R {
    #[doc = "Bits 0:1 - USB HS RAM"]
    #[inline(always)]
    pub fn usb_hs_ram(&self) -> USB_HS_RAM_R {
        USB_HS_RAM_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:5 - USB HS DEV"]
    #[inline(always)]
    pub fn usb_hs_dev(&self) -> USB_HS_DEV_R {
        USB_HS_DEV_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:9 - USB HS HOST"]
    #[inline(always)]
    pub fn usb_hs_host(&self) -> USB_HS_HOST_R {
        USB_HS_HOST_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 12:13 - SCT"]
    #[inline(always)]
    pub fn sct(&self) -> SCT_R {
        SCT_R::new(((self.bits >> 12) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - USB HS RAM"]
    #[inline(always)]
    #[must_use]
    pub fn usb_hs_ram(&mut self) -> USB_HS_RAM_W<0> {
        USB_HS_RAM_W::new(self)
    }
    #[doc = "Bits 4:5 - USB HS DEV"]
    #[inline(always)]
    #[must_use]
    pub fn usb_hs_dev(&mut self) -> USB_HS_DEV_W<4> {
        USB_HS_DEV_W::new(self)
    }
    #[doc = "Bits 8:9 - USB HS HOST"]
    #[inline(always)]
    #[must_use]
    pub fn usb_hs_host(&mut self) -> USB_HS_HOST_W<8> {
        USB_HS_HOST_W::new(self)
    }
    #[doc = "Bits 12:13 - SCT"]
    #[inline(always)]
    #[must_use]
    pub fn sct(&mut self) -> SCT_W<12> {
        SCT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AHB Peripheral 2 Slave Rule 0 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ahb_periph2_slave_rule0](index.html) module"]
pub struct AHB_PERIPH2_SLAVE_RULE0_SPEC;
impl crate::RegisterSpec for AHB_PERIPH2_SLAVE_RULE0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ahb_periph2_slave_rule0::R](R) reader structure"]
impl crate::Readable for AHB_PERIPH2_SLAVE_RULE0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ahb_periph2_slave_rule0::W](W) writer structure"]
impl crate::Writable for AHB_PERIPH2_SLAVE_RULE0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AHB_PERIPH2_SLAVE_RULE0 to value 0"]
impl crate::Resettable for AHB_PERIPH2_SLAVE_RULE0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
