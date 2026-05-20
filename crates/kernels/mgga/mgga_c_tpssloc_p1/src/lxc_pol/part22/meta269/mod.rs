//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta269 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1412;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1413;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta269<F: Float>(t12129: F, t17: F, t521: F, t9861: F, t3826: F, t592: F, t1285: F, t2225: F, t2371: F, t3691: F, t1294: F, t9494: F, t2535: F, t1995: F, t68: F, t1372: F, t3787: F, t215: F, t535: F, t9569: F, t1314: F, t2559: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t12130, t12132, t12133, t12134, t12136, t12138, t12141) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1412::<F>(t12129, t17, t521, t9861, t3826, t592, t1285, t2225, t2371, t3691, t1294, t9494);
        let (t12142, t12155, t12171, t12188, t12189) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1413::<F>(t2535, t3691, t1995, t68, t1372, t3787, t215, t535, t9569, t1314, t2559);
    (t12130, t12132, t12133, t12134, t12136, t12138, t12141, t12142, t12155, t12171, t12188, t12189)
}
