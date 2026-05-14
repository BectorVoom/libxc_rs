//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 893/939 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk893<F: Float>(t225: F, t33948: F, t114592: F, t114606: F, t121296: F, t121299: F, t121302: F, t121305: F, t121308: F, t121311: F, t121314: F, t121318: F, t121326: F, t13042: F, t2054: F, t866: F, t8741: F, t92847: F, t92939: F) -> (F,) {
    let t123443 = t33948 * t225;
    let t123452 = -0.3289868133696452873e-1 * t114592 + 0.76763589786250567037e-1 * t121296 + 0.6579736267392905746e-1 * t121299 - 0.3289868133696452873e-1 * t121302 + 0.16449340668482264365e-1 * t121305 - 0.3289868133696452873e-1 * t121308 - 0.6579736267392905746e-1 * t121311 - 0.6579736267392905746e-1 * t121314 - 0.3289868133696452873e-1 * t121318 - t123443 * t866 - 2.0 * t92847 * t2054 - 0.13159472534785811492e0 * t121326 - 0.15352717957250113407e0 * t114606 - t13042 * t8741 - 2.0 * t92939 * t2054;
    (t123452,)
}
