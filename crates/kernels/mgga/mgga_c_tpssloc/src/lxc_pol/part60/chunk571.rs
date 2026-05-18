//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 571/1064 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk571<F: Float>(t2136: F, t7313: F, t2147: F, t478: F, t2131: F, t6739: F, t2133: F, t461: F, t1009: F, t1209: F, t1211: F, t1207: F, sigma2: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t7315 = F::new(0.10093189023535097714e-3) * t7313 * t2136;
    let t7320 = t2147 * t478;
    let t7324 = t2131 * t6739;
    let t7325 = t2133 * t461;
    let t7326 = t7324 * t7325;
    let t7327 = t1009 * t1209;
    let t7328 = t7327 * t478;
    let t7337 = t1209 * sigma2;
    let t7338 = t7337 * t1211;
    let t7339 = t1207 * t7338;
    (t7315, t7320, t7324, t7325, t7326, t7327, t7328, t7337, t7338, t7339)
}
