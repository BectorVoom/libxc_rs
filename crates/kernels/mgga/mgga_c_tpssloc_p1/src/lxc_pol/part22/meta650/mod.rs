//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta650 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2190;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2191;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta650<F: Float>(t16398: F, t19890: F, t12283: F, t19972: F, t16046: F, t1814: F, t12250: F, t5286: F, t1372: F, t6414: F, t1338: F, t20009: F, t19731: F, t562: F, t16576: F, t751: F, t2517: F, t5520: F, t17109: F, t870: F, t16689: F, t2430: F, t12945: F, t4205: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t57450, t57457, t57530, t57568, t57618, t57659) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2190::<F>(t16398, t19890, t12283, t19972, t16046, t1814, t12250, t5286, t1372, t6414, t1338, t20009);
        let (t57704, t57887, t57897, t57932, t57947, t57960) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2191::<F>(t19731, t562, t16576, t751, t2517, t5520, t17109, t870, t16689, t2430, t12945, t4205);
    (t57450, t57457, t57530, t57568, t57618, t57659, t57704, t57887, t57897, t57932, t57947, t57960)
}
