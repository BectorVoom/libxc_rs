//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1241/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1241<F: Float>(t66390: F, t66393: F, t66394: F, t66399: F, t69926: F, t69928: F, t69930: F, t69932: F, t69934: F, t69936: F, t69938: F, t69941: F, t62690: F, t69945: F, t69948: F, t69950: F, t69952: F, t69954: F, t69956: F, t69958: F, t69960: F, t69962: F, t69964: F, t69966: F, t69968: F) -> (F, F) {
    let t72044 = -5.0 / 96.0 * t69926 + t69928 / 96.0 - t69930 / 48.0 - t66390 + t69932 / 192.0 + t69934 / 192.0 - 7.0 / 144.0 * t69936 + t69938 / 384.0 - t66393 - t66394 + t66399 + t69941 / 8.0;
    let t72057 = -t69945 / 2.0 + t69948 / 4.0 - t62690 - t69950 / 192.0 + 7.0 / 288.0 * t69952 - 35.0 / 288.0 * t69954 - t69956 / 384.0 - t69958 / 768.0 - 5.0 / 192.0 * t69960 - t69962 / 128.0 + t69964 / 128.0 + t69966 / 192.0 - t69968 / 768.0;
    (t72044, t72057)
}
