//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2198/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2198<F: Float>(t26161: F, t26163: F, t97920: F, t24991: F, t7685: F, t22574: F, t25988: F, t33136: F, t28823: F, t6876: F, t1874: F, t96709: F) -> (F, F, F, F, F) {
    let t97923 = F::new(4.0) * t26161 * t97920 * t26163;
    let t97925 = F::new(6.0) * t7685 * t24991;
    let t97928 = F::new(6.0) * t22574 * t33136 * t25988;
    let t97930 = F::new(2.0) * t6876 * t28823;
    let t97932 = F::new(2.0) * t96709 * t1874;
    (t97923, t97925, t97928, t97930, t97932)
}
