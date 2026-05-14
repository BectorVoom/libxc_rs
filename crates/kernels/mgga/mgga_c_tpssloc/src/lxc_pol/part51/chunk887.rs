//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 887/1308 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk887<F: Float>(t3113: F, t6754: F, t3107: F, t6753: F, t1012: F, t1933: F, t607: F, t1937: F, t1926: F, t3158: F, t40: F, t6722: F, t6712: F, t995: F, t1942: F, t3082: F) -> (F, F, F, F, F, F, F) {
    let t23433 = t3113 * t6754;
    let t23436 = t6753 * t3107;
    let t23437 = t1012 * t23436;
    let t23442 = t1933 * t607;
    let t23443 = t23442 * t1937;
    let t23447 = t1926 * t3158 / 432.0;
    let t23448 = t6722 * t40;
    let t23449 = t23448 * t1937;
    let t23463 = t6712 * t995;
    let t23469 = t1942 * t3082 / 6912.0;
    (t23433, t23437, t23443, t23447, t23449, t23463, t23469)
}
