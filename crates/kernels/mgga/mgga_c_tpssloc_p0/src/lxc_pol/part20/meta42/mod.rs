//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta42 (260520-c91 hierarchical CSE).
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
mod chunk9;
mod chunk10;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk306;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk307;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk308;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk309;
use chunk4::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk310;
use chunk5::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk311;
use chunk6::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk312;
use chunk7::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk313;
use chunk8::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk314;
use chunk9::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk315;
use chunk10::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk316;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta42<F: Float>(t819: F, t820: F, t829: F, t590: F, t61: F, t241: F, t244: F, t248: F, t238: F, t234: F, t236: F, t240: F, t812: F, t200: F, t243: F, t67: F, t776: F, t249: F, t787: F, t803: F, t805: F, t809: F, t817: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t831 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk306::<F>(t819, t820, t829);
        let t835 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk307::<F>(t590, t61);
        let t836 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk308::<F>(t241, t835);
        let t838 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk309::<F>(t244, t248, t836);
        let (t840, t841) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk310::<F>(t238, t838, t234, t236);
        let t842 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk311::<F>(t240, t841);
        let t843 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk312::<F>(t812, t842);
        let t845 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk313::<F>(t200, t243);
        let t847 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk314::<F>(t241, t67, t845);
        let t849 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk315::<F>(t776, t820, t847);
        let t852 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk316::<F>(t249, t787, t803, t805, t809, t817, t831, t840, t843, t849);
    (t831, t835, t836, t838, t840, t841, t842, t843, t845, t847, t849, t852)
}
