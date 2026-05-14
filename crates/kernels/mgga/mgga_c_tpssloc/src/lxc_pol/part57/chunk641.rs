//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 641/919 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk641<F: Float>(t1891: F, t22822: F, t133: F, t6601: F, t6590: F, t6604: F, t22813: F, t22816: F, t1895: F, t794: F, t1899: F, t2693: F, t281: F, t6598: F, t22690: F, t814: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t23093 = t22822 * t1891;
    let t23094 = t23093 * t133;
    let t23095 = t23094 * t6601;
    let t23096 = 0.52708876011794399171e-3 * t23095;
    let t23097 = t6590 * t6604;
    let t23102 = t22813 * t1891;
    let t23103 = t23102 * t22816;
    let t23104 = t794 * t1895;
    let t23105 = t23103 * t23104;
    let t23106 = 0.16821981705891829522e-4 * t23105;
    let t23107 = t1899 * t2693;
    let t23108 = 119.0 / 6912.0 * t23107;
    let t23109 = t6598 * t281;
    let t23110 = t22690 * t814;
    (t23094, t23095, t23096, t23097, t23103, t23105, t23106, t23107, t23108, t23109, t23110)
}
