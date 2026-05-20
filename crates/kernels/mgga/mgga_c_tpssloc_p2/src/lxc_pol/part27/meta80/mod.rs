//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta80 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk527;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk528;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk529;
use chunk3::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk530;
use chunk4::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk531;
use chunk5::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk532;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta80<F: Float>(t1589: F, t959: F, t1409: F, t978: F, t977: F, t1554: F, t906: F, t340: F, t343: F, t974: F, t971: F, t973: F, t381: F, t998: F, t225: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t1591, t1592, t1593, t1597) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk527::<F>(t1589, t959, t1409, t978, t977, t1554, t906);
        let t1598 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk528::<F>(t1597, t340);
        let t1599 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk529::<F>(t1598, t343);
        let (t1600, t1603) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk530::<F>(t1599, t974, t1593, t971, t973);
        let (t1604, t1606, t1607) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk531::<F>(t1603, t381, t1409, t998, t974);
        let t1610 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk532::<F>(t1603, t225);
    (t1591, t1592, t1593, t1597, t1598, t1599, t1600, t1603, t1604, t1606, t1607, t1610)
}
