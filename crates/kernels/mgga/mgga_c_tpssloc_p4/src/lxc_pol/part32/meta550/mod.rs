//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta550 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1905;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1906;
use chunk2::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1907;
use chunk3::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1908;
use chunk4::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1909;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta550<F: Float>(t1266: F, t2165: F, t2167: F, t2314: F, t26006: F, t26141: F, t26144: F, t26145: F, t26147: F, t26150: F, t26153: F, t26157: F, t4026: F, t4028: F, t4034: F, t5361: F, t7271: F, t7983: F, t7989: F, t1458: F, t7408: F, t2113: F, t671: F, t24932: F, t26109: F, t26111: F, t26113: F, t26116: F, t26119: F, t26121: F, t26123: F, t26125: F, t26137: F, t27371: F, t27863: F, t4072: F, t7266: F, t1393: F, t1459: F, t1849: F, t26166: F, t26170: F, t26178: F, t26181: F, t26183: F, t26505: F, t4037: F, t4073: F, t4077: F, t574: F, t652: F, t7412: F, t8107: F, t27860: F, t27867: F, t3: F, t112: F, t8110: F, t24969: F, t24972: F, t26533: F, t26535: F, t26537: F, t26539: F, t26541: F, t26544: F, t26547: F, t26549: F, t26552: F, t26554: F, t5376: F, t577: F, t7423: F) -> (F, F, F, F, F, F, F) {
        let t27878 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1905::<F>(t1266, t2165, t2167, t2314, t26006, t26141, t26144, t26145, t26147, t26150, t26153, t26157, t4026, t4028, t4034, t5361, t7271, t7983, t7989);
        let (t27879, t27888) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1906::<F>(t1458, t7408, t2113, t671);
        let t27903 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1907::<F>(t1458, t24932, t26109, t26111, t26113, t26116, t26119, t26121, t26123, t26125, t26137, t27371, t27863, t27888, t4072, t671, t7266);
        let t27905 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1908::<F>(t1393, t1459, t1849, t24932, t26166, t26170, t26178, t26181, t26183, t26505, t27879, t27888, t27903, t4037, t4073, t4077, t574, t652, t7266, t7412, t8107);
        let (t27907, t27908, t27921, t27930) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1909::<F>(t27860, t27867, t27878, t27905, t3, t112, t8110, t1458, t24969, t24972, t26533, t26535, t26537, t26539, t26541, t26544, t26547, t26549, t26552, t26554, t4072, t5376, t577, t671, t7423);
    (t27879, t27888, t27903, t27907, t27908, t27921, t27930)
}
