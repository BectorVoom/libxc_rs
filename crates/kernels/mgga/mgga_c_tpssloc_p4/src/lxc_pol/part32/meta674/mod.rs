//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta674 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2109;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2110;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta674<F: Float>(t27604: F, t3523: F, t24683: F, t24746: F, t8027: F, t4928: F, t52: F, t2132: F, t8040: F, t86292: F, t15689: F, t7310: F, t27674: F, t3548: F, t15753: F, t27608: F, t7321: F, t1222: F, t27586: F, t3540: F, t8049: F, t2136: F, t3966: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t95465, t95480, t95484, t95487, t95491, t95507) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2109::<F>(t27604, t3523, t24683, t24746, t8027, t4928, t52, t2132, t8040, t86292, t15689, t7310);
        let (t95511, t95512, t95515, t95517, t95520, t95540) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2110::<F>(t27674, t3548, t15753, t7310, t27608, t7321, t1222, t27586, t3540, t8049, t2132, t2136, t3966);
    (t95465, t95480, t95484, t95487, t95491, t95507, t95511, t95512, t95515, t95517, t95520, t95540)
}
