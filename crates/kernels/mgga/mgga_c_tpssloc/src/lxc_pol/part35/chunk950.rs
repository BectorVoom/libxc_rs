//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 950/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk950<F: Float>(t25: F, t28: F, t157: F, t20384: F, t20394: F, t182: F, t11987: F, t1298: F, t20216: F, t20376: F, t5170: F, t5397: F, t12000: F, t1302: F, t20385: F, t20390: F, t5178: F, t5966: F, zeta_threshold: F) -> (F, F, F, F) {
    let t26 = t25 <= zeta_threshold;
    let t29 = t28 <= zeta_threshold;
    let t20396 = (t20384 + t20394) * t157;
    let t20398 = F::new(0.19751673498613801407e-1) * t20396 * t182;
    let t20406 = piecewise3::<f64>(t26, F::new(0.0), F::new(8.0) / F::new(27.0) * t11987 * t20376 - F::new(2.0) / F::new(3.0) * t5170 * t5397 + F::new(2.0) / F::new(3.0) * t1298 * t20216);
    let t20414 = piecewise3::<f64>(t29, F::new(0.0), F::new(8.0) / F::new(27.0) * t12000 * t20385 - F::new(2.0) / F::new(3.0) * t5178 * t5966 + F::new(2.0) / F::new(3.0) * t1302 * t20390);
    (t20396, t20398, t20406, t20414)
}
