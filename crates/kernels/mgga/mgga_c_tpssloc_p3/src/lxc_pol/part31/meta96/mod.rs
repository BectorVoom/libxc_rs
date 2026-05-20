//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta96 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk590;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk591;
use chunk2::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk592;
use chunk3::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk593;
use chunk4::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk594;
use chunk5::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk595;
use chunk6::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk596;
use chunk7::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk597;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta96<F: Float>(t2085: F, t539: F, t553: F, t2011: F, t544: F, t1378: F, t1375: F, t1989: F, t568: F, t533: F, t1390: F, t113: F, t1983: F, t2036: F, t2040: F, t2075: F, t2079: F, t510: F, t574: F, t652: F, t3: F, t1401: F, t2039: F, t577: F, t11: F, t2: F, t584: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t2086, t2089) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk590::<F>(t2085, t539, t553);
        let t2091 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk591::<F>(t2011, t2089, t544);
        let t2092 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk592::<F>(t1378, t2091);
        let t2094 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk593::<F>(t1375, t1989, t2086, t2092, t568);
        let t2095 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk594::<F>(t2094, t533);
        let t2096 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk595::<F>(t1390, t2095);
        let t2098 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk596::<F>(t113, t1983, t2036, t2040, t2075, t2079, t2096, t510, t574, t652);
        let (t2099, t2105, t2218, t2219) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk597::<F>(t2098, t3, t1401, t2039, t577, t11, t2, t584);
    (t2086, t2089, t2091, t2092, t2094, t2095, t2096, t2098, t2099, t2105, t2218, t2219)
}
