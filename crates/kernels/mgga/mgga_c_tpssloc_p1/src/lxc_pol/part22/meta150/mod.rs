//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta150 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk950;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk951;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk952;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk953;
use chunk4::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk954;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta150<F: Float>(t4343: F, t4518: F, t3966: F, t978: F, t977: F, t135: F, t1599: F, t973: F, t1597: F, t2987: F, t2990: F, t2824: F, t3003: F, t4384: F, t4387: F, t4390: F, t4393: F, t340: F, t343: F, t974: F, t984: F, t1593: F, t1600: F, t2958: F, t2960: F, t2969: F, t2972: F, t2975: F, t2986: F, t4507: F, t4511: F, t4515: F, t381: F, t1049: F, t1603: F, t1604: F, t225: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t4519, t4522, t4523, t4528, t4529, t4531) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk950::<F>(t4343, t4518, t3966, t978, t977, t135, t1599, t973, t1597, t2987);
        let (t4532, t4540) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk951::<F>(t2990, t4531, t2824, t3003, t4384, t4387, t4390, t4393);
        let (t4542, t4543, t4546) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk952::<F>(t340, t4540, t343, t974);
        let (t4548, t4552) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk953::<F>(t1597, t984, t343, t4546, t1593, t1600, t2958, t2960, t2969, t2972, t2975, t2986, t4507, t4511, t4515, t4519, t4523, t4529, t4532, t4543, t973);
        let (t4553, t4555, t4557) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk954::<F>(t381, t4552, t1049, t1603, t1604, t225);
    (t4522, t4528, t4529, t4531, t4540, t4542, t4546, t4548, t4552, t4553, t4555, t4557)
}
