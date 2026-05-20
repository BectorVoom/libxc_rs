//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta245 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1176;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1177;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1178;
use chunk3::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1179;
use chunk4::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1180;
use chunk5::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1181;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta245<F: Float>(t25: F, t868: F, t1877: F, t1915: F, t2522: F, t606: F, t6542: F, t6666: F, t6670: F, t337: F, t614: F, t1887: F, t1922: F, t968: F, t1920: F, t221: F, t60: F, t1926: F, t344: F, t976: F, t381: F, t225: F, t387: F, t884: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t6671, t6678, t6679, t6680) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1176::<F>(t25, t868, t1877, t1915, t2522, t606, t6542, t6666, t6670, t337, t614, t1887);
        let (t6683, t6685, t6686, t6687) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1177::<F>(t1922, t968, t1920, t221, t60, t1926);
        let t6688 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1178::<F>(t344, t976);
        let t6689 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1179::<F>(t381, t6688);
        let t6690 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1180::<F>(t225, t387);
        let t6691 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1181::<F>(t6690, t884);
    (t6671, t6678, t6679, t6680, t6683, t6685, t6686, t6687, t6688, t6689, t6690, t6691)
}
