//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta340 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1730;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1731;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1732;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta340<F: Float>(t1268: F, t12724: F, t12725: F, t12728: F, t12734: F, t12739: F, t12813: F, t1458: F, t2314: F, t2363: F, t4028: F, t4072: F, t5113: F, t671: F, t9348: F, t89: F, t12545: F, t12550: F, t12557: F, t1442: F, t1459: F, t1849: F, t2323: F, t2364: F, t3652: F, t3660: F, t4034: F, t4037: F, t4073: F, t574: F, t652: F, t672: F, t510: F, t4098: F, t751: F, t2752: F, t4303: F, t172: F, t4095: F, t763: F, t1472: F, t2517: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let t12816 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1730::<F>(t1268, t12724, t12725, t12728, t12734, t12739, t12813, t1458, t2314, t2363, t4028, t4072, t5113, t671, t9348);
        let (t12823, t12832) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1731::<F>(t2363, t89, t12545, t12550, t12557, t12725, t12734, t12816, t1442, t1459, t1849, t2314, t2323, t2364, t3652, t3660, t4028, t4034, t4037, t4073, t574, t652, t672, t9348);
        let (t12835, t12841, t12850, t12854, t12858, t12860, t12861) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1732::<F>(t12813, t510, t1458, t3652, t4098, t751, t2752, t4303, t172, t4095, t763, t1472, t2517);
    (t12816, t12823, t12832, t12835, t12841, t12850, t12854, t12858, t12860, t12861)
}
