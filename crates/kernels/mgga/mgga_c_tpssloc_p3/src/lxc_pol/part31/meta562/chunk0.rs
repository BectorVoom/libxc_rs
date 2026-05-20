//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1791/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1791<F: Float>(t81575: F, t25251: F, t87049: F, t23012: F, t7529: F, t23110: F, t23185: F, t25241: F, t1484: F, t852: F, t81595: F, t81602: F) -> (F, F, F, F, F, F, F) {
    let t87073 = F::cast_from(0.3289868133696452873e-1_f64) * t81575;
    let t87078 = t87049 * t25251;
    let t87080 = t23012 * t7529;
    let t87100 = t23185 * t23110 * t25241;
    let t87111 = t852 * t1484;
    let t87119 = F::cast_from(0.16449340668482264365e-1_f64) * t81595;
    let t87127 = F::cast_from(0.12793931631041761173e0_f64) * t81602;
    (t87073, t87078, t87080, t87100, t87111, t87119, t87127)
}
