//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1075/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1075<F: Float>(t25: F, t265: F, t394: F, t31691: F, t641: F, t8513: F, t6534: F, t88: F, t30952: F, t30776: F, t40: F, t607: F, t8678: F, t30988: F, t191: F, t192: F, t7412: F, t2020: F, t6997: F, t8690: F, dens_threshold: F, rho0: F, zeta_threshold: F) -> (F, F, F, F, F, F, F) {
    let t26 = t25 <= zeta_threshold;
    let t115 = rho0 <= dens_threshold || t26;
    let t395 = t265 < t394;
    let t31693 = t8513 * t31691 * t641;
    let t31717 = t88 * t6534;
    let t31823 = piecewise3(t395, 0.0, t30952);
    let t31828 = piecewise3(t115, t30776, t31823 * t40 / 2.0 + t8678 * t607 / 2.0);
    let t31829 = t31828 + t30988;
    let t31832 = t7412 * t191 * t192;
    let t31833 = t31832 * t2020;
    let t31834 = t8690 * t6997;
    (t31693, t31717, t31823, t31829, t31832, t31833, t31834)
}
