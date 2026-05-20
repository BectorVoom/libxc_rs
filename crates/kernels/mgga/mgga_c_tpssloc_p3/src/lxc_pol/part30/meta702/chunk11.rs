//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2284/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2284<F: Float>(t17673: F, t17984: F, t25589: F, t4596: F, t4600: F, t7578: F, t83054: F, t83058: F, t88320: F, t88321: F, t88324: F, t88335: F, t88336: F, t88339: F, t88594: F, t88600: F) -> F {
    let t99571 = t83054 * t17673 / F::new(256.0) - t83058 * t17984 / F::new(256.0) + t88594 * t4596 / F::new(384.0) - t88600 * t4600 / F::new(768.0) - F::cast_from(0.20186378047070195428e-3_f64) * t25589 * t7578 + t88320 - t88321 / F::new(5184.0) + t88324 - t88335 - t88336 / F::new(648.0) + t88339;
    t99571
}
