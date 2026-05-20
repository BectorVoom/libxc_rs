//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta94 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk614;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk615;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk616;
use chunk3::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk617;
use chunk4::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk618;
use chunk5::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk619;
use chunk6::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk620;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta94<F: Float>(t1983: F, t2020: F, t1401: F, t1873: F, t50: F, t56: F, t63: F, t67: F, t1864: F, t5: F, t1860: F, t112: F, t265: F, t394: F, t1964: F, t25: F, t1918: F, t40: F, t337: F, t1887: F, dens_threshold: F, rho0: F, zeta_threshold: F, t225: F, t491: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t2021, t2028, t2108, t2109) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk614::<F>(t1983, t2020, t1401, t1873, t50, t56, t63, t67);
        let t2110 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk615::<F>(t1864, t2109);
        let t2113 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk616::<F>(t5, t1860, t2110);
        let t2114 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk617::<F>(t112, t2113);
        let t2116 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk618::<F>(t265, t394, t1964);
        let (t2119, t2120, t2121) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk619::<F>(t25, t1918, t2116, t40, t337, t50, t1887, dens_threshold, rho0, zeta_threshold);
        let t2122 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk620::<F>(t225, t491);
    (t2021, t2028, t2108, t2109, t2110, t2113, t2114, t2116, t2119, t2120, t2121, t2122)
}
