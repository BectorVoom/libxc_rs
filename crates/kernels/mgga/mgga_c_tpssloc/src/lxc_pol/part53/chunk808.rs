//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 808/939 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk808<F: Float>(t32211: F, t32280: F, t3: F, t112: F, t8811: F, t2039: F, t7056: F, t12524: F, t20173: F, t24462: F, t24465: F, t31284: F, t31287: F, t3941: F, t577: F, t671: F, t7230: F, t7235: F, t8508: F, t8717: F) -> (F, F, F, F, F) {
    let t32281 = t32211 + t32280;
    let t32282 = t3 * t32281;
    let t32295 = t8811 * t112;
    let t32308 = t2039 * t7056;
    let t32311 = 0.45e1 * t32281 * t577 + 0.135e2 * t32295 * t671 + 27.0 * t24462 * t2039 + 54.0 * t24465 * t7235 + 27.0 * t7230 * t7056 + 27.0 * t12524 * t8717 + 27.0 * t20173 * t8717 + 54.0 * t3941 * t32308 + t31284 + t31287 + t8508;
    (t32281, t32282, t32295, t32308, t32311)
}
