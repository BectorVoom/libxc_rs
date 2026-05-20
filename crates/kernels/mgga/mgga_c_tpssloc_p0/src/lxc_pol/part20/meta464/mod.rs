//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta464 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1929;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1930;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta464<F: Float>(t15540: F, t4582: F, t12648: F, t4987: F, t13969: F, t4983: F, t3515: F, t486: F, t5011: F, t4978: F, t11709: F, t11738: F, t11814: F, t11825: F, t1213: F, t1227: F, t15524: F, t15527: F, t15531: F, t15535: F, t1737: F, t1748: F, t3490: F, t3506: F, t3531: F, t3536: F, t4980: F, t4989: F, t5014: F, t5024: F) -> (F, F, F, F, F, F, F, F) {
        let (t15541, t15544, t15545, t15548, t15550, t15553) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1929::<F>(t15540, t4582, t12648, t4987, t13969, t4983, t3515, t486, t5011);
        let (t15554, t15555, t15558) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1930::<F>(t15553, t4978, t4582, t11709, t11738, t11814, t11825, t1213, t1227, t15524, t15527, t15531, t15535, t15541, t15545, t15550, t1737, t1748, t3490, t3506, t3515, t3531, t3536, t4980, t4989, t5014, t5024);
    (t15541, t15544, t15545, t15548, t15553, t15554, t15555, t15558)
}
