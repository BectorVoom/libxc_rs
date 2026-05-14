//! MGGA_C_TPSS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1363/1368 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part21_v4rho3sigma_3_chunk1363<F: Float>(t1668: F, t18589: F, t116: F, t19596: F, t547: F, t645: F, t20124: F, t2105: F, t2061: F, t6112: F, t18599: F, t1279: F, t20125: F, t13265: F, t1786: F, t18596: F) -> (F, F, F, F, F, F, F, F) {
    let t66101 = 6.0 * t1668 * t18589;
    let t66108 = t116 * t19596;
    let t66111 = 12.0 * t547 * t66108 * t645;
    let t66114 = 6.0 * t547 * t20124 * t2105;
    let t66121 = 6.0 * t547 * t2061 * t6112;
    let t66123 = 3.0 * t1668 * t18599;
    let t66125 = 12.0 * t1279 * t20125;
    let t66127 = 3.0 * t13265 * t1786;
    let t66129 = 6.0 * t1668 * t18596;
    (t66101, t66111, t66114, t66121, t66123, t66125, t66127, t66129)
}
