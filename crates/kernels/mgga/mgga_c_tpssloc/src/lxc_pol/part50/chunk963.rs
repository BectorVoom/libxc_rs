//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 963/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk963<F: Float>(t225: F, t387: F, t4657: F, t345: F, t7569: F, t1921: F, t25749: F, t986: F, t7593: F, t990: F, t25705: F, t349: F) -> (F, F, F, F, F) {
    let t25766 = t4657 * t225 * t387;
    let t25767 = t345 * t25766;
    let t25778 = t7569 * t225;
    let t25784 = t1921 * t25749;
    let t25785 = t986 * t25784;
    let t25789 = t990 * t7593;
    let t25791 = t349 * t25705;
    (t25767, t25778, t25785, t25789, t25791)
}
