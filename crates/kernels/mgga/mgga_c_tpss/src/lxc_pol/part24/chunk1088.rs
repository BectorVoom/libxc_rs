//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1088/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk1088<F: Float>(t1072: F, t1081: F, t15351: F, t1089: F, t5301: F, t9519: F, t4063: F, t4105: F, t5082: F, t9507: F, t2857: F, t5114: F, t15248: F, t15251: F, t15292: F, t15294: F, t15296: F, t15299: F, t15301: F, t15304: F, t15307: F, t15309: F, t15312: F) -> (F, F, F, F, F, F) {
    let t15353 = t1072 * t15351 * t1081;
    let t15355 = 0.5848223622634646207e0 * t1089 * t15353;
    let t15356 = t5301 * t9519;
    let t15361 = 2.0 * t4063 * t4105;
    let t15363 = 2.0 * t9507 * t5082;
    let t15365 = 1.0 * t2857 * t5114;
    let t15385 = -0.54771111111111111111e-1 * t15248 + 0.29896666666666666667e0 * t15251 + 0.1898925e1 * t15292 + 0.3071625e0 * t15294 + 0.18257037037037037037e-1 * t15296 - 0.76790625e-1 * t15299 + 0.3071625e0 * t15301 + 0.15358125e0 * t15304 + 0.142419375e1 * t15307 - 0.1898925e1 * t15309 - 0.9494625e0 * t15312;
    (t15355, t15356, t15361, t15363, t15365, t15385)
}
