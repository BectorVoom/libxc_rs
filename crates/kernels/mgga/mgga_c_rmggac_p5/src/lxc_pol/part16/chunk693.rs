//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 693/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk693<F: Float>(t236: F, t9988: F, t7231: F, t7230: F, t1743: F, t645: F, t903: F, t6108: F, t1971: F, t7365: F, t6182: F, t1970: F) -> (F, F, F, F, F, F, F, F) {
    let t9989 = t236 * t9988;
    let t9990 = t7231 * t9989;
    let t9991 = t7230 * t9990;
    let t9999 = t645 * t1743;
    let t10000 = t903 * t9999;
    let t10013 = t236 * t6108;
    let t10014 = t1971 * t10013;
    let t10015 = t7365 * t10014;
    let t10017 = t236 * t6182;
    let t10018 = t1971 * t10017;
    let t10019 = t1970 * t10018;
    (t9990, t9991, t9999, t10000, t10014, t10015, t10018, t10019)
}
