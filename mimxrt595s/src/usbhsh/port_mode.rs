#[doc = "Register `PORT_MODE` reader"]
pub struct R(crate::R<PORT_MODE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PORT_MODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PORT_MODE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PORT_MODE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PORT_MODE` writer"]
pub struct W(crate::W<PORT_MODE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PORT_MODE_SPEC>;
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
impl From<crate::W<PORT_MODE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PORT_MODE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DEV_ENABLE` reader - If this bit is set to one, the port will behave as a USB device. If this bit is set to zero, the port will be controlled by the USB host block."]
pub type DEV_ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `DEV_ENABLE` writer - If this bit is set to one, the port will behave as a USB device. If this bit is set to zero, the port will be controlled by the USB host block."]
pub type DEV_ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, PORT_MODE_SPEC, bool, O>;
impl R {
    #[doc = "Bit 16 - If this bit is set to one, the port will behave as a USB device. If this bit is set to zero, the port will be controlled by the USB host block."]
    #[inline(always)]
    pub fn dev_enable(&self) -> DEV_ENABLE_R {
        DEV_ENABLE_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 16 - If this bit is set to one, the port will behave as a USB device. If this bit is set to zero, the port will be controlled by the USB host block."]
    #[inline(always)]
    #[must_use]
    pub fn dev_enable(&mut self) -> DEV_ENABLE_W<16> {
        DEV_ENABLE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port Mode\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [port_mode](index.html) module"]
pub struct PORT_MODE_SPEC;
impl crate::RegisterSpec for PORT_MODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [port_mode::R](R) reader structure"]
impl crate::Readable for PORT_MODE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [port_mode::W](W) writer structure"]
impl crate::Writable for PORT_MODE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PORT_MODE to value 0x0004_0000"]
impl crate::Resettable for PORT_MODE_SPEC {
    const RESET_VALUE: Self::Ux = 0x0004_0000;
}
