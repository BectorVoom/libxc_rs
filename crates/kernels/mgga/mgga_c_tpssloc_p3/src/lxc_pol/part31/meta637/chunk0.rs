//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1904/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1904<F: Float>(t28192: F, t80727: F, t1307: F, t1377: F, t22633: F, t22635: F, t6460: F, t1842: F, t26331: F, t26337: F, t26189: F, t26193: F, t6888: F) -> (F, F, F, F) {
    let t97664 = t80727 * t28192;
    let t97705 = t22633 * t22635 * t1377 * t6460 * t1307;
    let t97721 = t1842 * t1307;
    let t97724 = t26331 * t22635 * t26337 * t97721;
    let t97729 = t6888 * t26193 * t26189;
    (t97664, t97705, t97724, t97729)
}
