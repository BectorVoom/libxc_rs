//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta362 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1784;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1785;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1786;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta362<F: Float>(t2684: F, t4295: F, t13171: F, t860: F, t4265: F, t814: F, t829: F, t13377: F, t235: F, t2679: F, t4282: F, t4280: F, t808: F, t13384: F, t13176: F, t13336: F, t1499: F, t1523: F, t1525: F, t226: F, t255: F, t2613: F, t2617: F, t2738: F, t2740: F, t4162: F, t4166: F, t4283: F, t4286: F, t4288: F, t4291: F, t4298: F, t812: F, t861: F, t863: F, t9612: F, t13425: F, t858: F, t225: F, t4149: F, t13050: F, t13053: F, t13059: F, t13062: F, t13065: F, t13068: F, t13072: F, t13378: F, t259: F, t2597: F, t2713: F, t2720: F, t4268: F, t4273: F, t4301: F, t855: F, t866: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t13429, t13431, t13433, t13434, t13448, t13450, t13453) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1784::<F>(t2684, t4295, t13171, t860, t4265, t814, t829, t13377, t235, t2679, t4282, t4280, t808);
        let (t13456, t13459) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1785::<F>(t13384, t829, t13176, t13336, t13429, t13431, t13434, t13448, t13450, t13453, t1499, t1523, t1525, t226, t255, t2613, t2617, t2738, t2740, t4162, t4166, t4283, t4286, t4288, t4291, t4298, t808, t812, t861, t863, t9612);
        let (t13460, t13461, t13463, t13470) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1786::<F>(t13425, t13459, t858, t225, t4149, t13050, t13053, t13059, t13062, t13065, t13068, t13072, t13378, t259, t2597, t2713, t2720, t4268, t4273, t4301, t855, t866);
    (t13429, t13431, t13433, t13434, t13448, t13450, t13453, t13456, t13460, t13461, t13463, t13470)
}
