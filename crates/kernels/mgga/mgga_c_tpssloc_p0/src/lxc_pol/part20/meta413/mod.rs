//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta413 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1818;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1819;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta413<F: Float>(t14032: F, t3071: F, t1616: F, t2771: F, t10408: F, t1539: F, t3121: F, t3048: F, t4571: F, t10390: F, t10891: F, t10904: F, t10937: F, t10957: F, t14006: F, t14009: F, t14012: F, t14015: F, t14018: F, t14027: F, t1622: F, t3070: F, t3098: F, t4575: F, t4596: F, t4600: F, t4644: F, t973: F, t3109: F, t4630: F, t4650: F, t884: F, t10436: F, t10441: F, t10449: F, t10455: F, t10460: F, t10490: F, t10496: F, t10504: F, t10511: F, t10517: F, t10863: F, t10866: F, t10871: F, t1618: F, t4636: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t14033, t14036, t14037, t14040, t14041, t14050) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1818::<F>(t14032, t3071, t1616, t2771, t10408, t1539, t3121, t3048, t4571, t10390, t10891, t10904, t10937, t10957, t14006, t14009, t14012, t14015, t14018, t14027, t1622, t3070, t3098, t4575, t4596, t4600, t4644, t973);
        let (t14068, t14069, t14074) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1819::<F>(t3109, t4630, t4650, t884, t3071, t10436, t10441, t10449, t10455, t10460, t10490, t10496, t10504, t10511, t10517, t10863, t10866, t10871, t1618, t1622, t3048, t3070, t4636);
    (t14033, t14036, t14037, t14040, t14041, t14050, t14068, t14069, t14074)
}
