//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta446 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1993;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1994;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta446<F: Float>(t1227: F, t15643: F, t11705: F, t11719: F, t11728: F, t11734: F, t11746: F, t15610: F, t15612: F, t15617: F, t15622: F, t15627: F, t15631: F, t15637: F, t15642: F, t3490: F, t3496: F, t3506: F, t3515: F, t4974: F, t4984: F, t5019: F, t12652: F, t4972: F, t4582: F, t11153: F, t3584: F, t14165: F, t1734: F, t3508: F) -> (F, F, F, F, F, F, F, F) {
        let (t15645, t15648) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1993::<F>(t1227, t15643, t11705, t11719, t11728, t11734, t11746, t15610, t15612, t15617, t15622, t15627, t15631, t15637, t15642, t3490, t3496, t3506, t3515, t4974, t4984, t5019);
        let (t15649, t15650, t15654, t15655, t15656, t15659) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1994::<F>(t12652, t4972, t4582, t11153, t3584, t14165, t1734, t3508);
    (t15645, t15648, t15649, t15650, t15654, t15655, t15656, t15659)
}
