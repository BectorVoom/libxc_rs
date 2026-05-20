//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta332 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1619;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1620;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1621;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta332<F: Float>(t1174: F, t11761: F, t11766: F, t11770: F, t11774: F, t11781: F, t11787: F, t11792: F, t11794: F, t11798: F, t11802: F, t11805: F, t11809: F, t11814: F, t1218: F, t1227: F, t3515: F, t486: F, t676: F, t1216: F, t248: F, t1213: F, t1226: F, t3566: F) -> (F, F, F, F, F) {
        let t11817 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1619::<F>(t1174, t11761, t11766, t11770, t11774, t11781, t11787, t11792, t11794, t11798, t11802, t11805, t11809, t11814, t1218, t1227, t3515);
        let t11818 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1620::<F>(t486, t676);
        let (t11820, t11821, t11825) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1621::<F>(t11818, t1216, t248, t1213, t1226, t3566);
    (t11817, t11818, t11820, t11821, t11825)
}
