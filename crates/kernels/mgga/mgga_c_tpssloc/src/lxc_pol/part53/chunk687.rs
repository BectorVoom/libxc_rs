//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 687/939 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk687<F: Float>(t22816: F, t23102: F, t1895: F, t794: F, t1899: F, t2693: F, t281: F, t6598: F, t22690: F, t814: F) -> (F, F, F, F, F) {
    let t23103 = t23102 * t22816;
    let t23104 = t794 * t1895;
    let t23105 = t23103 * t23104;
    let t23107 = t1899 * t2693;
    let t23109 = t6598 * t281;
    let t23110 = t22690 * t814;
    (t23103, t23105, t23107, t23109, t23110)
}
