//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2482/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2482<F: Float>(t1023: F, t14218: F, t14508: F, t17673: F, t17701: F, t17734: F, t21138: F, t21597: F, t3070: F, t3071: F, t3114: F, t42388: F, t42752: F, t4650: F, t48570: F, t48611: F, t49853: F, t49872: F, t49934: F, t5681: F, t62306: F, t69935: F) -> F {
    let t70623 = -t49853 + t14508 * t17734 / F::new(256.0) + F::new(3.0) / F::new(512.0) * t48570 * t17673 - t49934 * t17701 / F::new(1536.0) + t42752 / F::new(15552.0) - t49872 - t62306 / F::new(2304.0) + t3070 * t3071 * t21138 * t1023 / F::new(768.0) + t3114 * t21597 / F::new(3072.0) - t3070 * t3071 * t5681 * t4650 / F::new(768.0) + F::new(3.0) / F::new(512.0) * t42388 * t48611 * t69935 * t14218;
    t70623
}
