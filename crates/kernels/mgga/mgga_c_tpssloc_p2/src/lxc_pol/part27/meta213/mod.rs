//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta213 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1059;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1060;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta213<F: Float>(t1118: F, t4781: F, t1099: F, t1670: F, t3315: F, t1117: F, t3313: F, t3238: F, t3319: F, t4721: F, t4726: F, t4731: F, t4735: F, t1128: F, t1675: F, t1136: F, t1683: F, t3295: F, t3339: F, t3346: F, t4749: F, t4757: F, t4765: F, t4767: F, t4770: F, t4773: F, t4776: F, t4779: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t4782, t4784, t4785, t4786, t4788, t4794) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1059::<F>(t1118, t4781, t1099, t1670, t3315, t1117, t3313, t3238, t3319, t4721, t4726, t4731, t4735);
        let (t4797, t4802, t4819) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1060::<F>(t1128, t1675, t1136, t1683, t3238, t3295, t3339, t3346, t4721, t4726, t4731, t4735, t4749, t4757, t4765, t4767, t4770, t4773, t4776, t4779);
    (t4782, t4784, t4785, t4786, t4788, t4794, t4797, t4802, t4819)
}
