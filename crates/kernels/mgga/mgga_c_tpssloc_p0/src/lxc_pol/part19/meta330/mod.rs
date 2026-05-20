//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta330 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1177;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1178;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1179;
use chunk3::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1180;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta330<F: Float>(t12222: F, t16081: F, t116: F, t1314: F, t9534: F, t1307: F, t133: F, t6600: F, t12226: F, t16094: F, t3719: F, t686: F, t3736: F, t40018: F, t12012: F, t12220: F, t16101: F, t210: F, t213: F, t214: F, t221: F, t3733: F, t3734: F, t39622: F, t40343: F, t40347: F, t40350: F, t40351: F, t40356: F, t40360: F, t5195: F, t59: F, t9223: F, t120: F, t212: F, t22815: F, t67: F, t535: F, t1317: F, t40005: F, t12189: F, t3745: F, t9580: F, t3741: F, t2566: F, t3732: F, t12204: F, t12214: F, t792: F, t118: F, t12156: F, t794: F, t2229: F, t60: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t40366, t40372, t40376) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1177::<F>(t12222, t16081, t116, t1314, t9534, t1307, t133, t6600, t12226, t16094, t3719, t686);
        let t40389 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1178::<F>(t3736, t40018, t12012, t12220, t16101, t210, t213, t214, t221, t3719, t3733, t3734, t39622, t40343, t40347, t40350, t40351, t40356, t40360, t40366, t40372, t40376, t5195);
        let (t40394, t40399, t40401, t40402, t40404) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1179::<F>(t59, t9223, t116, t120, t212, t22815, t67, t535, t1317, t40005, t12189, t3745);
        let (t40407, t40410, t40415, t40419) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1180::<F>(t1314, t9580, t3741, t2566, t3732, t12204, t12214, t792, t118, t12156, t794, t2229, t59, t60);
    (t40389, t40394, t40399, t40401, t40402, t40404, t40407, t40410, t40415, t40419)
}
