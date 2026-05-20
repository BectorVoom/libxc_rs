//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta339 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1725;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1726;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1727;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1728;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1729;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta339<F: Float>(t12757: F, t666: F, t2358: F, t4043: F, t1444: F, t2342: F, t9384: F, t2341: F, t92: F, t2219: F, t659: F, t2248: F, t4049: F, t584: F, t95: F, t16: F, t4053: F, t1449: F, t2350: F, t9398: F, t100: F, t2349: F, t662: F, t2354: F, t4059: F, t103: F, t4063: F, t1445: F, t1447: F, t2336: F, t2351: F, t2355: F, t4050: F, t4054: F, t657: F, t656: F, t12747: F, t12750: F, t12752: F, t12754: F, t64: F, t9358: F, t9359: F, t9361: F, t9363: F, t109: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t12758, t12761, t12771, t12774, t12775, t12778) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1725::<F>(t12757, t666, t2358, t4043, t1444, t2342, t9384, t2341, t92, t2219, t659, t2248, t4049);
        let (t12781, t12784, t12792, t12795, t12796, t12799) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1726::<F>(t584, t95, t16, t4053, t1449, t2350, t9398, t100, t2349, t2219, t662, t2354, t4059);
        let (t12802, t12805, t12808) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1727::<F>(t103, t584, t16, t4063, t100, t12771, t12774, t12775, t12778, t12781, t12784, t12792, t12795, t12796, t12799, t1445, t1447, t2336, t2351, t2355, t4050, t4054, t657, t92);
        let (t12809, t12812) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1728::<F>(t12808, t656, t12747, t12750, t12752, t12754, t12758, t12761, t64, t9358, t9359, t9361, t9363);
        let t12813 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1729::<F>(t109, t12812);
    (t12758, t12761, t12774, t12792, t12795, t12796, t12799, t12802, t12805, t12808, t12809, t12813)
}
