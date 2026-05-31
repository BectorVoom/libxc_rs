//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2757/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2757<F: Float>(t4119: F, t868: F, t12652: F, t12939: F, t4195: F, t1462: F, t47172: F) -> (F, F, F, F) {
    let t58071 = t4119 * t868;
    let t58080 = F::cast_from(96.0_f64) * t12939 * t4195 * t12652;
    let t58085 = F::cast_from(8.0_f64) * t47172 * t1462;
    let t58090 = t4119 * t4119;
    (t58071, t58080, t58085, t58090)
}
