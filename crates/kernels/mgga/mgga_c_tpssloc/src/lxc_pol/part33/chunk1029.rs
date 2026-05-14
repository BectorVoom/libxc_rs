//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1029/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1029<F: Float>(t23122: F, t25064: F, t4166: F, t6620: F, t1516: F, t23133: F, t7503: F, t838: F, t23046: F, t242: F, t812: F, t23062: F, t7497: F, t23077: F, t6604: F, t23083: F, t7500: F) -> (F, F, F, F, F, F, F, F) {
    let t25065 = t23122 * t25064;
    let t25068 = t4166 * t6620;
    let t25077 = t23133 * t1516;
    let t25080 = t7503 * t838;
    let t25083 = t23046 * t242;
    let t25084 = t812 * t25083;
    let t25109 = t23062 * t7497;
    let t25119 = t23077 * t6604;
    let t25126 = t23083 * t7500;
    (t25065, t25068, t25077, t25080, t25084, t25109, t25119, t25126)
}
