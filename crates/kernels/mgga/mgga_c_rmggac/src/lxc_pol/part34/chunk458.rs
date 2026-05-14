//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 458/916 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk458<F: Float>(t14019: F, t14022: F, t14027: F, t217: F, t3127: F, t3131: F, t3119: F, t128: F, t446: F, t118: F, t13862: F, t3129: F, t4441: F, t3128: F, t1996: F, t202: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t14028 = t14019 * t14022 * t14027;
    let t14030 = t217 * t3127;
    let t14031 = t14030 * t3131;
    let t14032 = t14031 * t3119;
    let t14033 = t128 * t446;
    let t14034 = t118 * t14033;
    let t14035 = t13862 * t14034;
    let t14036 = t14032 * t14035;
    let t14039 = 1.0 / t3129 / t4441;
    let t14040 = t3128 * t14039;
    let t14041 = t14040 * t3119;
    let t14042 = t13862 * t1996;
    let t14043 = t14041 * t14042;
    let t14045 = t217 * t202;
    (t14028, t14030, t14031, t14032, t14034, t14035, t14036, t14039, t14040, t14041, t14042, t14043, t14045)
}
