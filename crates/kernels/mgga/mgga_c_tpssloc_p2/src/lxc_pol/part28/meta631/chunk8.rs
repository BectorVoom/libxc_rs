//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1985/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1985<F: Float>(t92586: F, t92605: F, t92623: F, t92642: F, t92663: F, t92682: F, t92701: F, t92719: F, t87565: F, t226: F, t235: F, t24269: F, t26661: F, t2684: F, t4234: F, t812: F, t81623: F, t81630: F, t81633: F, t81642: F, t81653: F, t87531: F, t87538: F, t87541: F, t87554: F, t92560: F, t92561: F, t92564: F, t92565: F) -> (F, F) {
    let t92722 = t92586 + t92605 + t92623 + t92642 + t92663 + t92682 + t92701 + t92719;
    let t92729 = F::cast_from(0.15352717957250113407e0_f64) * t87565;
    let t92732 = -F::cast_from(0.13159472534785811492e0_f64) * t87531 + t92560 + t92561 - F::cast_from(0.16449340668482264365e-1_f64) * t87538 + F::cast_from(0.6579736267392905746e-1_f64) * t87541 - t92564 - t92565 - F::cast_from(0.3289868133696452873e-1_f64) * t87554 - F::new(2.0) * t812 * t24269 * t4234 + F::cast_from(0.15352717957250113407e0_f64) * t81623 + t226 * t235 * t92722 + F::cast_from(0.16449340668482264365e-1_f64) * t81630 - F::cast_from(0.51175726524167044691e0_f64) * t81633 - F::cast_from(0.49348022005446793095e-1_f64) * t81642 - F::cast_from(0.3289868133696452873e-1_f64) * t81653 - t92729 - t812 * t26661 * t2684;
    (t92722, t92732)
}
