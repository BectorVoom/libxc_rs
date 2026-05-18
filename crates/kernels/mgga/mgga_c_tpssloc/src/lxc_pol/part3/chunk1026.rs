//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 3 (v3rho3_1) CSE chunk 1026/1255 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part3_v3rho3_1_chunk1026<F: Float>(t2679: F, t4180: F, t4181: F, t4240: F, t9638: F, t13242: F, t2645: F, t2647: F, t10007: F, t4191: F, t13275: F, t13277: F, t13280: F, t13283: F, t13287: F, t13289: F, t13293: F, t13297: F, t13302: F, t13306: F, t13312: F, t1512: F, t2571: F, t2618: F, t2635: F, t2643: F, t2686: F, t4167: F, t4236: F, t4250: F, t9559: F, t9613: F, t9642: F) -> F {
    let t13316 = t4180 * t4181 * t2679;
    let t13320 = F::new(7.0) / F::new(2304.0) * t9638 * t4240;
    let t13322 = t2645 * t13242 * t2647;
    let t13326 = t2645 * t4181 * t10007;
    let t13330 = F::new(7.0) / F::new(576.0) * t9638 * t4191;
    let t13331 = -t9613 * t1512 / F::new(3072.0) - t2618 * t4236 / F::new(1536.0) + t13275 + t13277 + t13280 - t4167 * t2686 / F::new(3072.0) + t13283 * t2635 / F::new(1536.0) - t13287 - t9559 * t13289 / F::new(4.0) + t2571 * t13293 / F::new(8.0) + t2571 * t13297 / F::new(16.0) + t2643 * t13302 / F::new(384.0) + t2643 * t13306 / F::new(768.0) + t9642 * t4250 / F::new(384.0) - t2643 * t13312 / F::new(1536.0) - t2643 * t13316 / F::new(3072.0) + t13320 + t2643 * t13322 / F::new(384.0) + t2643 * t13326 / F::new(768.0) - t13330;
    t13331
}
