//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta51 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk353;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk354;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk355;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta51<F: Float>(t340: F, t984: F, t343: F, t974: F, t346: F, t964: F, t971: F, t973: F, t980: F, t381: F, t221: F, t967: F, t339: F, t883: F, t976: F, t607: F, t225: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t986, t987, t990) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk353::<F>(t340, t984, t343, t974, t346, t964, t971, t973, t980);
        let (t991, t995, t997, t998) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk354::<F>(t381, t990, t221, t967, t339, t883, t976);
        let (t999, t1000, t1003) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk355::<F>(t607, t998, t974, t225, t990);
    (t986, t987, t990, t991, t995, t997, t998, t999, t1000, t1003)
}
