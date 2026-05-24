//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1254/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1254<F: Float>(t20154: F, t219: F, t6420: F, t1265: F, t18490: F, t6424: F, t18967: F, t19521: F, t1266: F, t1657: F, t1842: F, t18483: F, t18496: F, t18950: F, t19507: F, t19509: F, t4494: F, t4517: F, t538: F, t5739: F, t5921: F, t5925: F, t5930: F, t5933: F, t6260: F, t6425: F, param_beta: F) -> (F, F, F, F, F) {
    let t20155 = param_beta * t20154;
    let t20157 = t6420 * t219;
    let t20171 = t18490 * t6424 * t1265;
    let t20174 = t18967 * t19521;
    let t20177 = -t1266 * t20157 - t1657 * t18950 - t1842 * t19507 + F::new(2.0) * t18483 * t6425 - F::new(2.0) * t18496 * t20174 + F::new(2.0) * t19509 * t5925 + t19509 * t5930 + t20155 * t538 - F::new(6.0) * t20171 * t5739 + F::new(2.0) * t4494 * t5921 - t4517 * t5921 - t5933 * t6260;
    (t20155, t20157, t20171, t20174, t20177)
}
