//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta52 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk360;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk361;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk362;
use chunk3::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk363;
use chunk4::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk364;
use chunk5::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk365;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta52<F: Float>(t607: F, t998: F, t974: F, t225: F, t990: F, t68: F, t369: F, t191: F, t349: F, t361: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t999, t1000, t1003) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk360::<F>(t607, t998, t974, t225, t990);
        let t1004 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk361::<F>(t1003, t68);
        let (t1005, t1008, t1009) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk362::<F>(t1004, t369, t191);
        let (t1010, t1011) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk363::<F>(t1009, t349, t68);
        let t1012 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk364::<F>(t1010, t1011);
        let (t1013, t1014) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk365::<F>(t361);
    (t999, t1000, t1003, t1004, t1005, t1008, t1009, t1010, t1011, t1012, t1013, t1014)
}
