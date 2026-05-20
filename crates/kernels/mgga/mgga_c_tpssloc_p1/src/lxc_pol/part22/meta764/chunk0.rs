//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2580/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2580<F: Float>(t71700: F, t71704: F, t71707: F, t71711: F, t71784: F, t71786: F, t71788: F, t71790: F, t71793: F, t71795: F, t71797: F, t71800: F, t71803: F, t71806: F, t71809: F, t71811: F, t71814: F, t71817: F, t71819: F, t71821: F, t71850: F, t71853: F) -> (F, F) {
    let t72077 = -t71700 + t71704 + t71707 + t71711 - t71784 + t71786 - t71788 + t71790 - t71793 + t71795 + t71797;
    let t72078 = t71800 - t71803 - t71806 - t71809 + t71811 + t71814 + t71817 + t71819 - t71821 - t71850 + t71853;
    (t72077, t72078)
}
