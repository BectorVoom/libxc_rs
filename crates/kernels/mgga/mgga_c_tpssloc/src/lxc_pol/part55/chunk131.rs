//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 131/1304 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk131<F: Float>(t281: F, t282: F, t415: F, t407: F, t410: F, t413: F, t409: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t417 = t281 * t282 * t415;
    let t419 = F::new(0.379785e1) * t410 + F::new(0.8969e0) * t407 + F::new(0.204775e0) * t413 + F::new(0.123235e0) * t417;
    let t422 = F::new(1.0) + F::new(0.16081979498692535067e2) / t419;
    let t423 = f64::ln(t422);
    let t425 = F::new(0.621814e-1) * t409 * t423;
    let t427 = F::new(1.0) + F::new(0.5137e-1) * t407;
    let t432 = F::new(0.705945e1) * t410 + F::new(0.1549425e1) * t407 + F::new(0.420775e0) * t413 + F::new(0.1562925e0) * t417;
    let t435 = F::new(1.0) + F::new(0.32163958997385070134e2) / t432;
    let t436 = f64::ln(t435);
    let t440 = F::new(1.0) + F::new(0.278125e-1) * t407;
    (t417, t419, t422, t423, t425, t427, t432, t435, t436, t440)
}
