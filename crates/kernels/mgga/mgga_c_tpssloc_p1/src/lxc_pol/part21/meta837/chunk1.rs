//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2979/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2979<F: Float>(t10508: F, t248: F, t3039: F, t5878: F, t1041: F, t10863: F, t13980: F, t14085: F, t14107: F, t14180: F, t17693: F, t17712: F, t3117: F, t3130: F, t4582: F, t4585: F, t4644: F, t49734: F, t49748: F, t49854: F, t50193: F, t5861: F, t61855: F, t62148: F, t62150: F, t62152: F, t62164: F, t62177: F) -> F {
    let t62183 = t3039 * t248 * t10508 * t5878;
    let t62185 = -t62148 / F::new(6912.0) - t62150 / F::new(648.0) + t62152 / F::new(1152.0) + t49734 / F::new(2304.0) + t3130 * t4582 * t17712 * t13980 / F::new(1536.0) + F::new(5.0) / F::new(3456.0) * t4644 * t14180 + t50193 * t14107 / F::new(1536.0) - t62164 / F::new(2304.0) - F::new(5.0) / F::new(432.0) * t1041 * t4582 * t49854 * t61855 + F::new(5.0) / F::new(3456.0) * t3117 * t17693 - t14085 * t4585 / F::new(576.0) + F::new(5.0) / F::new(3888.0) * t49748 - t62177 / F::new(13824.0) - F::new(5.0) / F::new(1296.0) * t10863 * t5861 + t62183 / F::new(13824.0);
    t62185
}
