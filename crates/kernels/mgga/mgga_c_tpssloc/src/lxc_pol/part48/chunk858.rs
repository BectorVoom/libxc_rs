//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 858/1034 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk858<F: Float>(t28: F, t265: F, t504: F, t23788: F, t31441: F, t25927: F, t31448: F, t1081: F, t1914: F, t31477: F, t1877: F, t24191: F, t24339: F, t2522: F, t26756: F, t30974: F, t31430: F, t31434: F, t52: F, t607: F, t6841: F, t6848: F, t7114: F, t8566: F, t8586: F, t8591: F, dens_threshold: F, rho1: F, zeta_threshold: F) -> (F, F, F, F, F) {
    let t29 = t28 <= zeta_threshold;
    let t401 = rho1 <= dens_threshold || t29;
    let t505 = t265 < t504;
    let t31496 = t23788 * t31441;
    let t31502 = t25927 * t31448;
    let t31504 = t1081 * t1914;
    let t31512 = piecewise3::<f64>(t505, F::new(0.0), t31477);
    let t31517 = piecewise3::<f64>(t401, F::new(3.0) / F::new(2.0) * t2522 * t8566 * t6841 + t1877 * t31430 * t28 / F::new(2.0) - t1877 * t31434 * t6848 / F::new(2.0) + t1877 * t8566 * t1081 / F::new(2.0) - F::new(3.0) / F::new(2.0) * t24191 * t31496 - t1877 * t24339 * t8586 / F::new(2.0) + t26756 * t31502 - t1877 * t7114 * t31504 / F::new(2.0) - t1877 * t7114 * t30974 / F::new(2.0), t31512 * t52 / F::new(2.0) - t8591 * t607 / F::new(2.0));
    (t31496, t31502, t31504, t31512, t31517)
}
