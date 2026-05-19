//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1151/1304 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1151<F: Float>(t2006: F, t212: F, t22642: F, t6890: F, t3886: F, t6992: F, t22716: F, t8459: F, t22779: F, t31162: F, t22817: F, t794: F, t8462: F) -> (F, F, F, F, F) {
    let t113941 = F::cast_from(0.16449340668482264365e-1_f64) * t22642 * t212 * t2006 * t6890;
    let t113946 = t3886 * t6992;
    let t113963 = F::cast_from(0.12793931631041761173e0_f64) * t22716 * t8459;
    let t113966 = t22779 * t31162;
    let t113981 = t22817 * t794 * t8462;
    (t113941, t113946, t113963, t113966, t113981)
}
