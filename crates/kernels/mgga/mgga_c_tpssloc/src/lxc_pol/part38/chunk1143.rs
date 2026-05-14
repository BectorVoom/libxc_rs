//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 38 (v4rho3tau_2) CSE chunk 1143/1193 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part38_v4rho3tau_2_chunk1143<F: Float>(t1345: F, t1348: F, t16176: F, t16186: F, t16192: F, t16196: F, t16199: F, t16202: F, t1819: F, t1821: F, t3839: F, t3844: F, t3847: F, t5272: F, t5278: F, t5280: F, t5283: F, t546: F, t548: F) -> (F,) {
    let t16205 = 6.0 * t1345 * t5283 + 6.0 * t1348 * t5272 - t16176 * t548 - 24.0 * t16186 * t5280 + 60.0 * t16192 * t5278 - 24.0 * t16196 * t5278 - 12.0 * t16199 * t5278 + 3.0 * t16202 * t546 - 12.0 * t1819 * t3844 + 3.0 * t1819 * t3847 + 3.0 * t1821 * t3839;
    (t16205,)
}
