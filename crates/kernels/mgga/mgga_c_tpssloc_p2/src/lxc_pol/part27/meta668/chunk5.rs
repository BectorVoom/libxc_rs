//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2360/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2360<F: Float>(t16153: F, t24995: F, t8945: F, t22574: F, t25988: F, t31035: F, t2018: F, t40611: F, t1845: F, t3698: F, t26161: F, t15868: F, t1983: F, t6996: F) -> (F, F, F, F) {
    let t91681 = F::new(6.0) * t24995 * t8945 * t16153;
    let t91684 = F::new(6.0) * t22574 * t31035 * t25988;
    let t91686 = t2018 * t40611;
    let t91687 = t1845 * t3698;
    let t91690 = F::new(6.0) * t26161 * t91686 * t91687;
    let t91694 = F::new(2.0) * t1983 * t6996 * t15868;
    (t91681, t91684, t91690, t91694)
}
