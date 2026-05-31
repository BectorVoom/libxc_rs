//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 1775/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1775<F: Float>(t10007: F, t2645: F, t4181: F, t4191: F, t9638: F, t13275: F, t13277: F, t13280: F, t13283: F, t13287: F, t13289: F, t13293: F, t13297: F, t13302: F, t13306: F, t13312: F, t13316: F, t13320: F, t13322: F, t1512: F, t2571: F, t2618: F, t2635: F, t2643: F, t2686: F, t4167: F, t4236: F, t4250: F, t9559: F, t9613: F, t9642: F) -> (F, F, F) {
    let t13326 = t2645 * t4181 * t10007;
    let t13330 = F::cast_from(7.0_f64) / F::cast_from(576.0_f64) * t9638 * t4191;
    let t13331 = -t9613 * t1512 / F::cast_from(3072.0_f64) - t2618 * t4236 / F::cast_from(1536.0_f64) + t13275 + t13277 + t13280 - t4167 * t2686 / F::cast_from(3072.0_f64) + t13283 * t2635 / F::cast_from(1536.0_f64) - t13287 - t9559 * t13289 / F::cast_from(4.0_f64) + t2571 * t13293 / F::cast_from(8.0_f64) + t2571 * t13297 / F::cast_from(16.0_f64) + t2643 * t13302 / F::cast_from(384.0_f64) + t2643 * t13306 / F::cast_from(768.0_f64) + t9642 * t4250 / F::cast_from(384.0_f64) - t2643 * t13312 / F::cast_from(1536.0_f64) - t2643 * t13316 / F::cast_from(3072.0_f64) + t13320 + t2643 * t13322 / F::cast_from(384.0_f64) + t2643 * t13326 / F::cast_from(768.0_f64) - t13330;
    (t13326, t13330, t13331)
}
