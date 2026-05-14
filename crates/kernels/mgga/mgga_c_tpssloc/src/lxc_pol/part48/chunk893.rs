//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 893/910 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk893<F: Float>(t31863: F, t9239: F, t9231: F, t131: F, t8662: F, t2240: F, t24525: F, t39054: F, t22573: F, t8689: F, t191: F, t192: F, t24939: F, t2098: F, t7426: F, t32392: F, t580: F) -> (F, F, F, F, F, F, F, F, F) {
    let t116106 = t9239 * t31863;
    let t116111 = t9231 * t31863;
    let t116114 = t8662 * t131;
    let t116115 = t9239 * t116114;
    let t116119 = t2240 * t24525 * t131;
    let t116124 = t39054 * t8662;
    let t116135 = t8689 * t22573;
    let t116304 = t24939 * t191 * t192;
    let t117407 = t2098 * t7426;
    let t117410 = t32392 * t580;
    (t116106, t116111, t116115, t116119, t116124, t116135, t116304, t117407, t117410)
}
