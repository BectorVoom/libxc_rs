//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 1038/1059 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk1038<F: Float>(t124152: F, t124165: F, t115572: F, t117287: F, t117300: F, t122251: F, t122260: F, t122270: F, t122278: F, t122281: F, t122295: F, t122304: F, t124124: F, t1375: F, t1385: F, t1386: F, t24082: F, t27062: F, t33810: F, t33843: F, t3882: F, t3887: F, t539: F, t568: F, t7194: F, t7937: F) -> (F, F) {
    let t124166 = t124152 + t124165;
    let t124176 = F::new(0.15352717957250113407e0) * t122251 + t117287 - t124124 * t1386 - F::new(2.0) * t24082 * t7937 - F::new(0.3289868133696452873e-1) * t122260 - F::new(6.0) * t3882 * t33810 + F::new(2.0) * t1375 * t3887 * t33843 * t1385 + F::new(0.6579736267392905746e-1) * t122270 + t539 * t124166 * t568 + F::new(0.6579736267392905746e-1) * t122278 - F::new(0.3289868133696452873e-1) * t122281 + t117300 + F::new(0.76763589786250567037e-1) * t122295 + F::new(0.16449340668482264365e-1) * t115572 + F::new(0.19739208802178717238e0) * t122304 + F::new(4.0) * t7194 * t27062;
    (t124166, t124176)
}
