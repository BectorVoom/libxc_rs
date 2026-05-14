//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1229/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1229<F: Float>(t20284: F, t71: F, t33: F, t75284: F, t1437: F, t5441: F, t72: F, t3953: F, t5392: F, t1860: F, t1863: F, t1864: F, t1865: F, t20217: F, t20234: F, t21510: F, t22505: F, t26084: F, t27949: F, t27950: F, t27953: F, t27956: F, t27957: F, t27972: F, t6490: F, t6500: F, t67: F, t7428: F, t7435: F, t7441: F, t7445: F, t83796: F, t83803: F) -> (F,) {
    let t106800 = t71 * t20284;
    let t106804 = t75284 * t33;
    let t106813 = t72 * t5441 * t1437;
    let t106816 = t3953 * t5392;
    let t106819 = -t7428 * t27957 / 2.0 - t1860 * (-5.0 / 108.0 * t83796 * t20234 + 5.0 / 6.0 * t22505 * t21510 + 5.0 / 6.0 * t6500 * t20217 + t83803) * t67 * t1864 / 6.0 - t1860 * t27949 * t7445 / 2.0 - t1860 * t7441 * t27956 / 2.0 - t1860 * t1863 * t106800 / 6.0 - t106804 * t1865 / 6.0 + t7435 * t27950 + 5.0 * t26084 * t27972 + 2.0 * t7435 * t27953 + 5.0 / 2.0 * t6490 * t106813 + t106816 * t1865 + t7435 * t27957;
    (t106819,)
}
