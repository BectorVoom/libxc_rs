//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 1350/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1350<F: Float>(t11588: F, t1184: F, t1239: F, t68: F, t1203: F, t3540: F, t2393: F, t374: F, t486: F, t485: F, t3576: F, t3604: F) -> (F, F, F, F, F, F, F, F) {
    let t11589 = t11588 * t1184;
    let t11604 = t1239 * t1239;
    let t11605 = F::new(1.0) / t11604;
    let t11606 = t68 * t11605;
    let t11644 = t1203 * t3540;
    let t11647 = t374 * t2393 * t486;
    let t11649 = t485 * t11647 / F::new(10368.0);
    let t11665 = t3604 * t3576;
    (t11589, t11604, t11605, t11606, t11644, t11647, t11649, t11665)
}
