//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1218/1266 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1218<F: Float>(t63907: F, t63913: F, t63917: F, t63899: F, t63901: F, t63903: F, t63905: F, t63909: F, t63911: F, t63921: F, t63923: F, t63925: F, t63928: F, t63945: F, t61034: F, t61051: F, t61054: F, t61058: F, t61060: F, t62690: F, t63930: F, t63932: F, t63939: F, t63941: F, t63943: F) -> (F, F) {
    let t66390 = 7.0 / 144.0 * t63907;
    let t66393 = 7.0 / 144.0 * t63913;
    let t66394 = 7.0 / 288.0 * t63917;
    let t66398 = -t63899 / 384.0 - t63901 / 768.0 + t63903 / 96.0 + t63905 / 192.0 - t66390 + t63909 / 192.0 + t63911 / 96.0 - t66393 - t66394 - t63921 / 128.0 + t63923 / 128.0 - t63925 / 768.0;
    let t66399 = 7.0 / 576.0 * t63928;
    let t66410 = 119.0 / 3456.0 * t63945;
    let t66411 = t66399 + t63930 / 96.0 - t62690 - 5.0 / 96.0 * t63932 - 7.0 / 144.0 * t61034 - 119.0 / 1728.0 * t61051 + 7.0 / 1152.0 * t61054 - 7.0 / 576.0 * t61058 + 7.0 / 1152.0 * t61060 + t63939 / 192.0 - 5.0 / 192.0 * t63941 - t63943 / 96.0 - t66410;
    (t66398, t66411)
}
