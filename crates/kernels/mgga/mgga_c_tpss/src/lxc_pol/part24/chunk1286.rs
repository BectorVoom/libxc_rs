//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1286/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk1286<F: Float>(t5: F, t69117: F, t69150: F, t69181: F, t69217: F, t69258: F, t69284: F, t69326: F, t69360: F, t117: F, t68898: F, t69062: F, t69064: F, t69066: F, t69068: F, t69071: F, t69074: F, t69076: F, t69078: F, t69080: F, t69082: F, t69084: F, t69086: F) -> (F, F) {
    let t7 = piecewise3(0.0 < t5, t5, -t5);
    let t8 = -t7 <= -0.999999999999e0;
    let t69364 = piecewise3(t8, 0.0, t69117 + t69150 + t69181 + t69217 + t69258 + t69284 + t69326 + t69360);
    let t69365 = t69364 * t117;
    let t69367 = t69062 + t69064 + t69066 + t69068 + t69071 + t69074 + t69076 + t69078 + t69080 + t69082 + t69084 + t69086 + t69365 + 2.0 * t68898;
    (t69365, t69367)
}
