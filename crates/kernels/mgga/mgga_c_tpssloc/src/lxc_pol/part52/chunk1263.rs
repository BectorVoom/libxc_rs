//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1263/1400 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1263<F: Float>(t1862: F, t8308: F, t113875: F, t31867: F, t9239: F, t31863: F, t9231: F, t131: F, t8662: F, t2240: F, t24525: F, t39054: F) -> (F, F, F, F, F, F, F, F, F) {
    let t115833 = t8308 * t1862;
    let t115903 = t113875 * t1862;
    let t116082 = t9239 * t31867;
    let t116106 = t9239 * t31863;
    let t116111 = t9231 * t31863;
    let t116114 = t8662 * t131;
    let t116115 = t9239 * t116114;
    let t116119 = t2240 * t24525 * t131;
    let t116124 = t39054 * t8662;
    (t115833, t115903, t116082, t116106, t116111, t116114, t116115, t116119, t116124)
}
