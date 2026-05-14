//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 392/919 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk392<F: Float>(t1020: F, t4630: F, t1009: F, t1603: F, t1011: F, t1019: F, t1040: F, t1611: F, t1626: F, t225: F, t1057: F, t193: F, t336: F, t1654: F, t690: F) -> (F, F, F, F, F, F, F, F) {
    let t4631 = t1020 * t4630;
    let t4639 = t1603 * t1009;
    let t4640 = t4639 * t1011;
    let t4641 = t4640 * t1019;
    let t4644 = t1611 * t1040;
    let t4660 = t1626 * t225;
    let t4669 = t4639 * t1057;
    let t4700 = t193 * t336;
    let t4721 = t690 * t1654;
    (t4631, t4640, t4641, t4644, t4660, t4669, t4700, t4721)
}
