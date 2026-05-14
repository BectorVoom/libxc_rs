//! MGGA_C_TPSS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1263/1368 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part21_v4rho3sigma_3_chunk1263<F: Float>(t61086: F, t803: F, t17974: F, t2391: F, t17990: F, t5570: F, t17982: F, t219: F, t228: F, t32386: F, t18005: F, t5567: F, t1706: F, t8347: F, t5562: F, t768: F) -> (F, F, F, F, F, F, F, F) {
    let t61087 = t61086 * t803;
    let t61089 = t17974 * t2391;
    let t61183 = t17990 * t5570;
    let t61190 = t17982 * t219;
    let t61195 = t32386 * t228;
    let t61222 = t5567 * t18005;
    let t61226 = t1706 * t5570 * t8347;
    let t61232 = t768 * t5562;
    (t61087, t61089, t61183, t61190, t61195, t61222, t61226, t61232)
}
