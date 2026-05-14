//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1163/1310 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1163<F: Float>(t23047: F, t4166: F, t1512: F, t81807: F, t23040: F, t1496: F, t81942: F, t7497: F, t81933: F, t23132: F, t1516: F, t81763: F, t25064: F, t81788: F, t2693: F, t7503: F) -> (F, F, F, F, F, F, F, F, F) {
    let t87218 = t4166 * t23047;
    let t87243 = t81807 * t1512;
    let t87261 = t4166 * t23040;
    let t87304 = t81942 * t1496;
    let t87306 = t81933 * t7497;
    let t87340 = t4166 * t23132;
    let t87345 = t81763 * t1516;
    let t87387 = t81788 * t25064;
    let t87403 = t7503 * t2693;
    (t87218, t87243, t87261, t87304, t87306, t87340, t87345, t87387, t87403)
}
