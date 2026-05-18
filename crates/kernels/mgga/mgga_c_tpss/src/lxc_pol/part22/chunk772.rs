//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 772/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk772<F: Float>(t1081: F, t4180: F, t1542: F, t3001: F, t1080: F, t1054: F, t1063: F, t1073: F, t1082: F, t1531: F, t1543: F, t2925: F, t2930: F, t2955: F, t2969: F, t2974: F, t2999: F, t4062: F, t4065: F, t4067: F, t4070: F, t4107: F, t4111: F, t4117: F, t4120: F, t4125: F, t4143: F, t4147: F, t4156: F, t4158: F, t4163: F, t421: F) -> (F, F, F, F) {
    let t4181 = t4180 * t1081;
    let t4184 = t1542 * t3001;
    let t4185 = t4184 * t1080;
    let t4188 = -F::new(0.310907e-1) * t4117 * t421 + F::new(1.0) * t4120 * t1063 + F::new(1.0) * t2925 * t1531 - F::new(2.0) * t2930 * t4125 + F::new(1.0) * t1054 * t4143 + F::new(0.32163958997385070134e2) * t2955 * t4147 + t4062 - t4065 - t4067 + t4070 - t4107 - t4111 - F::new(0.19751673498613801407e-1) * t4156 + F::new(0.5848223622634646207e0) * t4158 * t1082 + F::new(0.5848223622634646207e0) * t2969 * t1543 - F::new(0.11696447245269292414e1) * t2974 * t4163 + F::new(0.5848223622634646207e0) * t1073 * t4181 + F::new(0.17315859105681463759e2) * t2999 * t4185;
    (t4181, t4184, t4185, t4188)
}
