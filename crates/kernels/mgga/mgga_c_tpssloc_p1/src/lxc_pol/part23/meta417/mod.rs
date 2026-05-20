//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta417 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1236;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1237;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta417<F: Float>(t13012: F, t20927: F, t12984: F, t12998: F, t5544: F, t686: F, t20933: F, t2563: F, t20923: F, t41011: F, t118: F, t20756: F, t41170: F, t794: F, t20800: F, t2576: F, t21008: F, t9573: F, t20896: F, t2697: F, t13360: F, t5624: F, t1516: F, t58844: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t68073, t68110, t68116, t68118, t68122) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1236::<F>(t13012, t20927, t12984, t12998, t5544, t686, t20933, t2563, t20923, t41011, t118, t20756, t41170, t794);
        let (t68131, t68148, t68195, t68197, t68199) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1237::<F>(t118, t20800, t2576, t794, t21008, t9573, t20896, t2697, t13360, t5624, t1516, t58844);
    (t68073, t68110, t68116, t68118, t68122, t68131, t68148, t68195, t68197, t68199)
}
