//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1069/1304 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1069<F: Float>(t6880: F, t8690: F, t2165: F, t6534: F, t652: F, t1873: F, t24969: F, t24972: F, t7015: F, t7423: F, t1190: F, t8882: F) -> (F, F, F, F, F, F, F) {
    let t31916 = t8690 * t6880;
    let t31918 = t2165 * t6534;
    let t31919 = t652 * t31918;
    let t31940 = t24969 * t1873;
    let t31942 = t24972 * t7015;
    let t31944 = t7423 * t6534;
    let t32422 = t1190 * t8882;
    (t31916, t31918, t31919, t31940, t31942, t31944, t32422)
}
