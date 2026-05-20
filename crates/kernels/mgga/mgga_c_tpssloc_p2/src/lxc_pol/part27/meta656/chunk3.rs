//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2293/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2293<F: Float>(t22674: F, t26202: F, t6897: F, t22716: F, t7701: F, t1834: F, t212: F, t22642: F, t6890: F, t1373: F, t254: F, t81267: F) -> (F, F, F, F, F) {
    let t90645 = t6897 * t22674 * t26202;
    let t90646 = F::cast_from(0.82246703342411321824e-2_f64) * t90645;
    let t90659 = t22716 * t7701;
    let t90663 = t22642 * t212 * t1834 * t6890;
    let t90665 = t1373 * t254;
    let t90670 = F::cast_from(0.16449340668482264365e-1_f64) * t81267;
    (t90646, t90659, t90663, t90665, t90670)
}
