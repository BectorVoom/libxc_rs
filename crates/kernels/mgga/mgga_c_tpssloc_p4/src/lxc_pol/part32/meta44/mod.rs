//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta44 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk311;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk312;
use chunk2::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk313;
use chunk3::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk314;
use chunk4::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk315;
use chunk5::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk316;
use chunk6::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk317;
use chunk7::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk318;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta44<F: Float>(t257: F, t68: F, t252: F, t814: F, t829: F, t235: F, t852: F, t226: F, t255: F, t808: F, t812: F, t259: F, t799: F, t853: F, t855: F, t261: F, t193: F, t202: F, t680: F, t705: F, t710: F, t719: F, t752: F, t755: F, t760: F, t765: F, t766: F, t776: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t856, t857) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk311::<F>(t257);
        let t858 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk312::<F>(t68, t857);
        let t860 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk313::<F>(t252, t814);
        let (t861, t863, t865) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk314::<F>(t829, t860, t235, t852, t226, t255, t808, t812);
        let t866 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk315::<F>(t858, t865);
        let t868 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk316::<F>(t259, t799, t853, t855, t866);
        let t870 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk317::<F>(t261);
        let t873 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk318::<F>(t193, t202, t680, t705, t710, t719, t752, t755, t760, t765, t766, t776, t868, t870);
    (t856, t857, t858, t860, t861, t863, t865, t866, t868, t870, t873)
}
