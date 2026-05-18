//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 711/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk711<F: Float>(t1300: F, t1303: F, t2024: F, t44: F, t4589: F, t4592: F, t4597: F, t4602: F, t4605: F, t56: F, t61: F, t38: F) -> (F, F) {
    let t4608 = F::new(5.0) / F::new(18.0) * t44 * t4589 + F::new(5.0) / F::new(6.0) * t44 * t4592 + F::new(88.0) / F::new(9.0) * t4597 * t61 + F::new(40.0) / F::new(9.0) * t1300 * t1303 + F::new(5.0) / F::new(18.0) * t56 * t4602 - F::new(5.0) / F::new(6.0) * t56 * t4605 - t2024;
    let t4609 = t38 * t4608;
    (t4608, t4609)
}
