//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1462/1484 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1462<F: Float>(t5: F, t124814: F, t124860: F, t112: F, t104977: F, t117533: F, t122718: F, t122719: F, t122720: F, t122917: F, t122920: F, t124715: F, t124728: F, t1458: F, t2039: F, t24932: F, t27170: F, t27863: F, t27888: F, t32350: F, t33152: F, t33154: F, t33690: F, t4072: F, t671: F, t7056: F, t7266: F, t7801: F, t8446: F, t96238: F) -> (F, F) {
    let t7 = piecewise3::<F>(F::new(0.0) < t5, t5, -t5);
    let t8 = -t7 <= -F::cast_from(0.999999999999e0_f64);
    let t124862 = piecewise3::<F>(t8, F::new(0.0), t124814 + t124860);
    let t124863 = t124862 * t112;
    let t124867 = F::new(2.0) * t104977 * t2039 + F::new(2.0) * t117533 * t1458 + F::new(2.0) * t122917 * t2039 + F::new(2.0) * t122920 * t2039 + F::new(2.0) * t124715 * t1458 + F::new(2.0) * t124728 * t671 + F::new(2.0) * t2039 * t96238 + F::new(2.0) * t24932 * t7801 + F::new(2.0) * t27170 * t7266 + F::new(2.0) * t27863 * t7056 + F::new(2.0) * t27888 * t7801 + F::new(2.0) * t32350 * t4072 + F::new(2.0) * t33690 * t7056 + F::new(2.0) * t122718 + F::new(2.0) * t122719 + F::new(2.0) * t122720 + t124863 + t33152 + t33154 + t8446;
    (t124863, t124867)
}
