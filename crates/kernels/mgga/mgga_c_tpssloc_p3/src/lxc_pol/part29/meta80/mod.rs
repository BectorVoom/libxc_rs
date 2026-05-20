//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta80 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk528;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk529;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk530;
use chunk3::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk531;
use chunk4::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk532;
use chunk5::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk533;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta80<F: Float>(t1611: F, t369: F, t1545: F, t1559: F, t1585: F, t1587: F, t1591: F, t360: F, t1021: F, t248: F, t1044: F, t1539: F, t1020: F, t1038: F, t1041: F, t1607: F, t378: F, t973: F, t997: F, t349: F, t381: F) -> (F, F, F, F, F, F, F, F) {
        let (t1612, t1615) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk528::<F>(t1611, t369, t1545, t1559, t1585, t1587, t1591);
        let t1616 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk529::<F>(t1615, t360);
        let t1618 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk530::<F>(t1021, t1616, t248);
        let t1622 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk531::<F>(t1044, t1539, t248);
        let t1625 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk532::<F>(t1020, t1038, t1041, t1607, t1612, t1618, t1622, t378, t973, t997);
        let (t1626, t1629) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk533::<F>(t1625, t349, t1615, t381);
    (t1612, t1615, t1616, t1618, t1622, t1625, t1626, t1629)
}
