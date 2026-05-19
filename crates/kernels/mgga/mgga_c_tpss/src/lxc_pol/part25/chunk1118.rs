//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1118/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1118<F: Float>(t1072: F, t1081: F, t15351: F, t1089: F, t5301: F, t9519: F, t4063: F, t4105: F, t5082: F, t9507: F, t2857: F, t5114: F) -> (F, F, F, F, F) {
    let t15353 = t1072 * t15351 * t1081;
    let t15355 = F::cast_from(0.5848223622634646207e0_f64) * t1089 * t15353;
    let t15356 = t5301 * t9519;
    let t15361 = F::new(2.0) * t4063 * t4105;
    let t15363 = F::new(2.0) * t9507 * t5082;
    let t15365 = F::new(1.0) * t2857 * t5114;
    (t15355, t15356, t15361, t15363, t15365)
}
