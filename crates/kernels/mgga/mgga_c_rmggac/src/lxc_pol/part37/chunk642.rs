//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 642/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk642<F: Float>(t13819: F, t7757: F, t1985: F, t3814: F, t14224: F, t7229: F, t3114: F, t3124: F, t70186: F, t14240: F, t68524: F, t14245: F, t14229: F, t8516: F, t69179: F, t739: F) -> (F, F, F, F, F, F, F, F) {
    let t70387 = t13819 * t7757;
    let t70397 = t1985 * t3814;
    let t70423 = t7229 * t14224;
    let t70439 = t3114 * t70186 * t3124;
    let t70441 = t68524 * t14240;
    let t70442 = 0.29085809927086856922e-4 * t70441;
    let t70443 = t68524 * t14245;
    let t70444 = 0.87257429781260570766e-4 * t70443;
    let t70460 = t8516 * t14229;
    let t70479 = 0.2927036860455597649e0 * t739 * t69179;
    (t70387, t70397, t70423, t70439, t70442, t70444, t70460, t70479)
}
