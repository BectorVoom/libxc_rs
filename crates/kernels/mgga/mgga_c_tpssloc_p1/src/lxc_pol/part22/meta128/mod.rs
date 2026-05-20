//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta128 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk855;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk856;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk857;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk858;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta128<F: Float>(t17: F, t3826: F, t1285: F, t592: F, t1287: F, t588: F, t1365: F, t68: F, t248: F, t2691: F, t557: F, t555: F, t1361: F, t835: F, t1336: F, t1369: F, t1995: F, t241: F, t67: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t3827, t3829, t3832, t3833, t3836, t3843, t3862) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk855::<F>(t17, t3826, t1285, t592, t1287, t588, t1365, t68, t248, t2691, t557);
        let (t3864, t3865) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk856::<F>(t3862, t555, t1361, t835);
        let t3866 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk857::<F>(t1336, t3865);
        let (t3867, t3870) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk858::<F>(t1369, t3866, t1995, t241, t67);
    (t3827, t3829, t3832, t3833, t3836, t3843, t3862, t3864, t3865, t3866, t3867, t3870)
}
