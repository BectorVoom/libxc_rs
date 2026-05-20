//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2178/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2178<F: Float>(t13336: F, t1909: F, t25269: F, t2617: F, t4182: F, t4281: F, t7533: F, t81980: F, t81989: F, t82005: F, t82013: F, t82016: F, t87620: F, t87660: F, t87666: F, t87669: F, t87672: F, t87676: F, t87680: F, t87687: F, t87692: F, t9612: F) -> F {
    let t87694 = F::cast_from(0.16449340668482264365e-1_f64) * t87660 - F::cast_from(0.11514538467937585055e0_f64) * t81980 + F::cast_from(0.38381794893125283518e-1_f64) * t81989 + F::cast_from(0.38381794893125283518e-1_f64) * t82005 + t13336 * t1909 - F::cast_from(0.63969658155208805863e-1_f64) * t87666 + t87669 - F::cast_from(0.3289868133696452873e-1_f64) * t87672 - F::cast_from(0.16449340668482264365e-1_f64) * t87676 + t87680 + F::new(4.0) * t4281 * t87620 * t4182 - t9612 * t7533 - F::new(2.0) * t2617 * t25269 - t87687 - F::cast_from(0.38381794893125283518e-1_f64) * t82013 - F::cast_from(0.82246703342411321824e-2_f64) * t82016 - F::cast_from(0.82246703342411321825e-2_f64) * t87692;
    t87694
}
