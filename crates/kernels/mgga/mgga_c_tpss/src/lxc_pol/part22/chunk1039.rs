//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1039/1266 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1039<F: Float>(t1561: F, t3110: F, t1133: F, t4245: F, t12352: F, t466: F, t2785: F, t3073: F, t450: F, t3053: F, t9080: F, t1578: F, t1113: F, t1141: F, t1143: F, t12554: F, t12580: F, t12590: F, t1581: F, t220: F, t3124: F, t3126: F, t3138: F, t3139: F, t4293: F, t4303: F, t4307: F, t4310: F, t4314: F, t468: F, t9749: F, t9759: F, t9764: F, t9787: F) -> (F,) {
    let t12597 = t3110 * t1561;
    let t12600 = t1133 * t4245;
    let t12607 = t466 * t12352;
    let t12614 = t2785 * t3073 * t450;
    let t12618 = t9080 * t3053 * t450;
    let t12621 = t1578 * t3053;
    let t12629 = t1578 * t3073;
    let t12636 = 2.0 * t1113 * t1141 * t1143 * t4293 + t1141 * t1143 * t12597 + 2.0 * t1141 * t1143 * t12600 + t1141 * t1143 * t12607 + t1141 * t1143 * t12629 + t12554 * t220 * t468 + 6.0 * t12580 * t1581 * t9749 - 6.0 * t12590 * t1581 * t9764 - t12614 * t1581 * t3138 + t12618 * t1581 * t9787 + 2.0 * t12621 * t3124 * t3126 - t12621 * t3138 * t3139 + 2.0 * t1581 * t3124 * t9759 + 4.0 * t3124 * t4303 * t4307 + 4.0 * t3124 * t4303 * t4310 - 2.0 * t3138 * t4307 * t4314 - 2.0 * t3138 * t4310 * t4314;
    (t12636,)
}
