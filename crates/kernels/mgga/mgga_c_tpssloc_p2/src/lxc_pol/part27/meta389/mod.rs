//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta389 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1596;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1597;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1598;
use chunk3::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1599;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta389<F: Float>(t3247: F, t3966: F, t607: F, t1088: F, t123: F, t2250: F, t4728: F, t11137: F, t11139: F, t11141: F, t11143: F, t11247: F, t14702: F, t14708: F, t14721: F, t14723: F, t14724: F, t14728: F, t14733: F, t14738: F, t14742: F, t14746: F, t1100: F, t1667: F, t2403: F, t14720: F, t11215: F, t11217: F, t14722: F) -> (F, F, F, F, F, F, F, F) {
        let (t14749, t14751) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1596::<F>(t3247, t3966, t607, t1088, t123);
        let (t14753, t14755) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1597::<F>(t2250, t4728, t1088, t123);
        let t14758 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1598::<F>(t11137, t11139, t11141, t11143, t11247, t14702, t14708, t14721, t14723, t14724, t14728, t14733, t14738, t14742, t14746, t14751, t14755);
        let (t14759, t14766, t14776) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1599::<F>(t1100, t14758, t1667, t2403, t14720, t11215, t11217, t14722, t14733, t14738, t14742, t14746, t14751, t14755);
    (t14749, t14751, t14753, t14755, t14758, t14759, t14766, t14776)
}
