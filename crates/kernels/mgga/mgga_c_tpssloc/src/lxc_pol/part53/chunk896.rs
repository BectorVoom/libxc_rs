//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 896/939 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk896<F: Float>(t114792: F, t114795: F, t116536: F, t121413: F, t121419: F, t121426: F, t121429: F, t121431: F, t121435: F, t121437: F, t121444: F, t121448: F, t121457: F, t121464: F, t24297: F, t32002: F, t4268: F, t7830: F) -> (F,) {
    let t123521 = 0.6579736267392905746e-1 * t121413 - 0.13159472534785811492e0 * t121419 + 4.0 * t24297 * t7830 + 0.6579736267392905746e-1 * t121426 + 0.6579736267392905746e-1 * t121429 + 0.76763589786250567037e-1 * t121431 + 0.6579736267392905746e-1 * t121435 - 0.15352717957250113407e0 * t121437 + 0.16449340668482264365e-1 * t114792 + 0.16449340668482264365e-1 * t114795 - 0.3289868133696452873e-1 * t121444 + 0.6579736267392905746e-1 * t121448 + 4.0 * t4268 * t32002 - t116536 - 0.3289868133696452873e-1 * t121457 + 0.3289868133696452873e-1 * t121464;
    (t123521,)
}
