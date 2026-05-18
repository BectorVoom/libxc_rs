//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 709/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk709<F: Float>(t3455: F, t581: F, t3431: F, t60: F, t1294: F, t1300: F, t2024: F, t3447: F, t3450: F, t44: F, t56: F, t589: F, t595: F) -> F {
    let t3456 = t3455 * t581;
    let t3459 = t60 * t3431;
    let t3462 = -F::new(20.0) / F::new(9.0) * t589 * t1294 + F::new(5.0) / F::new(18.0) * t44 * t3447 + F::new(5.0) / F::new(6.0) * t44 * t3450 + F::new(20.0) / F::new(9.0) * t1300 * t595 + F::new(5.0) / F::new(18.0) * t56 * t3456 - F::new(5.0) / F::new(6.0) * t56 * t3459 - t2024;
    t3462
}
