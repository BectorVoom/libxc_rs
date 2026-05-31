//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2488/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2488<F: Float>(t13969: F, t21486: F, t3130: F, t1041: F, t13995: F, t17705: F, t17976: F, t18036: F, t21512: F, t3117: F, t43219: F, t4582: F, t4588: F, t4644: F, t49929: F, t50175: F, t50181: F, t62631: F, t62640: F, t70316: F) -> F {
    let t70805 = t3130 * t13969 * t21486;
    let t70823 = t70805 / F::cast_from(768.0_f64) + F::cast_from(5.0_f64) / F::cast_from(4608.0_f64) * t3117 * t21512 + F::cast_from(5.0_f64) / F::cast_from(4608.0_f64) * t1041 * t4582 * t4588 * t70316 - t4644 * t17976 / F::cast_from(384.0_f64) - t50175 + t50181 / F::cast_from(3456.0_f64) + t13995 * t17705 / F::cast_from(768.0_f64) + t49929 * t18036 / F::cast_from(768.0_f64) - t62631 / F::cast_from(72.0_f64) + t62640 / F::cast_from(48.0_f64) + t43219 / F::cast_from(10368.0_f64);
    t70823
}
