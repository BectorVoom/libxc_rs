//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta53 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk366;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk367;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk368;
use chunk3::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk369;
use chunk4::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk370;
use chunk5::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk371;
use chunk6::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk372;
use chunk7::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk373;
use chunk8::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk374;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta53<F: Float>(t1014: F, t363: F, t336: F, t371: F, t368: F, t1012: F, t376: F, t61: F, t890: F, t916: F, t956: F, t958: F, t963: F, t360: F, t248: F, t34: F, t365: F, t35: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let t1015 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk366::<F>(t1014, t363);
        let (t1016, t1017) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk367::<F>(t336, t371);
        let (t1018, t1019) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk368::<F>(t1017, t368, t1015);
        let t1020 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk369::<F>(t1012, t1019);
        let t1021 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk370::<F>(t376, t61);
        let t1022 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk371::<F>(t890, t916, t956, t958, t963);
        let t1023 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk372::<F>(t1022, t360);
        let t1025 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk373::<F>(t1021, t1023, t248);
        let t1030 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk374::<F>(t34, t365, t35);
    (t1015, t1016, t1017, t1018, t1019, t1020, t1021, t1022, t1023, t1025, t1030)
}
