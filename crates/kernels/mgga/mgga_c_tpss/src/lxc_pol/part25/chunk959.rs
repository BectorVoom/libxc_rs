//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 959/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk959<F: Float>(t125: F, t4701: F, t2175: F, t783: F, t782: F, t8279: F, t14174: F, t3628: F, t3630: F, t10600: F, t10779: F, t14171: F, t14176: F, t14181: F, t14185: F, t14189: F, t14193: F, t14197: F, t14202: F, t2173: F, t3626: F) -> (F, F, F, F, F) {
    let t14205 = t125 * t4701;
    let t14207 = t2175 * t14205 * t783;
    let t14210 = t8279 * t782;
    let t14212 = t3628 * t14174 * t14210;
    let t14216 = t3628 * t14174 * t3630;
    let t14219 = t2173 * t14171 / 768.0 - t3626 * t14176 / 384.0 + t3626 * t14181 / 768.0 - t2173 * t14185 / 3072.0 + t2173 * t14189 / 768.0 - t2173 * t14193 / 1536.0 - t2173 * t14197 / 3072.0 - 5.0 / 768.0 * t2173 * t14202 + t2173 * t14207 / 768.0 - t10779 * t14212 / 512.0 + t10600 + t3626 * t14216 / 512.0;
    (t14207, t14210, t14212, t14216, t14219)
}
