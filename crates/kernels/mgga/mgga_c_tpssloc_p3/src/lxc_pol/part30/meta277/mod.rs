//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta277 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1255;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1256;
use chunk2::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1257;
use chunk3::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1258;
use chunk4::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1259;
use chunk5::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1260;
use chunk6::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1261;
use chunk7::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1262;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta277<F: Float>(t1539: F, t6785: F, t6784: F, t1599: F, t1949: F, t1629: F, t6800: F, t6799: F, t1625: F, t1948: F, t345: F, t1615: F, t1945: F, t1060: F, t383: F, t7593: F, t1058: F, t1610: F, t1920: F, t1953: F, t353: F, t6687: F, t6783: F, t6797: F, t1055: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t7603 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1255::<F>(t1539, t6785);
        let t7604 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1256::<F>(t6784, t7603);
        let t7607 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1257::<F>(t1599, t1949);
        let (t7610, t7611) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1258::<F>(t1629, t6800, t6799);
        let t7614 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1259::<F>(t1625, t1948);
        let (t7615, t7619) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1260::<F>(t345, t7614, t1615, t1945);
        let (t7620, t7622, t7624) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1261::<F>(t1060, t7619, t383, t7593, t1058, t1610, t1920, t1953, t353, t6687, t6783, t6797, t7604, t7607, t7611, t7615);
        let t7625 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1262::<F>(t1055, t7624);
    (t7603, t7604, t7607, t7610, t7611, t7614, t7615, t7619, t7620, t7622, t7624, t7625)
}
