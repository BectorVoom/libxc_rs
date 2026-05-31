//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 803/1049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk803<F: Float>(t25: F, t1409: F, t1965: F, t28469: F, t28756: F, t40: F, t5398: F, t7643: F, t28: F, t5527: F, t1915: F, t23788: F, t28248: F, dens_threshold: F, rho0: F, zeta_threshold: F) -> (F, F, F, F) {
    let t26 = t25 <= zeta_threshold;
    let t115 = rho0 <= dens_threshold || t26;
    let t28763 = piecewise3::<F>(t115, t28469, t28756 * t40 / F::cast_from(2.0_f64) + t7643 * t1409 + t1965 * t5398 / F::cast_from(2.0_f64));
    let t28764 = t28 * t5527;
    let t28765 = t1915 * t28764;
    let t28771 = t23788 * t28248;
    (t28763, t28764, t28765, t28771)
}
