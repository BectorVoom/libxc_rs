//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 540/1056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk540<F: Float>(t38: F, t43: F, t625: F, t44: F, t607: F, t614: F) -> (F, F) {
    let t6500 = t38 * t43;
    let t6503 = F::new(8.0) / F::new(3.0) * t625;
    let t6504 = -F::new(8.0) / F::new(3.0) * t614 * t44 + F::new(5.0) / F::new(6.0) * t6500 * t607 + t6503;
    (t6500, t6504)
}
