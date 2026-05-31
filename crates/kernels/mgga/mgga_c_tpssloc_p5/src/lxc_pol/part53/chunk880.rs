//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 880/1059 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk880<F: Float>(t31998: F, t858: F, t2053: F, t2718: F, t7106: F, t865: F, t8740: F, t2054: F, t24297: F, t24305: F, t259: F, t2597: F, t2713: F, t31317: F, t31964: F, t31971: F, t31974: F, t31985: F, t855: F, t866: F, t8734: F, t8741: F) -> (F, F, F, F) {
    let t31999 = t858 * t31998;
    let t32002 = t2718 * t2053 * t7106;
    let t32005 = t8740 * t865;
    let t32006 = t2718 * t32005;
    let t32009 = -t2597 * t8741 - F::cast_from(2.0_f64) * t24305 * t2054 - t31964 * t866 + F::cast_from(2.0_f64) * t2597 * t8734 - F::cast_from(2.0_f64) * t24297 * t2054 + F::cast_from(0.3289868133696452873e-1_f64) * t31317 - t31971 + F::cast_from(2.0_f64) * t2713 * t8734 + t31974 * t259 + t31985 * t259 - t855 * t31999 + F::cast_from(4.0_f64) * t855 * t32002 + F::cast_from(2.0_f64) * t855 * t32006;
    (t31999, t32002, t32006, t32009)
}
