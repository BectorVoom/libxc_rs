//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 954/1049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk954<F: Float>(t22986: F, t23270: F, t30622: F, t5544: F, t118649: F, t118532: F, t32844: F, t16891: F, t232: F, t30714: F, t4180: F, t112792: F, t16839: F, t2632: F) -> (F, F, F, F, F) {
    let t126290 = F::new(0.3289868133696452873e-1) * t22986 * t23270 * t30622 * t5544;
    let t126291 = F::new(0.15352717957250113407e0) * t118649;
    let t126294 = t118532 * t32844;
    let t126298 = t30714 * t4180 * t16891 * t232;
    let t126302 = t112792 * t4180 * t16839 * t2632;
    (t126290, t126291, t126294, t126298, t126302)
}
