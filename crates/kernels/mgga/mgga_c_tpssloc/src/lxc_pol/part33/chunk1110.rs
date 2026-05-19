//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1110/1415 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1110<F: Float>(t344: F, t381: F, t225: F, t1054: F, t883: F, t1922: F, t2966: F, t1920: F, t134: F, t221: F, t1926: F) -> (F, F, F, F) {
    let t23328 = t344 * t381;
    let t23329 = t23328 * t225;
    let t23330 = t1054 * t883;
    let t23357 = t2966 * t1922;
    let t23359 = F::cast_from(0.18277045187202515961e-2_f64) * t1920 * t23357;
    let t23383 = t221 * t134;
    let t23384 = t1926 * t23383;
    (t23329, t23330, t23359, t23384)
}
