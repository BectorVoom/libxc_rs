//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1211/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1211<F: Float>(t10224: F, t5828: F, t973: F, t42875: F, t5817: F, t10508: F, t248: F, t3130: F, t5873: F, t3030: F, t5848: F, t3032: F) -> (F, F, F, F, F) {
    let t61597 = t973 * t10224 * t5828;
    let t61600 = t973 * t42875 * t5817;
    let t61663 = t3130 * t248 * t10508 * t5873;
    let t61734 = t5848 * t3030;
    let t61735 = t61734 * t3032;
    (t61597, t61600, t61663, t61734, t61735)
}
