#[doc = r"Register block"]
#[repr(C)]
pub struct CTX {
    #[doc = "0x00..0x10 - AES Key Word"]
    pub ctx_key: [CTX_KEY; 4],
    #[doc = "0x10..0x18 - AES Counter Word"]
    pub ctx_ctr: [CTX_CTR; 2],
    #[doc = "0x18 - AES Region Descriptor Word0"]
    pub ctx_rgd_w0: CTX_RGD_W0,
    #[doc = "0x1c - AES Region Descriptor Word1"]
    pub ctx_rgd_w1: CTX_RGD_W1,
}
#[doc = "CTX_KEY (rw) register accessor: an alias for `Reg<CTX_KEY_SPEC>`"]
pub type CTX_KEY = crate::Reg<ctx_key::CTX_KEY_SPEC>;
#[doc = "AES Key Word"]
pub mod ctx_key;
#[doc = "CTX_CTR (rw) register accessor: an alias for `Reg<CTX_CTR_SPEC>`"]
pub type CTX_CTR = crate::Reg<ctx_ctr::CTX_CTR_SPEC>;
#[doc = "AES Counter Word"]
pub mod ctx_ctr;
#[doc = "CTX_RGD_W0 (rw) register accessor: an alias for `Reg<CTX_RGD_W0_SPEC>`"]
pub type CTX_RGD_W0 = crate::Reg<ctx_rgd_w0::CTX_RGD_W0_SPEC>;
#[doc = "AES Region Descriptor Word0"]
pub mod ctx_rgd_w0;
#[doc = "CTX_RGD_W1 (rw) register accessor: an alias for `Reg<CTX_RGD_W1_SPEC>`"]
pub type CTX_RGD_W1 = crate::Reg<ctx_rgd_w1::CTX_RGD_W1_SPEC>;
#[doc = "AES Region Descriptor Word1"]
pub mod ctx_rgd_w1;
