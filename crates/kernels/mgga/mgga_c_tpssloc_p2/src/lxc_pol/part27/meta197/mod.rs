//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta197 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1006;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1007;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta197<F: Float>(t4370: F, t894: F, t1547: F, t2815: F, t896: F, t901: F, t1553: F, t699: F, t2826: F, t4338: F, t136: F, t4343: F, t908: F, t4347: F, t2766: F, t2810: F, t2823: F, t2824: F, t4335: F, t4340: F, t4345: F, t4349: F, t4363: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t4371, t4378, t4379, t4381, t4384, t4386, t4387, t4389) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1006::<F>(t4370, t894, t1547, t2815, t896, t901, t1553, t699, t2826, t4338, t136, t4343, t908);
        let (t4390, t4392, t4393, t4395) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1007::<F>(t136, t4389, t4347, t908, t2766, t2810, t2823, t2824, t4335, t4340, t4345, t4349, t4363, t4371, t4379, t4381, t4384, t4387);
    (t4371, t4378, t4379, t4381, t4384, t4386, t4387, t4389, t4390, t4392, t4393, t4395)
}
