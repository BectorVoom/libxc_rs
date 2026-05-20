//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta275 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1038;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta275<F: Float>(t12083: F, t182: F, t1294: F, t9722: F, t172: F, t3681: F, t763: F, t2528: F, t3691: F, t9919: F, t12051: F, t12053: F, t12055: F, t12057: F, t12059: F, t9789: F, t9793: F, t9797: F) -> (F, F, F, F, F, F, F) {
        let (t12085, t12087, t12088, t12090, t12092, t12094, t12095) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1038::<F>(t12083, t182, t1294, t9722, t172, t3681, t763, t2528, t3691, t9919, t12051, t12053, t12055, t12057, t12059, t9789, t9793, t9797);
    (t12085, t12087, t12088, t12090, t12092, t12094, t12095)
}
