//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta738 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2600;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2601;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta738<F: Float>(t3577: F, t44951: F, t4953: F, t11677: F, t15245: F, t1174: F, t14753: F, t3431: F, t14744: F, t11651: F, t15438: F, t1227: F, t13969: F, t15540: F, t15530: F, t3515: F, t11702: F, t5002: F, t11708: F, t15502: F, t15506: F, t15554: F, t3506: F, t10469: F, t1720: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t52758, t52766, t52773, t52776, t52781, t52792) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2600::<F>(t3577, t44951, t4953, t11677, t15245, t1174, t14753, t3431, t14744, t11651, t15438, t1227, t13969, t15540);
        let (t52795, t52801, t52810, t52813, t52817, t52834) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2601::<F>(t13969, t15530, t3515, t11702, t5002, t11708, t15502, t15506, t15554, t3506, t10469, t1720);
    (t52758, t52766, t52773, t52776, t52781, t52792, t52795, t52801, t52810, t52813, t52817, t52834)
}
