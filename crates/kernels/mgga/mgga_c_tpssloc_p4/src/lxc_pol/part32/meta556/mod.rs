//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta556 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1917;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1918;
use chunk2::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1919;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta556<F: Float>(t22761: F, t6390: F, t2002: F, t6378: F, t559: F, t6422: F, t6945: F, t6427: F, t6952: F, t6431: F, t1831: F, t26257: F, t1799: F, t1824: F, t550: F, t1339: F, t22827: F, t22833: F, t6396: F, t22820: F, t22826: F, t22859: F, t22864: F, t22868: F, t26272: F, t26295: F, t28083: F, t539: F, t2015: F, t6460: F, t3887: F, t1842: F, t26337: F, t22635: F, t22633: F, t1825: F, t26421: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t28085, t28088, t28089, t28091, t28093, t28095, t28097) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1917::<F>(t22761, t6390, t2002, t6378, t559, t6422, t6945, t6427, t6952, t6431, t1831, t26257);
        let (t28100, t28101, t28106) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1918::<F>(t1799, t1824, t550, t1339, t22827, t22833, t6396, t22820, t22826, t22859, t22864, t22868, t26272, t26295, t28085, t28089, t28091, t28093, t28095, t28097);
        let (t28107, t28108, t28111, t28116, t28117, t28118, t28130) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1919::<F>(t28083, t28106, t539, t2015, t6460, t3887, t1842, t26337, t22635, t22633, t1825, t26421);
    (t28088, t28100, t28101, t28107, t28108, t28111, t28116, t28117, t28118, t28130)
}
