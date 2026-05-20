//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta51 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk351;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk352;
use chunk2::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk353;
use chunk3::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk354;
use chunk4::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk355;
use chunk5::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk356;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta51<F: Float>(t607: F, t978: F, t977: F, t906: F, t910: F, t340: F, t343: F, t974: F, t346: F, t964: F, t971: F, t973: F, t381: F, t221: F, t967: F, t339: F, t883: F, t976: F, t225: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t979, t980, t984) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk351::<F>(t607, t978, t977, t906, t910);
        let t986 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk352::<F>(t340, t984, t343);
        let t990 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk353::<F>(t974, t986, t346, t964, t971, t973, t980);
        let (t991, t995) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk354::<F>(t381, t990, t221, t967);
        let (t997, t998) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk355::<F>(t339, t995, t883, t976);
        let (t999, t1000, t1003) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk356::<F>(t607, t998, t974, t225, t990);
    (t979, t984, t986, t990, t991, t995, t997, t998, t999, t1000, t1003)
}
