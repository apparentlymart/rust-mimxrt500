#[doc = "Register `FrameBufferStride0` reader"]
pub struct R(crate::R<FRAME_BUFFER_STRIDE0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FRAME_BUFFER_STRIDE0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FRAME_BUFFER_STRIDE0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FRAME_BUFFER_STRIDE0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FrameBufferStride0` writer"]
pub struct W(crate::W<FRAME_BUFFER_STRIDE0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FRAME_BUFFER_STRIDE0_SPEC>;
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
impl From<crate::W<FRAME_BUFFER_STRIDE0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FRAME_BUFFER_STRIDE0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `STRIDE` reader - Number of bytes from start of one line to next line. This value needs to be 128 byte aligned."]
pub type STRIDE_R = crate::FieldReader<u32, u32>;
#[doc = "Field `STRIDE` writer - Number of bytes from start of one line to next line. This value needs to be 128 byte aligned."]
pub type STRIDE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FRAME_BUFFER_STRIDE0_SPEC, u32, u32, 17, O>;
impl R {
    #[doc = "Bits 0:16 - Number of bytes from start of one line to next line. This value needs to be 128 byte aligned."]
    #[inline(always)]
    pub fn stride(&self) -> STRIDE_R {
        STRIDE_R::new(self.bits & 0x0001_ffff)
    }
}
impl W {
    #[doc = "Bits 0:16 - Number of bytes from start of one line to next line. This value needs to be 128 byte aligned."]
    #[inline(always)]
    #[must_use]
    pub fn stride(&mut self) -> STRIDE_W<0> {
        STRIDE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Stride of the Frame Buffer in Bytes\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [frame_buffer_stride0](index.html) module"]
pub struct FRAME_BUFFER_STRIDE0_SPEC;
impl crate::RegisterSpec for FRAME_BUFFER_STRIDE0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [frame_buffer_stride0::R](R) reader structure"]
impl crate::Readable for FRAME_BUFFER_STRIDE0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [frame_buffer_stride0::W](W) writer structure"]
impl crate::Writable for FRAME_BUFFER_STRIDE0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FrameBufferStride0 to value 0"]
impl crate::Resettable for FRAME_BUFFER_STRIDE0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
