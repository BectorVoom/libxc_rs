//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1511/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1511<F: Float>(t1307: F, t210: F, t6370: F, t1810: F, t5187: F, t6374: F, t1358: F, t6379: F, t19805: F, t554: F, t12211: F, t6371: F) -> (F, F, F, F, F, F) {
    let t19823 = t210 * t6370 * t1307;
    let t19827 = t210 * t1810 * t5187;
    let t19831 = t210 * t6374 * t1307;
    let t19834 = t6379 * t1358;
    let t19836 = t19805 * t554;
    let t19839 = t12211 * t6371;
    (t19823, t19827, t19831, t19834, t19836, t19839)
}
