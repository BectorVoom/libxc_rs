//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1172/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1172<F: Float>(t1268: F, t5371: F, t1206: F, t5451: F, t1625: F, t4519: F, t4706: F, t821: F, t16264: F, t782: F, t4701: F, t1364: F, t3724: F, t18495: F, t5736: F, t10179: F, t1771: F, t5570: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t51631 = t5371 * t1268;
    let t51635 = t5451 * t1206;
    let t51642 = t1625 * t4519;
    let t51664 = t5451 * t1268;
    let t51780 = t4706 * t821;
    let t52460 = t16264 * t782;
    let t52613 = t4701 * t821;
    let t52639 = t1364 * t3724;
    let t60649 = t5736 * t18495;
    let t60653 = t1771 * t5570 * t10179;
    (t51631, t51635, t51642, t51664, t51780, t52460, t52613, t52639, t60649, t60653)
}
