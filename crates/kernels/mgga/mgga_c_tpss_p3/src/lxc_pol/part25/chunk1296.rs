//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1296/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1296<F: Float>(t19352: F, t5791: F, t18660: F, t6073: F, t19411: F, t19414: F, t19417: F, t6080: F, t18670: F, t19388: F, t42178: F, t5784: F) -> (F, F, F, F, F, F, F, F) {
    let t67389 = F::new(16.0) / F::new(9.0) * t19352 * t5791;
    let t67391 = F::new(16.0) / F::new(9.0) * t6073 * t18660;
    let t67429 = F::new(32.0) / F::new(9.0) * t19411 * t5791;
    let t67431 = F::new(32.0) / F::new(9.0) * t19414 * t5791;
    let t67433 = F::new(32.0) / F::new(9.0) * t19417 * t5791;
    let t67436 = F::new(32.0) / F::new(9.0) * t6080 * t18660;
    let t67440 = F::new(80.0) / F::new(9.0) * t18670 * t19388;
    let t67441 = t42178 * t5784;
    (t67389, t67391, t67429, t67431, t67433, t67436, t67440, t67441)
}
