//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1381/1484 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1381<F: Float>(t33395: F, t814: F, t22986: F, t2647: F, t26656: F, t6646: F, t1484: F, t2047: F, t829: F, t22893: F, t23164: F, t33375: F) -> (F, F, F, F, F) {
    let t121488 = t814 * t33395;
    let t121493 = t22986 * t6646 * t26656 * t2647;
    let t121495 = t2047 * t1484;
    let t121498 = t22986 * t6646 * t121495 * t829;
    let t121501 = t23164 * t22893 * t33375;
    (t121488, t121493, t121495, t121498, t121501)
}
