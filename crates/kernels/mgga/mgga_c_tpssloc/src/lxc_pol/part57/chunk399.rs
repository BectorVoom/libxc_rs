//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 399/1049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk399<F: Float>(t1314: F, t792: F, t116: F, t534: F, t212: F, t2586: F, t2600: F, t541: F, t1337: F, t551: F) -> (F, F, F, F, F) {
    let t3739 = t792 * t1314;
    let t3748 = t534 * t116;
    let t3749 = t3748 * t212;
    let t3751 = F::new(0.83333333333333333332e-3) * t2586 * t3749;
    let t3762 = F::new(35.0) / F::new(432.0) * t2600 * t541;
    let t3787 = F::new(1.0) / t1337 / t551;
    (t3739, t3748, t3751, t3762, t3787)
}
