//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 959/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk959<F: Float>(t1125: F, t12359: F, t1501: F, t242: F, t9666: F, t3062: F, t4258: F, t1113: F, t1561: F, t1014: F, t450: F, t1557: F, t672: F, t1098: F, t3054: F, t1127: F, t2840: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t12361 = t1125 * t12359 / 3456.0;
    let t12367 = t242 * t9666 * t1501;
    let t12368 = t1125 * t12367;
    let t12371 = t4258 * t3062 / 432.0;
    let t12377 = t1561 * t1113;
    let t12378 = t450 * t1014;
    let t12384 = t672 * t1557;
    let t12385 = t1098 * t12384;
    let t12387 = t1561 * t3054;
    let t12399 = t1127 * t2840;
    (t12361, t12367, t12368, t12371, t12377, t12378, t12384, t12385, t12387, t12399)
}
