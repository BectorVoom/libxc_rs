//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 870/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk870<F: Float>(t460: F, t9699: F, t3097: F, t774: F, t1137: F, t73: F, t8549: F, t9615: F, t8548: F, t9080: F, t9619: F, t3048: F, t3054: F, t1107: F, t3308: F, t8229: F) -> (F, F, F, F, F, F, F, F, F) {
    let t9701 = t460 * t9699 / 10368.0;
    let t9702 = t774 * t3097;
    let t9737 = t1137 * t1137;
    let t9738 = 1.0 / t9737;
    let t9739 = t73 * t9738;
    let t9748 = t8549 * t9615;
    let t9749 = t8548 * t9748;
    let t9751 = t9080 * t9619;
    let t9763 = t8549 * t3048;
    let t9764 = t8548 * t9763;
    let t9765 = t9080 * t3054;
    let t9786 = t8549 * t1107;
    let t9787 = t8548 * t9786;
    let t9839 = 0.21687162600603479684e-1 * t3308 * t8229;
    (t9701, t9702, t9739, t9749, t9751, t9764, t9765, t9787, t9839)
}
