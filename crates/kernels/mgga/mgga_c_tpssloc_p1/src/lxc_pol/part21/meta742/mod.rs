//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta742 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2608;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2609;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta742<F: Float>(t10477: F, t1742: F, t11713: F, t3503: F, t1210: F, t11719: F, t13969: F, t15626: F, t11529: F, t1174: F, t4729: F, t11647: F, t1731: F, t1227: F, t15616: F, t14706: F, t248: F, t3521: F, t11814: F, t4997: F, t15492: F, t3536: F, t11692: F, t11697: F, t15703: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t53081, t53083, t53087, t53093, t53096, t53099) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2608::<F>(t10477, t1742, t11713, t3503, t1210, t11719, t13969, t15626, t11529, t1174, t4729, t11647, t1731);
        let (t53102, t53114, t53116, t53118, t53135) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2609::<F>(t1227, t13969, t15616, t14706, t248, t3521, t11814, t4997, t15492, t3536, t11692, t11697, t15703);
    (t53081, t53083, t53087, t53093, t53096, t53099, t53102, t53114, t53116, t53118, t53135)
}
