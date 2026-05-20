//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta459 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1831;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1832;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta459<F: Float>(t1268: F, t1458: F, t19451: F, t20293: F, t20296: F, t20347: F, t4028: F, t5493: F, t7676: F, t19542: F, t19576: F, t1799: F, t6330: F, t15875: F, t15877: F, t15890: F, t15895: F, t19591: F, t11982: F, t11984: F, t193: F, t20077: F, t3918: F, t5160: F, t5161: F, t571: F, t6463: F, t9457: F, t9476: F, t9484: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t20350, t20354, t20355, t20356) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1831::<F>(t1268, t1458, t19451, t20293, t20296, t20347, t4028, t5493, t7676, t19542, t19576, t1799, t6330);
        let (t20360, t20361, t20365, t20366, t20370, t20371) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1832::<F>(t15875, t15877, t15890, t15895, t19591, t11982, t11984, t1799, t193, t20077, t20354, t20355, t20356, t3918, t5160, t5161, t571, t6463, t9457, t9476, t9484);
    (t20350, t20354, t20355, t20356, t20360, t20361, t20365, t20366, t20370, t20371)
}
