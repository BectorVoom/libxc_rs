//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta250 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1049;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1050;
use chunk2::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1051;
use chunk3::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1052;
use chunk4::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1053;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta250<F: Float>(t334: F, t371: F, t28: F, t776: F, t868: F, t1271: F, t191: F, t192: F, t1307: F, t1390: F, t1984: F, t6546: F, t1988: F, t131: F, t209: F, t547: F, t1878: F, t214: F, t562: F, t225: F, t567: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t6793, t6841, t6848, t6875, t6876) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1049::<F>(t334, t371, t28, t776, t868, t1271, t191, t192);
        let (t6879, t6883) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1050::<F>(t1307, t1390, t1984, t6546);
        let (t6884, t6887, t6888) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1051::<F>(t1988, t6883, t131, t209, t547, t1878);
        let t6889 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1052::<F>(t214, t562);
        let t6890 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1053::<F>(t225, t567);
    (t6793, t6841, t6848, t6875, t6876, t6879, t6883, t6884, t6887, t6888, t6889, t6890)
}
