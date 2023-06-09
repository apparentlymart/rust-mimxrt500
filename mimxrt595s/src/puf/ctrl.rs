#[doc = "Register `CTRL` reader"]
pub struct R(crate::R<CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL` writer"]
pub struct W(crate::W<CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL_SPEC>;
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
impl From<crate::W<CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ZEROIZE` reader - Zeroize"]
pub type ZEROIZE_R = crate::BitReader<bool>;
#[doc = "Field `ZEROIZE` writer - Zeroize"]
pub type ZEROIZE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `ENROLL` reader - Enroll"]
pub type ENROLL_R = crate::BitReader<bool>;
#[doc = "Field `ENROLL` writer - Enroll"]
pub type ENROLL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `START` reader - Start"]
pub type START_R = crate::BitReader<bool>;
#[doc = "Field `START` writer - Start"]
pub type START_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `GENERATEKEY` reader - Set Intrinsic Key"]
pub type GENERATEKEY_R = crate::BitReader<bool>;
#[doc = "Field `GENERATEKEY` writer - Set Intrinsic Key"]
pub type GENERATEKEY_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `SETKEY` reader - Set Key"]
pub type SETKEY_R = crate::BitReader<bool>;
#[doc = "Field `SETKEY` writer - Set Key"]
pub type SETKEY_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `GETKEY` reader - Get Key"]
pub type GETKEY_R = crate::BitReader<bool>;
#[doc = "Field `GETKEY` writer - Get Key"]
pub type GETKEY_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Zeroize"]
    #[inline(always)]
    pub fn zeroize(&self) -> ZEROIZE_R {
        ZEROIZE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enroll"]
    #[inline(always)]
    pub fn enroll(&self) -> ENROLL_R {
        ENROLL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Start"]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Set Intrinsic Key"]
    #[inline(always)]
    pub fn generatekey(&self) -> GENERATEKEY_R {
        GENERATEKEY_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Set Key"]
    #[inline(always)]
    pub fn setkey(&self) -> SETKEY_R {
        SETKEY_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Get Key"]
    #[inline(always)]
    pub fn getkey(&self) -> GETKEY_R {
        GETKEY_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Zeroize"]
    #[inline(always)]
    #[must_use]
    pub fn zeroize(&mut self) -> ZEROIZE_W<0> {
        ZEROIZE_W::new(self)
    }
    #[doc = "Bit 1 - Enroll"]
    #[inline(always)]
    #[must_use]
    pub fn enroll(&mut self) -> ENROLL_W<1> {
        ENROLL_W::new(self)
    }
    #[doc = "Bit 2 - Start"]
    #[inline(always)]
    #[must_use]
    pub fn start(&mut self) -> START_W<2> {
        START_W::new(self)
    }
    #[doc = "Bit 3 - Set Intrinsic Key"]
    #[inline(always)]
    #[must_use]
    pub fn generatekey(&mut self) -> GENERATEKEY_W<3> {
        GENERATEKEY_W::new(self)
    }
    #[doc = "Bit 4 - Set Key"]
    #[inline(always)]
    #[must_use]
    pub fn setkey(&mut self) -> SETKEY_W<4> {
        SETKEY_W::new(self)
    }
    #[doc = "Bit 6 - Get Key"]
    #[inline(always)]
    #[must_use]
    pub fn getkey(&mut self) -> GETKEY_W<6> {
        GETKEY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PUF Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](index.html) module"]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl::R](R) reader structure"]
impl crate::Readable for CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl::W](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
