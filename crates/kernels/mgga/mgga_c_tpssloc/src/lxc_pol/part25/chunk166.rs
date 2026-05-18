//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 166/1226 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk166<F: Float>(t25: F, t28: F, t17: F, t522: F, t182: F, t521: F, t514: F, t194: F, t517: F, zeta_threshold: F) -> (F, F, F, F, F) {
    let t26 = t25 <= zeta_threshold;
    let t29 = t28 <= zeta_threshold;
    let t523 = t17 * t522;
    let t525 = F::new(0.19751673498613801407e-1) * t521 * t182;
    let t526 = t514 * t514;
    let t527 = piecewise3::<f64>(t26, t194, t526);
    let t528 = t517 * t517;
    let t529 = piecewise3::<f64>(t29, t194, t528);
    let t531 = t527 / F::new(2.0) + t529 / F::new(2.0);
    (t523, t525, t526, t528, t531)
}
