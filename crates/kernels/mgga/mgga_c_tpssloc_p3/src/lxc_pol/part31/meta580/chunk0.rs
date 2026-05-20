//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1818/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1818<F: Float>(t22716: F, t7701: F, t1834: F, t212: F, t22642: F, t6890: F, t81267: F, t26215: F, t81228: F, t81326: F, t6897: F, t6907: F, t90544: F) -> (F, F, F, F, F) {
    let t90659 = t22716 * t7701;
    let t90663 = t22642 * t212 * t1834 * t6890;
    let t90670 = F::cast_from(0.16449340668482264365e-1_f64) * t81267;
    let t90686 = t81228 * t81326 * t26215;
    let t90701 = t6897 * t90544 * t6907;
    (t90659, t90663, t90670, t90686, t90701)
}
