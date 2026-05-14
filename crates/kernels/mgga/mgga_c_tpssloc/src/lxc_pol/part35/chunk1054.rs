//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1054/1310 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1054<F: Float>(t1527: F, t2717: F, t6547: F, t7485: F, t1484: F, t22690: F, t841: F, t23122: F, t4166: F, t6620: F, t1516: F, t23133: F, t7503: F, t838: F, t23046: F, t242: F) -> (F, F, F, F, F, F, F, F) {
    let t25044 = t2717 * t1527;
    let t25049 = t6547 * t7485;
    let t25064 = t22690 * t841 * t1484;
    let t25065 = t23122 * t25064;
    let t25068 = t4166 * t6620;
    let t25077 = t23133 * t1516;
    let t25080 = t7503 * t838;
    let t25083 = t23046 * t242;
    (t25044, t25049, t25064, t25065, t25068, t25077, t25080, t25083)
}
