#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - pktFIFO Control"]
    pub fifoctrl: FIFOCTRL,
}
#[doc = "pktFIFO Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifoctrl](fifoctrl) module"]
pub type FIFOCTRL = crate::Reg<u32, _FIFOCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FIFOCTRL;
#[doc = "`read()` method returns [fifoctrl::R](fifoctrl::R) reader structure"]
impl crate::Readable for FIFOCTRL {}
#[doc = "`write(|w| ..)` method takes [fifoctrl::W](fifoctrl::W) writer structure"]
impl crate::Writable for FIFOCTRL {}
#[doc = "pktFIFO Control"]
pub mod fifoctrl;
