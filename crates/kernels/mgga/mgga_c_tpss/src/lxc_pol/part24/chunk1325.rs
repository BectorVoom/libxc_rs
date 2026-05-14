//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1325/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk1325<F: Float>(t21398: F, t5638: F, t4977: F, t5640: F, t15076: F, t15135: F, t15143: F, t15155: F, t15186: F, t1730: F, t1731: F, t18133: F, t18156: F, t18171: F, t18178: F, t19901: F, t19904: F, t19910: F, t19914: F, t19927: F, t19928: F, t19936: F, t21390: F, t21399: F, t21424: F, t21434: F, t2799: F, t342: F, t345: F, t347: F, t4016: F, t5037: F, t5626: F, t5631: F, t5632: F, t5639: F, t5643: F, t5646: F, t61285: F, t61476: F, t61489: F, t61540: F, t6167: F, t6180: F, t64529: F, t64565: F, t64645: F, t70434: F, t70462: F, t70487: F, t70527: F, t990: F) -> (F, F) {
    let t70553 = t21398 * t5638;
    let t70560 = t5640 * t4977;
    let t70597 = 4.0 * t64529 * t19914 + t18171 * t18178 * t21434 - 12.0 * t19901 * t19910 + 2.0 * t5631 * t5632 * t21390 * t990 - t70553 * t5643 + 4.0 * t5626 * t15135 - 12.0 * t64565 * t70527 * t19928 - 2.0 * t61540 * t70560 * t2799 * t990 + 2.0 * t18171 * t19927 * t15186 + 4.0 * t18156 * t70462 * t19928 - 6.0 * t61285 * t61489 * t4977 * t15143 + 6.0 * t61285 * t70487 * t15155 - 2.0 * t64645 * t6180 - t21399 * t5646 - t5639 * t5640 * t15076 * t342 * t345 - 2.0 * t19904 * t19936 - t18133 * t5037 + 4.0 * t5631 * t5632 * t6167 * t4016 - 2.0 * t61476 * t21424 - t1730 * t1731 * t347 * t70434;
    (t70560, t70597)
}
