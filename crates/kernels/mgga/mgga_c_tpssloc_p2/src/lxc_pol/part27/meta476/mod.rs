//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta476 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1846;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1847;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta476<F: Float>(t23445: F, t23486: F, t23532: F, t23569: F, t349: F, t23346: F, t23385: F, t23387: F, t23389: F, t23392: F, t23396: F, t23399: F, t23403: F, t23408: F, t23410: F, t388: F, t6687: F, t6692: F, t23384: F, t1049: F, t6688: F, t6691: F, t1054: F, t1065: F, t1921: F, t986: F, t2978: F, t344: F, t381: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t23571, t23572, t23574) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1846::<F>(t23445, t23486, t23532, t23569, t349, t23346, t23385, t23387, t23389, t23392, t23396, t23399, t23403, t23408, t23410, t388, t6687, t6692);
        let (t23579, t23581, t23582, t23587, t23588, t23589, t23592, t23593) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1847::<F>(t23384, t6692, t1049, t6688, t6691, t1054, t1065, t1921, t986, t2978, t344, t381);
    (t23571, t23572, t23574, t23579, t23581, t23582, t23587, t23588, t23589, t23592, t23593)
}
