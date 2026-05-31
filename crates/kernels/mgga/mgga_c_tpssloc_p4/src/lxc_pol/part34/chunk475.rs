//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 475/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk475<F: Float>(t1314: F, t792: F, t116: F, t534: F, t212: F, t2586: F, t2600: F, t541: F, t1337: F, t551: F) -> (F, F, F, F, F, F) {
    let t3739 = t792 * t1314;
    let t3748 = t534 * t116;
    let t3749 = t3748 * t212;
    let t3751 = F::cast_from(0.83333333333333333332e-3_f64) * t2586 * t3749;
    let t3762 = F::cast_from(35.0_f64) / F::cast_from(432.0_f64) * t2600 * t541;
    let t3787 = F::cast_from(1.0_f64) / t1337 / t551;
    (t3739, t3748, t3749, t3751, t3762, t3787)
}
