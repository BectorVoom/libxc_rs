//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1135/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1135<F: Float>(t116114: F, t9239: F, t131: F, t2240: F, t24525: F, t39054: F, t8662: F, t22573: F, t8689: F, t111: F, t31876: F, t2174: F, t7002: F, t2169: F, t7020: F, t1404: F, t8692: F) -> (F, F, F, F, F, F, F, F) {
    let t116115 = t9239 * t116114;
    let t116119 = t2240 * t24525 * t131;
    let t116124 = t39054 * t8662;
    let t116135 = t8689 * t22573;
    let t116152 = t31876 * t111;
    let t116324 = t7002 * t2174;
    let t116326 = t2169 * t7020;
    let t116328 = t8692 * t1404;
    (t116115, t116119, t116124, t116135, t116152, t116324, t116326, t116328)
}
