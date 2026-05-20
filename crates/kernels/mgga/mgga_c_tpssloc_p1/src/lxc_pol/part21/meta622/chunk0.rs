//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2400/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2400<F: Float>(t212: F, t2586: F, t3734: F, t40353: F, t12225: F, t3719: F, t116: F, t1314: F, t9534: F, t1307: F, t133: F, t6600: F) -> (F, F, F, F) {
    let t40356 = t2586 * t40353 * t212 * t3734;
    let t40360 = t2586 * t12225 * t212 * t3719;
    let t40369 = t9534 * t1314 * t116;
    let t40372 = t40369 * t133 * t6600 * t1307;
    (t40356, t40360, t40369, t40372)
}
