//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2187/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2187<F: Float>(t24987: F, t7756: F, t2314: F, t28025: F, t4034: F, t1266: F, t28017: F, t652: F, t1845: F, t5187: F, t22574: F, t8643: F) -> (F, F, F, F, F) {
    let t97779 = F::new(2.0) * t24987 * t7756;
    let t97783 = F::new(2.0) * t2314 * t28025;
    let t97785 = F::new(2.0) * t4034 * t28025;
    let t97788 = F::new(2.0) * t652 * t1266 * t28017;
    let t97789 = t5187 * t1845;
    let t97792 = F::new(6.0) * t22574 * t8643 * t97789;
    (t97779, t97783, t97785, t97788, t97792)
}
