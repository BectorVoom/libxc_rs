//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1090/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1090<F: Float>(t15043: F, t2741: F, t4826: F, t949: F, t8523: F, t361: F, t4988: F, t3933: F, t3931: F, t11586: F, t11590: F, t14999: F, t15002: F, t15005: F, t15012: F, t15018: F, t15021: F, t15028: F, t15032: F, t15036: F, t15040: F, t2722: F, t2740: F, t2748: F, t4980: F, t5001: F, t5009: F, t8509: F, t8972: F, t9033: F, t9038: F, t925: F, t967: F) -> (F, F) {
    let t15044 = t2741 * t15043;
    let t15047 = t4826 * t949;
    let t15048 = t8523 * t15047;
    let t15051 = t361 * t4988;
    let t15052 = t15051 * t3933;
    let t15053 = t3931 * t15052;
    let t15056 = -t925 * t14999 / F::new(72.0) - t925 * t15002 / F::new(144.0) + t925 * t15005 / F::new(216.0) - F::new(5.0) / F::new(2592.0) * t2748 * t5001 - t15012 / F::new(3456.0) - t2748 * t5009 / F::new(864.0) + t15018 / F::new(6912.0) + t967 * t15021 / F::new(4608.0) + t11586 - t8972 * t4980 / F::new(288.0) + t15028 / F::new(2304.0) + t11590 + t9033 / F::new(2592.0) + t9038 + t2740 * t15032 / F::new(2304.0) + t2740 * t15036 / F::new(2304.0) + t2740 * t15040 / F::new(4608.0) - t8509 * t15044 / F::new(4608.0) + F::new(5.0) / F::new(13824.0) * t2740 * t15048 + t2722 * t15053 / F::new(1536.0);
    (t15051, t15056)
}
