//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 817/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk817<F: Float>(t33: F, t750: F, t821: F, t1006: F, t1692: F, t1713: F, t2439: F, t5586: F, t5590: F, t1688: F, t2056: F, t4347: F, t1165: F, t5531: F, t1168: F, t196: F) -> (F, F, F, F, F, F, F) {
    let t5671 = t33 * t750;
    let t5678 = t33 * t821;
    let t5685 = 3.0 / 2.0 * t2439 * t1713 * t5671 + t1692 * t5586 * t33 / 2.0 - t1692 * t5590 * t5678 / 2.0 + t1692 * t1713 * t1006 / 2.0;
    let t5697 = 2.0 * t2056 * t1688;
    let t5699 = 2.0 * t4347 * t1688;
    let t5701 = 2.0 * t1165 * t5531;
    let t5705 = t1168 * t196;
    (t5671, t5678, t5685, t5697, t5699, t5701, t5705)
}
