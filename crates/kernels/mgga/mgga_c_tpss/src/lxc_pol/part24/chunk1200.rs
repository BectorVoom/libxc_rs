//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1200/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk1200<F: Float>(t19917: F, t19955: F, t2814: F, t6185: F, t1485: F, t993: F, t1364: F, t1398: F, t14076: F, t1692: F, t1713: F, t18047: F, t18052: F, t19797: F, t198: F, t19802: F, t19809: F, t19818: F, t207: F, t2439: F, t3552: F, t3610: F, t3683: F, t3724: F, t5586: F, t5590: F, t6149: F, t750: F, t821: F, t823: F) -> (F, F, F, F) {
    let t19956 = t19917 + t19955;
    let t19960 = t6185 * t2814;
    let t19965 = t1485 * t993;
    let t20002 = t19797 * t198 * t207 * t823 + 3.0 * t1364 * t2439 * t5586 - t1398 * t1692 * t18047 - 3.0 * t14076 * t2439 * t5590 + 2.0 * t1692 * t18052 * t19818 - t1692 * t19802 * t821 - t1692 * t3724 * t5590 + 3.0 * t1713 * t2439 * t3610 + 6.0 * t1713 * t3552 * t3683 - 3.0 * t19809 * t2439 * t5590 + 3.0 * t2439 * t6149 * t750;
    (t19956, t19960, t19965, t20002)
}
