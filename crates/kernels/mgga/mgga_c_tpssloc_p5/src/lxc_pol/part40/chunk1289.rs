//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 40 (v4rho3tau_4) CSE chunk 1289/1303 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part40_v4rho3tau_4_chunk1289<F: Float>(t110075: F, t30149: F, t29895: F, t30156: F, t30165: F, t2331: F, t2585: F, t29900: F, t30172: F, t111: F, t8240: F, t112: F, t30217: F) -> (F, F, F, F, F, F, F) {
    let t110564 = F::new(4.0) * t110075 * t30149;
    let t110566 = F::new(20.0) / F::new(9.0) * t29895 * t30156;
    let t110586 = F::new(20.0) / F::new(9.0) * t29895 * t30165;
    let t110601 = t2585 * t2331;
    let t110615 = F::new(20.0) / F::new(27.0) * t29900 * t30172;
    let t110631 = t8240 * t111;
    let t110684 = t30217 * t112;
    (t110564, t110566, t110586, t110601, t110615, t110631, t110684)
}
