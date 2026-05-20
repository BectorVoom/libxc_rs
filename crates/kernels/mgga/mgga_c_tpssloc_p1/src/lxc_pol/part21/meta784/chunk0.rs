//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2718/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2718<F: Float>(t54460: F, t54462: F, t39851: F, t39857: F, t54467: F, t54469: F, t54471: F, t40221: F, t40225: F, t19573: F, t588: F, t592: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t57218 = F::new(160.0) * t54460;
    let t57219 = F::new(240.0) * t54462;
    let t57220 = F::new(24.0) * t39851;
    let t57221 = F::new(64.0) * t39857;
    let t57222 = F::cast_from(0.20508037716432813315e4_f64) * t54467;
    let t57223 = F::cast_from(0.23392894490538584828e1_f64) * t54469;
    let t57224 = F::cast_from(0.69263436422725855034e2_f64) * t54471;
    let t57225 = F::new(8.0) * t40221;
    let t57226 = F::new(24.0) * t40225;
    let t57227 = t588 * t19573;
    let t57228 = F::new(8.0) * t57227;
    let t57229 = t592 * t19573;
    (t57218, t57219, t57220, t57221, t57222, t57223, t57224, t57225, t57226, t57228, t57229)
}
