//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1240/1291 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1240<F: Float>(t23669: F, t995: F, t6802: F, t3158: F, t6796: F, t10481: F, t1945: F, t23665: F, t23674: F, t23600: F, t23680: F, t23606: F, t10348: F, t1058: F, t1060: F, t11065: F, t11066: F, t1949: F, t23327: F, t23346: F, t23613: F, t23647: F, t23685: F, t23686: F, t23714: F, t23715: F, t2776: F, t3010: F, t3120: F, t6687: F, t6768: F, t6784: F, t6805: F) -> (F, F) {
    let t82713 = t23669 * t995;
    let t82714 = t82713 * t6802;
    let t82716 = t6796 * t3158;
    let t82717 = t82716 * t6802;
    let t82730 = t1945 * t10481;
    let t82734 = t23665 * t23674;
    let t82736 = t23600 * t995;
    let t82737 = t82736 * t23680;
    let t82739 = t82736 * t23606;
    let t82749 = 0.43864908449286038307e-1 * t23346 * t23715 - 0.16449340668482264365e-1 * t23327 * t23613 * t23686 - 0.43864908449286038307e-1 * t82714 - 0.54831135561607547884e-2 * t82717 + 0.16449340668482264365e-1 * t23327 * t23613 * t23714 - 0.24674011002723396548e-1 * t6687 * t3010 * t6805 - 0.82246703342411321825e-2 * t6687 * t10348 * t1949 + 0.13159472534785811492e0 * t23346 * t23647 - 6.0 * t11065 * t82730 * t11066 + 0.82246703342411321826e-2 * t82734 + 0.16449340668482264365e-1 * t82737 - 0.82246703342411321826e-2 * t82739 - 0.16449340668482264365e-1 * t6687 * t6784 * t23685 * t2776 + 3.0 * t1058 * t6768 * t3120 * t1060;
    (t82730, t82749)
}
