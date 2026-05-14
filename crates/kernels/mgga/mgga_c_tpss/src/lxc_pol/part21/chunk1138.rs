//! MGGA_C_TPSS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1138/1368 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part21_v4rho3sigma_3_chunk1138<F: Float>(t12669: F, t13251: F, t3: F, t1338: F, t2061: F, t116: F, t3537: F, t645: F, t2105: F, t4555: F, t117: F, t13220: F, t1279: F, t1281: F, t1668: F, t1670: F, t3403: F, t3407: F, t3410: F, t4549: F, t4556: F, t4559: F, t547: F, t548: F) -> (F, F, F, F, F, F, F) {
    let t13252 = t12669 + t13251;
    let t13253 = t3 * t13252;
    let t13265 = param_d * t13252;
    let t13279 = t2061 * t1338;
    let t13282 = t116 * t3537;
    let t13283 = t13282 * t645;
    let t13286 = t4555 * t2105;
    let t13289 = t117 * t13220;
    let t13292 = 12.0 * t1279 * t4556 + 6.0 * t1279 * t4559 + 6.0 * t1281 * t4549 + t13265 * t548 + 6.0 * t13279 * t547 + 12.0 * t13283 * t547 + 6.0 * t13286 * t547 + 3.0 * t13289 * t547 + 6.0 * t1668 * t3407 + 3.0 * t1668 * t3410 + 3.0 * t1670 * t3403;
    (t13253, t13265, t13279, t13283, t13286, t13289, t13292)
}
