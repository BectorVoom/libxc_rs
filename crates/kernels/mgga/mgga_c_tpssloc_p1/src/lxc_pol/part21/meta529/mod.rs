//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta529 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2185;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2186;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta529<F: Float>(t1023: F, t5677: F, t10408: F, t1036: F, t5905: F, t1041: F, t10876: F, t10883: F, t10952: F, t13995: F, t14158: F, t14160: F, t17972: F, t17976: F, t17980: F, t17984: F, t17988: F, t17991: F, t17994: F, t3070: F, t3109: F, t4579: F, t5869: F, t5880: F, t973: F, t4571: F, t4644: F, t1031: F, t5904: F, t1022: F, t1539: F, t14211: F, t3071: F, t5685: F, t1616: F, t4343: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t17997, t17998, t18007) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2185::<F>(t1023, t5677, t10408, t1036, t5905, t1041, t10876, t10883, t10952, t13995, t14158, t14160, t17972, t17976, t17980, t17984, t17988, t17991, t17994, t3070, t3109, t4579, t5869, t5880, t973);
        let (t18008, t18010, t18014, t18015, t18016, t18020, t18021, t18024) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2186::<F>(t4571, t4644, t1031, t5904, t1022, t1539, t14211, t3071, t1023, t5685, t1616, t4343);
    (t17997, t17998, t18007, t18008, t18010, t18014, t18015, t18016, t18020, t18021, t18024)
}
