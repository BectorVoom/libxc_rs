//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 39 (v4rho3tau_3) CSE chunk 1117/1190 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part39_v4rho3tau_3_chunk1117<F: Float>(t1706: F, t3545: F, t11818: F, t1735: F, t248: F, t1213: F, t11789: F, t1653: F, t1227: F, t15437: F, t3505: F, t3576: F, t5064: F, t13969: F, t4988: F, t15708: F, t4723: F) -> (F, F, F, F, F, F, F) {
    let t15727 = t1706 * t3545;
    let t15730 = t248 * t11818 * t1735;
    let t15731 = t1213 * t15730;
    let t15734 = t248 * t11789 * t1653;
    let t15735 = t1227 * t15734;
    let t15737 = t15437 * t3505;
    let t15740 = t5064 * t3576;
    let t15743 = t13969 * t4988;
    let t15745 = 5.0 / 10368.0 * t1227 * t15743;
    let t15749 = t4723 * t15708;
    (t15727, t15731, t15735, t15737, t15740, t15745, t15749)
}
