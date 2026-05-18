//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 660/1438 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk660<F: Float>(t25: F, t28: F, t515: F, t1298: F, t2249: F, t3665: F, t518: F, t1302: F, t3231: F, t3673: F, zeta_threshold: F) -> (F, F, F) {
    let t26 = t25 <= zeta_threshold;
    let t29 = t28 <= zeta_threshold;
    let t3704 = F::new(1.0) / t515;
    let t3710 = piecewise3::<f64>(t26, F::new(0.0), -F::new(2.0) / F::new(9.0) * t3704 * t3665 + F::new(2.0) / F::new(3.0) * t1298 * t2249);
    let t3711 = F::new(1.0) / t518;
    let t3717 = piecewise3::<f64>(t29, F::new(0.0), -F::new(2.0) / F::new(9.0) * t3711 * t3673 + F::new(2.0) / F::new(3.0) * t1302 * t3231);
    let t3719 = t3710 / F::new(2.0) + t3717 / F::new(2.0);
    (t3704, t3711, t3719)
}
