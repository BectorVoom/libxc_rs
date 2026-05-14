//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 498/1308 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk498<F: Float>(t3739: F, t3741: F, t116: F, t534: F, t212: F, t2586: F, t1324: F, t225: F) -> (F, F, F, F) {
    let t3742 = t3739 * t3741;
    let t3748 = t534 * t116;
    let t3749 = t3748 * t212;
    let t3751 = 0.83333333333333333332e-3 * t2586 * t3749;
    let t3758 = t1324 * t225;
    (t3742, t3748, t3751, t3758)
}
