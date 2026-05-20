//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 792/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk792<F: Float>(t1409: F, t1419: F, t56: F, t6503: F, t7251: F, t67: F, t1864: F) -> (F, F, F) {
    let t7973 = -F::new(8.0) / F::new(3.0) * t1419 * t56 - F::new(5.0) / F::new(6.0) * t7251 * t1409 + t6503;
    let t7974 = t7973 * t67;
    let t7975 = t7974 * t1864;
    (t7973, t7974, t7975)
}
