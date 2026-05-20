//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 1475/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1475<F: Float>(t13196: F, t2701: F, t820: F, t2563: F, t4159: F, t119: F, t12971: F, t210: F, t4155: F, t9573: F, t2645: F, t2684: F, t4248: F) -> (F, F, F, F, F) {
    let t13198 = t2701 * t820 * t13196;
    let t13202 = F::new(7.0) / F::new(72.0) * t2563 * t4159;
    let t13203 = t119 * t12971;
    let t13204 = t210 * t13203;
    let t13208 = F::new(7.0) / F::new(24.0) * t9573 * t4155;
    let t13210 = t2645 * t4248 * t2684;
    (t13198, t13202, t13204, t13208, t13210)
}
