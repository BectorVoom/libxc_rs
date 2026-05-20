//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta43 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;
mod chunk8;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk296;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk297;
use chunk2::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk298;
use chunk3::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk299;
use chunk4::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk300;
use chunk5::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk301;
use chunk6::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk302;
use chunk7::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk303;
use chunk8::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk304;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta43<F: Float>(t244: F, t248: F, t836: F, t238: F, t234: F, t236: F, t240: F, t812: F, t200: F, t243: F, t241: F, t67: F, t776: F, t820: F, t249: F, t787: F, t803: F, t805: F, t809: F, t817: F, t831: F, t218: F, t225: F, t253: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let t838 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk296::<F>(t244, t248, t836);
        let (t840, t841) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk297::<F>(t238, t838, t234, t236);
        let t842 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk298::<F>(t240, t841);
        let t843 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk299::<F>(t812, t842);
        let t845 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk300::<F>(t200, t243);
        let t847 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk301::<F>(t241, t67, t845);
        let t849 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk302::<F>(t776, t820, t847);
        let t852 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk303::<F>(t249, t787, t803, t805, t809, t817, t831, t840, t843, t849);
        let (t853, t855) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk304::<F>(t218, t852, t225, t253);
    (t838, t840, t841, t842, t843, t845, t847, t849, t852, t853, t855)
}
