//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 99/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk99<F: Float>(t147: F, t135: F, t12: F, t2: F, t246: F, t142: F, t34: F, t6: F, t8: F, t247: F, t250: F, t150: F) -> (F, F, F, F, F, F, F, F) {
    let t362 = t147 * t147;
    let t363 = F::cast_from(1.0_f64) / t362;
    let t364 = t135 * t363;
    let t367 = F::sqrt(t12);
    let t368 = t367 * t2;
    let t369 = t368 * t246;
    let t374 = t142 * t6 / t34 / t8;
    let t376 = -F::cast_from(0.632975e0_f64) * t247 - F::cast_from(0.29896666666666666667e0_f64) * t250 - F::cast_from(0.1023875e0_f64) * t369 - F::cast_from(0.82156666666666666667e-1_f64) * t374;
    let t377 = F::cast_from(1.0_f64) / t150;
    (t362, t363, t364, t368, t369, t374, t376, t377)
}
