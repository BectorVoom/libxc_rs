//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2435/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2435<F: Float>(t2792: F, t4396: F, t5726: F, t1557: F, t17422: F, t10655: F, t21318: F, t1556: F, t2842: F, t60745: F, t17520: F, t4395: F) -> (F, F, F, F, F) {
    let t69302 = F::new(6.0) * t2792 * t4396 * t5726;
    let t69305 = F::new(6.0) * t2792 * t1557 * t17422;
    let t69307 = F::cast_from(0.48245938496077605201e2_f64) * t10655 * t21318;
    let t69310 = F::cast_from(0.48245938496077605201e2_f64) * t2842 * t60745 * t1556;
    let t69313 = F::cast_from(0.48245938496077605201e2_f64) * t2842 * t17520 * t4395;
    (t69302, t69305, t69307, t69310, t69313)
}
