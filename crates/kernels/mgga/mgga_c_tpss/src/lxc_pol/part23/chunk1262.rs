//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1262/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk1262<F: Float>(t1692: F, t1713: F, t7622: F, t3724: F, t580: F, t1288: F, t2433: F, t19817: F, t44474: F, t2133: F, t17929: F, t18043: F, t18047: F, t18052: F, t19678: F, t19681: F, t19829: F, t19836: F, t2439: F, t5586: F, t5590: F, t61269: F, t63766: F, t63771: F, t63782: F, t63787: F) -> (F, F) {
    let t63790 = 3.0 * t1692 * t1713 * t7622;
    let t63791 = t580 * t3724;
    let t63794 = t1288 * t2433;
    let t63797 = t19817 * t44474;
    let t63806 = t1288 * t2133;
    let t63810 = -3.0 * t17929 * t63766 - t1692 * t18047 * t19836 - t1692 * t5590 * t63771 / 2.0 + 3.0 * t2439 * t5586 * t19681 - 3.0 * t61269 * t19678 + t63782 + t63787 - t63790 - t1692 * t5590 * t63791 + t1692 * t18052 * t63794 + 3.0 * t17929 * t63797 + t1692 * t18043 * t1288 / 2.0 + 3.0 * t2439 * t5586 * t19829 + 3.0 / 2.0 * t2439 * t1713 * t63806;
    (t63790, t63810)
}
