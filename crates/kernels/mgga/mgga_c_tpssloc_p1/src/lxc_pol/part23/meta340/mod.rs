//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta340 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1118;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1119;
use chunk2::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1120;
use chunk3::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1121;
use chunk4::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1122;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta340<F: Float>(t11985: F, t526: F, t11998: F, t528: F, t2405: F, t2419: F, t690: F, t703: F, t2410: F, t2414: F, t701: F, t268: F, t682: F, t781: F, t204: F, t2421: F, t2411: F) -> (F, F, F, F, F, F, F) {
        let (t39419, t39436, t39463) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1118::<F>(t11985, t526, t11998, t528, t2405, t2419, t690, t703);
        let t39468 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1119::<F>(t2405, t2410, t2414, t690, t701);
        let t39472 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1120::<F>(t268, t682, t703, t781);
        let t39476 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1121::<F>(t204, t2419, t2421, t268);
        let t39483 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1122::<F>(t2405, t2411, t2421);
    (t39419, t39436, t39463, t39468, t39472, t39476, t39483)
}
