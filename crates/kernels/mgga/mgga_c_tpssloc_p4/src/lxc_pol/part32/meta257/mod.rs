//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta257 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1159;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1160;
use chunk2::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1161;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta257<F: Float>(t1307: F, t1998: F, t236: F, t6926: F, t1995: F, t6597: F, t133: F, t1999: F, t6600: F, t1996: F, t6604: F, t1339: F, t1352: F, t1332: F, t2002: F, t559: F, t1338: F, t59: F, t240: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t6928, t6929, t6931, t6933, t6935, t6936) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1159::<F>(t1307, t1998, t236, t6926, t1995, t6597, t133, t1999, t6600, t1996, t6604);
        let (t6937, t6938, t6940, t6941, t6943) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1160::<F>(t1339, t1352, t6936, t1332, t2002, t559, t1338, t59);
        let t6944 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1161::<F>(t240, t6943);
    (t6928, t6929, t6931, t6933, t6935, t6936, t6937, t6938, t6940, t6941, t6943, t6944)
}
