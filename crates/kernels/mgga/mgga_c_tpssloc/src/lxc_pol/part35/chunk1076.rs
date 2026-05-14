//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1076/1310 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1076<F: Float>(t2136: F, t27650: F, t210: F, t7998: F, t1193: F, t8020: F, t52: F, t8027: F, t461: F, t7573: F, t7324: F, t1210: F, t8039: F, t24721: F, t1714: F, t2133: F) -> (F, F, F, F, F, F, F, F, F) {
    let t27651 = t27650 * t2136;
    let t27674 = t7998 * t210;
    let t27677 = t8020 * t1193;
    let t27680 = t8027 * t52;
    let t27681 = t27680 * t2136;
    let t27683 = t7573 * t461;
    let t27684 = t7324 * t27683;
    let t27700 = t1210 * t8039;
    let t27701 = t24721 * t27700;
    let t27703 = t2133 * t1714;
    (t27651, t27674, t27677, t27681, t27683, t27684, t27700, t27701, t27703)
}
