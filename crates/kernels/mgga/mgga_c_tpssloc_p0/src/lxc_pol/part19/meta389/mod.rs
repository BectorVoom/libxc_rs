//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta389 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1462;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1463;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1464;
use chunk3::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1465;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta389<F: Float>(t3242: F, t415: F, t61: F, t42341: F, t44696: F, t42344: F, t483: F, t1210: F, t1174: F, t3561: F, t698: F, t11738: F, t11739: F, t248: F, t3570: F, t10471: F, t44690: F, t11727: F, t44722: F, t478: F, t11719: F, t11722: F, t3507: F, t486: F, t11655: F, t11731: F, t11825: F, t1214: F, t1227: F, t15615: F, t15654: F, t3490: F, t3494: F, t3555: F, t3587: F, t39097: F, t39103: F, t42468: F, t43764: F, t44699: F, t44725: F, t44803: F, t44805: F, t44811: F, t44817: F, t4582: F, t475: F, t974: F, t11638: F, t11818: F, t1213: F, t3506: F, t3509: F, t3515: F, t3516: F, t11718: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t44828, t44833, t44834, t44836, t44847, t44851) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1462::<F>(t3242, t415, t61, t42341, t44696, t42344, t483, t1210, t1174, t3561, t698, t11738, t11739, t248, t3570);
        let (t44857, t44858, t44863, t44871, t44873) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1463::<F>(t10471, t44690, t11727, t44722, t44833, t44834, t478, t11719, t11722, t248, t3570, t3507, t486);
        let t44878 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1464::<F>(t11655, t11731, t11738, t1174, t11825, t1214, t1227, t15615, t15654, t248, t3490, t3494, t3555, t3587, t39097, t39103, t42468, t43764, t44699, t44725, t44803, t44805, t44811, t44817, t44828, t44836, t44847, t44851, t44858, t44863, t44871, t44873, t4582, t475, t974);
        let (t44879, t44886, t44890, t44894, t44896) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1465::<F>(t11638, t486, t11818, t1213, t248, t3494, t3506, t3509, t3515, t3516, t11718, t44857);
    (t44833, t44834, t44857, t44873, t44878, t44879, t44886, t44890, t44894, t44896)
}
