//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2940/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2940<F: Float>(t10186: F, t13798: F, t13851: F, t13861: F, t17791: F, t17821: F, t2986: F, t42903: F, t42906: F, t42911: F, t42914: F, t4510: F, t4518: F, t59668: F, t59672: F, t59696: F, t59725: F, t59742: F) -> F {
    let t61241 = -F::cast_from(0.11111111111111111111e-2_f64) * t2986 * t13851 * t13861 - F::cast_from(0.6172839506172839506e-3_f64) * t42903 + F::cast_from(0.18518518518518518518e-3_f64) * t42906 - F::cast_from(0.98765432098765432096e-3_f64) * t42911 + F::cast_from(0.18518518518518518518e-3_f64) * t42914 - F::cast_from(0.55555555555555555554e-3_f64) * t2986 * t4518 * t59696 - F::cast_from(0.22222222222222222221e-2_f64) * t2986 * t4510 * t59742 + F::cast_from(0.74074074074074074072e-3_f64) * t2986 * t4510 * t59668 + F::cast_from(0.37037037037037037036e-3_f64) * t2986 * t4510 * t59672 + F::cast_from(0.86419753086419753084e-3_f64) * t2986 * t13798 * t59725 + F::cast_from(0.29629629629629629628e-2_f64) * t10186 * t17821 - F::cast_from(0.19753086419753086419e-2_f64) * t10186 * t17791;
    t61241
}
