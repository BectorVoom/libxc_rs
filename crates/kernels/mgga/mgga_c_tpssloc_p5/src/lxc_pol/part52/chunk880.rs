//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 880/1400 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk880<F: Float>(t28: F, t265: F, t504: F, t1877: F, t25: F, t8366: F, t8370: F, t202: F, t8365: F, t8369: F, t193: F, t2752: F, t870: F, t52: F, dens_threshold: F, rho1: F, zeta_threshold: F) -> (F, F, F, F, F, F) {
    let t29 = t28 <= zeta_threshold;
    let t401 = rho1 <= dens_threshold || t29;
    let t505 = t265 < t504;
    let t8374 = t1877 * t8366 * t25 / F::cast_from(2.0_f64) - t1877 * t8370 * t25 / F::cast_from(2.0_f64);
    let t8418 = t202 * t8365;
    let t8421 = t202 * t8369;
    let t8424 = -t193 * t2752 * t8421 + t193 * t8418 * t870;
    let t8434 = t1877 * t8366 * t28 / F::cast_from(2.0_f64) - t1877 * t8370 * t28 / F::cast_from(2.0_f64);
    let t8435 = piecewise3::<F>(t505, F::cast_from(0.0_f64), t8424);
    let t8438 = piecewise3::<F>(t401, t8434, t8435 * t52 / F::cast_from(2.0_f64));
    (t8374, t8418, t8421, t8424, t8435, t8438)
}
