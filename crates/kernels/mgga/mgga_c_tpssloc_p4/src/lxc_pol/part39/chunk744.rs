//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 39 (v4rho3tau_3) CSE chunk 744/1328 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part39_v4rho3tau_3_chunk744<F: Float>(t225: F, t3817: F, t3837: F, t1365: F, t68: F, t3734: F, t1347: F, t3719: F, t1345: F, t1348: F, t546: F, t548: F) -> (F, F, F, F) {
    let t3839 = (t3817 + t3837) * t225;
    let t3843 = t68 * t1365;
    let t3844 = t3843 * t3734;
    let t3847 = t1347 * t3719;
    let t3850 = F::cast_from(6.0_f64) * t1345 * t1348 - t3839 * t548 - F::cast_from(12.0_f64) * t3844 * t546 + F::cast_from(3.0_f64) * t3847 * t546;
    (t3839, t3844, t3847, t3850)
}
