//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta678 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2116;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2117;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta678<F: Float>(t27553: F, t95772: F, t477: F, t5052: F, t27654: F, t7327: F, t24745: F, t4935: F, t24585: F, t7999: F, t24574: F, t27800: F, t225: F, t27805: F, t27392: F, t1170: F, t2121: F, t27766: F, t2154: F, t45349: F, t27776: F, t11147: F, t497: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t95774, t95794, t95803, t95813, t95824, t95834) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2116::<F>(t27553, t95772, t477, t5052, t27654, t7327, t24745, t4935, t24585, t7999, t24574, t27800);
        let (t95836, t95863, t95866, t95884, t95889, t95890) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2117::<F>(t225, t27805, t24574, t27392, t1170, t2121, t27766, t2154, t45349, t27776, t95772, t11147, t497);
    (t95774, t95794, t95803, t95813, t95824, t95834, t95836, t95863, t95866, t95884, t95889, t95890)
}
