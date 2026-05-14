//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 759/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk759<F: Float>(t1013: F, t5072: F, t128: F, t2835: F, t4044: F, t5066: F, t5070: F, t408: F, t1519: F, t4063: F, t1518: F, t1043: F, t2862: F, t1509: F, t2868: F, t2872: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t5073 = t1013 * t5072;
    let t5074 = t128 * t5073;
    let t5076 = t2835 - 0.11872222222222222222e-1 * t4044 - 0.11872222222222222222e-1 * t5066 + 0.35616666666666666666e-1 * t5070 + 0.17808333333333333333e-1 * t5074;
    let t5078 = 0.621814e-1 * t5076 * t408;
    let t5080 = 2.0 * t4063 * t1519;
    let t5081 = t1518 * t1518;
    let t5082 = t5081 * t1043;
    let t5084 = 2.0 * t2862 * t5082;
    let t5085 = t1509 * t1509;
    let t5086 = t2868 * t5085;
    let t5092 = t2872 - 2.0 / 9.0 * t4044 - 2.0 / 9.0 * t5066 + 2.0 / 3.0 * t5070 + t5074 / 3.0;
    (t5073, t5074, t5076, t5078, t5080, t5081, t5082, t5084, t5085, t5086, t5092)
}
