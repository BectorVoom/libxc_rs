//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1292/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1292<F: Float>(t1147: F, t1156: F, t1164: F, t78114: F, t18915: F, t6098: F, t22222: F, t4869: F, t6085: F, t6105: F, t4861: F, t72062: F, t5988: F, t11277: F, t43969: F, t50834: F, t71335: F, t71337: F, t77959: F, t77963: F, t77967: F, t77971: F, t77975: F, t77979: F, t77983: F, t77989: F, t77992: F, t77995: F, t77998: F) -> (F, F, F, F, F, F, F, F) {
    let t78118 = 0.5848223622634646207e0 * t1164 * t1147 * t78114 * t1156;
    let t78120 = 0.70178683471615754484e1 * t18915 * t6098;
    let t78122 = 0.14035736694323150897e2 * t4869 * t22222;
    let t78125 = 0.21053605041484726346e2 * t1164 * t6105 * t6085;
    let t78128 = 0.69263436422725855036e2 * t1164 * t72062 * t4861;
    let t78129 = t5988 * t5988;
    let t78132 = 0.62071215503128080361e4 * t43969 * t78129 * t11277;
    let t78147 = 0.43816888888888888889e0 * t77959 - 0.85199506172839506175e-1 * t77963 - 0.82156666666666666668e-1 * t77967 + 0.49293999999999999999e0 * t77971 - 0.98587999999999999998e0 * t77975 + 0.197176e1 * t77979 + 0.82156666666666666667e-1 * t77983 + 0.21908444444444444444e0 * t71335 - 0.13145066666666666666e1 * t71337 - 0.12401580246913580247e1 * t50834 + 0.71752e1 * t77989 + 0.29896666666666666667e0 * t77992 - 0.88582716049382716048e0 * t77995 + 0.17938e1 * t77998;
    (t78118, t78120, t78122, t78125, t78128, t78129, t78132, t78147)
}
