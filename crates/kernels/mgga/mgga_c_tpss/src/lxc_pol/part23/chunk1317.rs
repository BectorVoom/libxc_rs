//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1317/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk1317<F: Float>(t1279: F, t20113: F, t4549: F, t5773: F, t1338: F, t547: F, t62124: F, t18592: F, t3537: F, t1688: F, t2105: F, t5531: F, t645: F, t20112: F, t3403: F, t6290: F) -> (F, F, F, F, F, F, F, F) {
    let t66075 = 12.0 * t1279 * t20113;
    let t66077 = 12.0 * t4549 * t5773;
    let t66080 = 6.0 * t547 * t62124 * t1338;
    let t66083 = 12.0 * t547 * t18592 * t3537;
    let t66087 = 6.0 * t547 * t2105 * t1688 * t1338;
    let t66091 = 12.0 * t547 * t645 * t5531 * t1338;
    let t66094 = 12.0 * t547 * t20112 * t3537;
    let t66098 = 6.0 * t3403 * t6290;
    (t66075, t66077, t66080, t66083, t66087, t66091, t66094, t66098)
}
