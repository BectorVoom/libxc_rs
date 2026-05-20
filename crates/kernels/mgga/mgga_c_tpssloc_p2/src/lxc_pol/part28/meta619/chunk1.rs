//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1939/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1939<F: Float>(t16311: F, t3788: F, t3791: F, t6936: F, t1339: F, t1825: F, t26288: F, t3734: F, t16314: F, t26309: F, t16227: F, t22833: F) -> (F, F, F, F) {
    let t91241 = t6936 * t3788 * t16311 * t3791;
    let t91256 = t26288 * t1339 * t1825 * t3734;
    let t91261 = t26309 * t16314;
    let t91263 = t22833 * t16227;
    (t91241, t91256, t91261, t91263)
}
