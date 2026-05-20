//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2153/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2153<F: Float>(t10475: F, t42342: F, t42345: F, t2770: F, t283: F, t10309: F, t1041: F, t10457: F, t248: F, t10444: F, t354: F, t364: F, t372: F) -> (F, F, F, F) {
    let t43385 = t42342 * t10475 * t42345;
    let t43398 = F::new(1.0) / t283 / t2770;
    let t43406 = t1041 * t248 * t10457 * t10309;
    let t43410 = t354 * t364 * t10444 * t372;
    (t43385, t43398, t43406, t43410)
}
