//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 932/1226 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk932<F: Float>(t28: F, t1081: F, t3711: F, t11122: F, t12000: F, t12001: F, t1302: F, t3231: F, t11997: F, zeta_threshold: F) -> F {
    let t29 = t28 <= zeta_threshold;
    let t12004 = t3711 * t1081;
    let t12010 = piecewise3::<F>(t29, F::new(0.0), F::new(8.0) / F::new(27.0) * t12000 * t12001 - F::new(2.0) / F::new(3.0) * t12004 * t3231 + F::new(2.0) / F::new(3.0) * t1302 * t11122);
    let t12012 = t11997 / F::new(2.0) + t12010 / F::new(2.0);
    t12012
}
