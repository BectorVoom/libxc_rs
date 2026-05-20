//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta545 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1895;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1896;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta545<F: Float>(t27480: F, t27529: F, t27568: F, t27739: F, t1241: F, t2154: F, t5088: F, t3598: F, t1751: F, t7299: F, t7302: F, t24574: F, t8015: F, t1238: F, t14980: F, t1761: F, t2155: F, t24589: F, t24880: F, t27406: F, t27422: F, t27424: F, t27427: F, t27434: F, t27438: F, t27441: F, t27446: F, t3487: F, t498: F, t7283: F, t7288: F, t8061: F) -> (F, F, F, F, F, F, F) {
        let (t27741, t27742, t27747, t27751, t27752, t27755) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1895::<F>(t27480, t27529, t27568, t27739, t1241, t2154, t5088, t3598, t1751, t7299, t7302, t24574, t8015);
        let t27757 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1896::<F>(t1238, t14980, t1761, t2155, t24589, t24880, t27406, t27422, t27424, t27427, t27434, t27438, t27441, t27446, t27742, t27747, t27752, t27755, t3487, t498, t7283, t7288, t8061);
    (t27741, t27742, t27747, t27751, t27752, t27755, t27757)
}
