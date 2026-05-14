//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1122/1310 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1122<F: Float>(t225: F, t497: F, t6238: F, t462: F, t27751: F, t8014: F, t1887: F, t29584: F) -> (F, F, F, F) {
    let t29670 = t6238 * t225 * t497;
    let t29671 = t462 * t29670;
    let t29674 = t27751 * t8014;
    let t29678 = t29584 * t1887;
    (t29670, t29671, t29674, t29678)
}
