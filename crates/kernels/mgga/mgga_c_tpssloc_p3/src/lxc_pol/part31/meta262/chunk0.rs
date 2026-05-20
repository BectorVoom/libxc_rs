//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1095/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1095<F: Float>(t6966: F, t6974: F, t1338: F, t2085: F) -> (F, F, F) {
    let t7202 = F::cast_from(0.38381794893125283518e-1_f64) * t6966;
    let t7204 = F::cast_from(0.82246703342411321825e-2_f64) * t6974;
    let t7208 = t1338 * t2085;
    (t7202, t7204, t7208)
}
