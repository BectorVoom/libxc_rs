//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1697/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1697<F: Float>(t1799: F, t26395: F, t6637: F, t6888: F, t1998: F, t6434: F, t214: F, t1985: F, t19739: F, t550: F, t6976: F, t1992: F) -> (F, F, F, F, F, F, F, F, F) {
    let t28148 = t26395 * t1799;
    let t28149 = t6637 * t28148;
    let t28150 = t6888 * t28149;
    let t28159 = t1998 * t6434;
    let t28160 = t214 * t28159;
    let t28161 = t1985 * t28160;
    let t28163 = t19739 * t550;
    let t28164 = t6976 * t28163;
    let t28165 = t1992 * t28164;
    (t28148, t28149, t28150, t28159, t28160, t28161, t28163, t28164, t28165)
}
