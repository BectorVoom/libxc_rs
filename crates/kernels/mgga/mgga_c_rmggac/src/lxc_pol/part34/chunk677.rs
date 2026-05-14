//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 677/916 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk677<F: Float>(t15361: F, t495: F, t14230: F, t14237: F, t2067: F, t14225: F, t3352: F, t8496: F, t15399: F, t68764: F, t21719: F, t7248: F, t9050: F, t9054: F, t9188: F, t9095: F) -> (F, F, F, F, F, F) {
    let t74005 = t15361 * t495;
    let t74008 = t14230 * t14237 * t2067 * t74005;
    let t74013 = t14225 * t3352 * t8496;
    let t74015 = t68764 * t15399;
    let t74018 = t21719 * t7248 * t9050;
    let t74021 = t21719 * t9188 * t9054;
    let t74024 = t21719 * t3352 * t9095;
    (t74008, t74013, t74015, t74018, t74021, t74024)
}
