//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 1039/1059 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk1039<F: Float>(t115586: F, t115596: F, t115601: F, t117275: F, t12021: F, t122328: F, t122331: F, t122370: F, t122377: F, t1375: F, t16022: F, t1807: F, t1843: F, t24095: F, t26224: F, t26989: F, t27009: F, t27061: F, t27131: F, t32147: F, t5210: F, t5353: F, t568: F, t7199: F, t7937: F, t8788: F, t8793: F, t8801: F) -> F {
    let t124205 = -F::new(2.0) * t24095 * t7937 - t16022 * t8801 - t117275 * t1843 - F::new(0.6579736267392905746e-1) * t122328 + F::new(0.3289868133696452873e-1) * t122331 - F::new(12.0) * t26224 * t26989 * t27131 + t5210 * t8788 * t568 + t1807 * t32147 * t568 - F::new(0.3289868133696452873e-1) * t115586 - F::new(0.15352717957250113407e0) * t115596 + F::new(4.0) * t27009 * t7199 + F::new(0.16449340668482264365e-1) * t115601 + F::new(0.6579736267392905746e-1) * t122370 - F::new(0.6579736267392905746e-1) * t122377 - F::new(12.0) * t26224 * t26989 * t27061 - F::new(6.0) * t1375 * t12021 * t8793 * t5353;
    t124205
}
