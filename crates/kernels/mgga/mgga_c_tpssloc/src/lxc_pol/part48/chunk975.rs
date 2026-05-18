//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 975/1034 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk975<F: Float>(t28: F, t265: F, t504: F, t115099: F, t115143: F, t115184: F, t2250: F, t31512: F, t52: F, t607: F, t8591: F, t113: F, t115107: F, t31540: F, t7057: F, dens_threshold: F, rho1: F, zeta_threshold: F) -> (F, F) {
    let t29 = t28 <= zeta_threshold;
    let t401 = rho1 <= dens_threshold || t29;
    let t505 = t265 < t504;
    let t115186 = piecewise3::<f64>(t505, F::new(0.0), t115099);
    let t115193 = piecewise3::<f64>(t401, t115143 + t115184, t115186 * t52 / F::new(2.0) - t31512 * t607 - t8591 * t2250 / F::new(2.0));
    let t115195 = t113 * (t115107 + t115193);
    let t115208 = F::new(4.0) * t31540 * t7057;
    (t115195, t115208)
}
