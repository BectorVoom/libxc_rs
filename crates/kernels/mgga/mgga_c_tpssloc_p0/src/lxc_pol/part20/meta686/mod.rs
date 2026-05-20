//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta686 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2599;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2600;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2601;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta686<F: Float>(t1734: F, t3507: F, t11721: F, t3493: F, t4978: F, t11786: F, t5005: F, t15730: F, t3536: F, t15594: F, t3523: F, t11678: F, t11684: F, t11805: F, t11809: F, t1215: F, t15569: F, t15659: F, t15660: F, t15761: F, t1653: F, t2244: F, t2250: F, t3247: F, t3490: F, t3578: F, t45197: F, t5024: F, t52687: F, t1174: F, t14726: F, t44562: F, t3577: F, t44951: F, t4953: F, t11677: F, t15245: F, t11665: F, t11668: F, t11670: F, t11694: F, t1177: F, t11853: F, t1227: F, t1230: F, t15714: F, t248: F, t3243: F, t3515: F, t44851: F, t44871: F, t4582: F, t4977: F, t5012: F, t50830: F, t50929: F, t14753: F, t3431: F, t14744: F, t11651: F, t15438: F, t13969: F, t15540: F, t15530: F, t11638: F, t11688: F, t15740: F, t3506: F, t3508: F, t44621: F, t44886: F, t44890: F, t44894: F, t50924: F) -> (F, F, F, F, F, F) {
        let (t52696, t52704, t52709, t52737) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2599::<F>(t1734, t3507, t11721, t3493, t4978, t11786, t5005, t15730, t3536, t15594, t3523, t11678, t11684, t11805, t11809, t1215, t15569, t15659, t15660, t15761, t1653, t2244, t2250, t3247, t3490, t3578, t45197, t5024, t52687);
        let t52769 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2600::<F>(t1174, t14726, t44562, t3577, t44951, t4953, t11677, t15245, t11665, t11668, t11670, t11694, t1177, t11853, t1227, t1230, t15569, t15714, t248, t3243, t3515, t44851, t44871, t4582, t4977, t5012, t50830, t50929);
        let t52797 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2601::<F>(t1174, t14753, t3431, t14744, t11651, t15438, t1227, t13969, t15540, t15530, t3515, t11638, t11688, t15740, t3506, t3508, t44621, t44886, t44890, t44894, t4582, t4977, t50924);
    (t52696, t52704, t52709, t52737, t52769, t52797)
}
