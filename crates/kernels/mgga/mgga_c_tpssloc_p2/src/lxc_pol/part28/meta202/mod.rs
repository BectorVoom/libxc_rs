//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta202 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk948;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk949;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta202<F: Float>(t4528: F, t973: F, t1597: F, t2987: F, t2990: F, t2824: F, t3003: F, t4384: F, t4387: F, t4390: F, t4393: F, t340: F, t343: F, t974: F, t984: F, t1593: F, t1600: F, t2958: F, t2960: F, t2969: F, t2972: F, t2975: F, t2986: F, t4507: F, t4511: F, t4515: F, t4519: F, t4523: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t4529, t4531, t4532, t4540, t4541) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk948::<F>(t4528, t973, t1597, t2987, t2990, t2824, t3003, t4384, t4387, t4390, t4393, t340);
        let (t4542, t4543, t4546, t4548, t4549, t4552) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk949::<F>(t343, t4541, t974, t340, t1597, t984, t1593, t1600, t2958, t2960, t2969, t2972, t2975, t2986, t4507, t4511, t4515, t4519, t4523, t4529, t4532, t973);
    (t4531, t4532, t4540, t4542, t4543, t4546, t4548, t4549, t4552)
}
