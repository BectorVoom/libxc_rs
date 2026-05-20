//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2174/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2174<F: Float>(t2281: F, t5465: F, t19474: F, t626: F, t19483: F, t19477: F, t1409: F, t628: F, t67: F, t19297: F, t604: F, t2239: F, t5385: F) -> (F, F, F, F, F, F, F) {
    let t55537 = t2281 * t5465;
    let t55546 = t626 * t19474;
    let t55559 = t626 * t19483;
    let t55561 = t626 * t19477;
    let t55653 = t1409 * t628 * t67;
    let t55880 = t19297 * t604;
    let t55921 = t5385 * t2239;
    (t55537, t55546, t55559, t55561, t55653, t55880, t55921)
}
