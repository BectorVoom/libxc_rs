//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1292/1415 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1292<F: Float>(t23384: F, t28496: F, t225: F, t28488: F, t28557: F, t381: F, t3173: F, t5919: F, t1921: F, t28702: F, t82431: F, t28510: F) -> (F, F, F, F, F, F) {
    let t99230 = t23384 * t28496;
    let t99248 = t28488 * t225;
    let t99273 = t28557 * t381;
    let t99296 = t3173 * t5919;
    let t99297 = t1921 * t99296;
    let t99301 = t82431 * t28702;
    let t99330 = t23384 * t28510;
    (t99230, t99248, t99273, t99297, t99301, t99330)
}
