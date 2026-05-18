//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1332/1438 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1332<F: Float>(t10041: F, t6581: F, t213: F, t6589: F, t9223: F, t6593: F, t23062: F, t23066: F, t1894: F, t236: F, t6591: F, t9516: F) -> (F, F, F, F) {
    let t81930 = t6581 * t10041;
    let t81933 = t9223 * t6589 * t213;
    let t81934 = t81933 * t6593;
    let t81936 = t23062 * t23066;
    let t81940 = t6591 * t1894 * t236 * t9516;
    (t81930, t81934, t81936, t81940)
}
