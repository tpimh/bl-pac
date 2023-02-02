#[doc = "Register `DMA_RawIntErrorStatus` reader"]
pub struct R(crate::R<DMA_RAW_INT_ERROR_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_RAW_INT_ERROR_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_RAW_INT_ERROR_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_RAW_INT_ERROR_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMA_RawIntErrorStatus` writer"]
pub struct W(crate::W<DMA_RAW_INT_ERROR_STATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMA_RAW_INT_ERROR_STATUS_SPEC>;
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
impl From<crate::W<DMA_RAW_INT_ERROR_STATUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMA_RAW_INT_ERROR_STATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RawIntErrorStatus` reader - "]
pub type RAW_INT_ERROR_STATUS_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn raw_int_error_status(&self) -> RAW_INT_ERROR_STATUS_R {
        RAW_INT_ERROR_STATUS_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA_RawIntErrorStatus.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_raw_int_error_status](index.html) module"]
pub struct DMA_RAW_INT_ERROR_STATUS_SPEC;
impl crate::RegisterSpec for DMA_RAW_INT_ERROR_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma_raw_int_error_status::R](R) reader structure"]
impl crate::Readable for DMA_RAW_INT_ERROR_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dma_raw_int_error_status::W](W) writer structure"]
impl crate::Writable for DMA_RAW_INT_ERROR_STATUS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMA_RawIntErrorStatus to value 0"]
impl crate::Resettable for DMA_RAW_INT_ERROR_STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}