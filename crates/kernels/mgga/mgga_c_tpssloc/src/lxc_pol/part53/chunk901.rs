//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 901/939 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk901<F: Float>(t33947: F, t814: F, t1509: F, t8728: F, t114659: F, t114666: F, t114670: F, t116673: F, t116681: F, t121493: F, t121498: F, t121501: F, t121504: F, t121509: F, t121517: F, t121521: F, t121524: F, t121528: F, t121533: F, t121536: F, t1510: F, t31994: F, t4166: F, t4291: F, t812: F, t829: F) -> (F, F) {
    let t123622 = t814 * t33947;
    let t123626 = t8728 * t1509;
    let t123641 = 0.6579736267392905746e-1 * t121493 + 0.6579736267392905746e-1 * t121498 + 0.3289868133696452873e-1 * t121501 - 0.16449340668482264365e-1 * t121504 - t812 * t123622 * t829 - 0.6579736267392905746e-1 * t121509 - t4291 * t123626 * t829 + 0.15352717957250113407e0 * t114659 + 0.3289868133696452873e-1 * t114666 - 0.6579736267392905746e-1 * t121517 - 0.6579736267392905746e-1 * t121521 + 0.16449340668482264365e-1 * t121524 - 0.3289868133696452873e-1 * t121528 - t812 * t116681 * t1510 + 0.15352717957250113407e0 * t121533 - 0.76763589786250567037e-1 * t114670 + t116673 + 0.76763589786250567037e-1 * t121536 - t4166 * t31994;
    (t123626, t123641)
}
