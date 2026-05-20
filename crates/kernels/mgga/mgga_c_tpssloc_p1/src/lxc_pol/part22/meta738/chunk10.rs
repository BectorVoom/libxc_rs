//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2432/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2432<F: Float>(t4359: F, t60357: F, t4400: F, t59959: F, t13727: F, t17517: F, t13520: F, t17521: F, t17524: F, t17528: F, t49274: F, t21238: F, t2932: F) -> (F, F, F, F, F, F, F) {
    let t69253 = F::new(6.0) * t60357 * t4359;
    let t69255 = F::cast_from(0.48245938496077605201e2_f64) * t59959 * t4400;
    let t69257 = F::new(6.0) * t13727 * t17517;
    let t69259 = F::cast_from(0.48245938496077605201e2_f64) * t13520 * t17521;
    let t69261 = F::cast_from(0.96491876992155210402e2_f64) * t13520 * t17524;
    let t69263 = F::cast_from(0.1551780387578202009e4_f64) * t49274 * t17528;
    let t69276 = t21238 * t2932;
    (t69253, t69255, t69257, t69259, t69261, t69263, t69276)
}
