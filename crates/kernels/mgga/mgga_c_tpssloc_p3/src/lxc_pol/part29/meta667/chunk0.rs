//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2226/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2226<F: Float>(t16413: F, t1985: F, t1998: F, t214: F, t16248: F, t22833: F, t16383: F, t16261: F, t26309: F, t22832: F, t5234: F, t3809: F) -> (F, F, F, F, F) {
    let t91091 = t1985 * t214 * t1998 * t16413;
    let t91094 = t22833 * t16248;
    let t91096 = t22833 * t16383;
    let t91098 = t26309 * t16261;
    let t91100 = t5234 * t22832;
    let t91101 = t91100 * t3809;
    (t91091, t91094, t91096, t91098, t91101)
}
