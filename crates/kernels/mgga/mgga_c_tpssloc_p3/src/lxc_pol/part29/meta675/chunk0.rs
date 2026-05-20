//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2264/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2264<F: Float>(t2018: F, t40611: F, t1845: F, t3698: F, t26161: F, t15868: F, t1983: F, t6996: F, t3734: F, t24995: F, t8643: F, t23831: F, t7458: F) -> (F, F, F, F) {
    let t91686 = t2018 * t40611;
    let t91687 = t1845 * t3698;
    let t91690 = F::new(6.0) * t26161 * t91686 * t91687;
    let t91694 = F::new(2.0) * t1983 * t6996 * t15868;
    let t91695 = t1845 * t3734;
    let t91698 = F::new(6.0) * t24995 * t8643 * t91695;
    let t91704 = F::new(2.0) * t7458 * t23831;
    (t91690, t91694, t91698, t91704)
}
