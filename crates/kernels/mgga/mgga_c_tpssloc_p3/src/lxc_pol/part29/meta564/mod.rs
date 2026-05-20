//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta564 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1974;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1975;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1976;
use chunk3::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1977;
use chunk4::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1978;
use chunk5::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1979;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta564<F: Float>(t265: F, t504: F, t27421: F, t27757: F, t27797: F, t27832: F, t3640: F, t8090: F, t1254: F, t1763: F, t1256: F, t193: F, t24905: F, t24909: F, t25882: F, t336: F, t4700: F, t5091: F, t7398: F, t28: F, t1409: F, t2161: F, t25949: F, t3966: F, t52: F, t607: F, t7402: F, t8097: F, t27380: F, t113: F, t24988: F, t24989: F, t24993: F, t24998: F, t25005: F, t25007: F, t25011: F, t25969: F, t25973: F, t27290: F, t27293: F, t27371: F, t510: F, t650: F, t652: F, t8103: F, dens_threshold: F, rho1: F, zeta_threshold: F, t111: F, t7982: F, t1442: F, t1774: F, t2114: F, t25975: F, t25977: F, t25979: F, t25982: F, t25987: F, t25991: F, t25993: F, t25996: F, t25998: F, t26002: F, t26005: F, t5107: F, t672: F, t7264: F, t7408: F, t1266: F, t2165: F, t2167: F, t2314: F, t26006: F, t26141: F, t26144: F, t26145: F, t26147: F, t26150: F, t26153: F, t26157: F, t4026: F, t4028: F, t4034: F, t5361: F, t7271: F, t7983: F, t7989: F, t1458: F, t2113: F, t671: F, t24932: F, t26109: F, t26111: F, t26113: F, t26116: F, t26119: F, t26121: F, t26123: F, t26125: F, t26137: F, t4072: F, t7266: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t27834, t27838, t27843, t27850) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1974::<F>(t265, t504, t27421, t27757, t27797, t27832, t3640, t8090, t1254, t1763, t1256, t193, t24905, t24909, t25882, t336, t4700, t5091, t7398);
        let (t27858, t27860) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1975::<F>(t28, t1409, t2161, t25949, t27850, t3966, t52, t607, t7402, t8097, t27380, t113, t24988, t24989, t24993, t24998, t25005, t25007, t25011, t25969, t25973, t27290, t27293, t27371, t510, t650, t652, t8103, dens_threshold, rho1, zeta_threshold);
        let t27863 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1976::<F>(t111, t7982);
        let t27867 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1977::<F>(t1442, t1774, t2114, t25975, t25977, t25979, t25982, t25987, t25991, t25993, t25996, t25998, t26002, t26005, t27863, t5107, t672, t7264, t7408);
        let t27878 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1978::<F>(t1266, t2165, t2167, t2314, t26006, t26141, t26144, t26145, t26147, t26150, t26153, t26157, t4026, t4028, t4034, t5361, t7271, t7983, t7989);
        let (t27879, t27888, t27903) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1979::<F>(t1458, t7408, t2113, t671, t24932, t26109, t26111, t26113, t26116, t26119, t26121, t26123, t26125, t26137, t27371, t27863, t4072, t7266);
    (t27834, t27838, t27843, t27850, t27858, t27860, t27863, t27867, t27878, t27879, t27888, t27903)
}
