//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1572/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1572<F: Float>(t1891: F, t22813: F, t22816: F, t1895: F, t794: F, t1899: F, t2693: F, t281: F, t6598: F) -> (F, F, F, F, F) {
    let t23102 = t22813 * t1891;
    let t23103 = t23102 * t22816;
    let t23104 = t794 * t1895;
    let t23105 = t23103 * t23104;
    let t23107 = t1899 * t2693;
    let t23109 = t6598 * t281;
    (t23102, t23104, t23105, t23107, t23109)
}
