//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1840/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1840<F: Float>(t12571: F, t23966: F, t6492: F, t7432: F, t84195: F, t23967: F, t26067: F, t23993: F, t7428: F, t23998: F, t1860: F, t23992: F, t7445: F) -> (F, F, F, F, F, F, F) {
    let t91957 = t12571 * t23966;
    let t91959 = F::new(80.0) / F::new(9.0) * t91957 * t6492;
    let t91961 = F::new(80.0) / F::new(9.0) * t84195 * t7432;
    let t91980 = F::new(80.0) / F::new(9.0) * t23967 * t26067;
    let t91996 = t7428 * t23993;
    let t92001 = F::new(16.0) / F::new(9.0) * t7428 * t23998;
    let t92003 = t1860 * t23992 * t7445;
    (t91957, t91959, t91961, t91980, t91996, t92001, t92003)
}
