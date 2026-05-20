//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2163/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2163<F: Float>(t16311: F, t3788: F, t5286: F, t6936: F, t28101: F, t80958: F, t1827: F, t91285: F, t22756: F, t6417: F, t19868: F, t6945: F) -> (F, F, F, F, F) {
    let t97236 = t6936 * t3788 * t16311 * t5286;
    let t97238 = t80958 * t28101;
    let t97240 = t91285 * t1827;
    let t97242 = t22756 * t6417;
    let t97244 = t6945 * t19868;
    (t97236, t97238, t97240, t97242, t97244)
}
