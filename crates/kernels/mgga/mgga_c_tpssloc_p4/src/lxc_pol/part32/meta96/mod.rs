//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta96 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk613;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk614;
use chunk2::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk615;
use chunk3::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk616;
use chunk4::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk617;
use chunk5::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk618;
use chunk6::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk619;
use chunk7::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk620;
use chunk8::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk621;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta96<F: Float>(t2108: F, t67: F, t1864: F, t5: F, t1860: F, t112: F, t265: F, t394: F, t1964: F, t25: F, t1918: F, t40: F, t337: F, t50: F, t1887: F, dens_threshold: F, rho0: F, zeta_threshold: F, t225: F, t491: F, t497: F, t462: F, t131: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t2109 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk613::<F>(t2108, t67);
        let t2110 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk614::<F>(t1864, t2109);
        let t2113 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk615::<F>(t5, t1860, t2110);
        let t2114 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk616::<F>(t112, t2113);
        let t2116 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk617::<F>(t265, t394, t1964);
        let (t2119, t2120, t2121) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk618::<F>(t25, t1918, t2116, t40, t337, t50, t1887, dens_threshold, rho0, zeta_threshold);
        let t2122 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk619::<F>(t225, t491);
        let t2123 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk620::<F>(t2122, t497);
        let (t2124, t2127) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk621::<F>(t2123, t462, t131, t2120);
    (t2109, t2110, t2113, t2114, t2116, t2119, t2120, t2121, t2122, t2123, t2124, t2127)
}
