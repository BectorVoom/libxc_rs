//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1055/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1055<F: Float>(t5: F, t2240: F, t31016: F, t6504: F, t8307: F, t8513: F, t31003: F, t641: F, t79: F, t31000: F, t31004: F, t31006: F, t31010: F, t31013: F, t8309: F) -> (F, F, F, F, F) {
    let t7 = piecewise3::<f64>(F::new(0.0) < t5, t5, -t5);
    let t8 = -t7 <= -F::new(0.999999999999e0);
    let t31017 = t2240 * t31016;
    let t31019 = t8513 * t8307 * t6504;
    let t31022 = t2240 * t31003;
    let t31024 = t8513 * t79 * t641;
    let t31028 = piecewise3::<f64>(t8, F::new(0.0), F::new(5.0) / F::new(144.0) * t31000 * t8309 - F::new(5.0) / F::new(24.0) * t31004 * t31006 - F::new(5.0) / F::new(36.0) * t31010 * t31013 + F::new(5.0) / F::new(72.0) * t31017 * t31019 + F::new(5.0) / F::new(72.0) * t31022 * t31024);
    (t31017, t31019, t31022, t31024, t31028)
}
