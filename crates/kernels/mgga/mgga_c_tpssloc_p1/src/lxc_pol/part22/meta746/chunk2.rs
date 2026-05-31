//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2483/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2483<F: Float>(t1041: F, t13969: F, t21550: F, t1023: F, t10937: F, t14218: F, t17697: F, t21570: F, t2986: F, t42358: F, t43361: F, t4582: F, t4644: F, t48611: F, t49907: F, t49923: F, t50366: F, t62343: F, t62349: F, t62360: F, t62840: F, t68513: F, t70273: F) -> F {
    let t70640 = t1041 * t13969 * t21550;
    let t70645 = -F::cast_from(3.0_f64) / F::cast_from(512.0_f64) * t43361 * t48611 * t62840 * t14218 + t49907 + t62343 / F::cast_from(1536.0_f64) - t62349 / F::cast_from(768.0_f64) - t49923 - t62360 / F::cast_from(4608.0_f64) + F::cast_from(5.0_f64) / F::cast_from(1728.0_f64) * t4644 * t17697 - F::cast_from(5.0_f64) / F::cast_from(864.0_f64) * t10937 * t21570 - t42358 * t4582 * t70273 * t1023 / F::cast_from(3072.0_f64) - t70640 / F::cast_from(1152.0_f64) + t2986 * t50366 * t68513 / F::cast_from(16.0_f64);
    t70645
}
