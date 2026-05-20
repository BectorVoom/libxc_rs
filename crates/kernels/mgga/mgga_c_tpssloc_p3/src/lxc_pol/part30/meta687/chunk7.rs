//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2184/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2184<F: Float>(t1307: F, t1842: F, t22635: F, t26331: F, t26337: F, t26189: F, t26193: F, t6888: F, t22892: F, t7691: F, t90544: F, t1835: F, t254: F) -> (F, F, F, F) {
    let t97721 = t1842 * t1307;
    let t97724 = t26331 * t22635 * t26337 * t97721;
    let t97729 = t6888 * t26193 * t26189;
    let t97732 = t22892 * t90544 * t7691;
    let t97740 = t1835 * t254;
    (t97724, t97729, t97732, t97740)
}
