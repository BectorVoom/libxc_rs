//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2522/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2522<F: Float>(t11131: F, t4869: F, t11427: F, t14850: F, t50826: F, t43727: F, t43729: F, t43748: F, t43750: F, t50824: F, t50828: F, t50832: F, t50834: F, t50837: F, t50839: F) -> (F, F, F) {
    let t51131 = F::cast_from(0.35089341735807877242e1_f64) * t4869 * t11131;
    let t51133 = F::new(6.0) * t14850 * t11427;
    let t51137 = F::cast_from(0.39862222222222222223e0_f64) * t50826;
    let t51147 = F::new(0.147882e1) * t50824 + t51137 - F::cast_from(0.29896666666666666667e0_f64) * t50828 + F::cast_from(0.29896666666666666667e0_f64) * t50832 - F::cast_from(0.31003950617283950619e0_f64) * t50834 + F::cast_from(0.427258125e1_f64) * t50837 - F::cast_from(0.230371875e0_f64) * t50839 + F::cast_from(0.19931111111111111112e0_f64) * t43727 - F::cast_from(0.59793333333333333333e0_f64) * t43729 - F::cast_from(0.26574814814814814816e0_f64) * t43748 - F::cast_from(0.11072839506172839506e0_f64) * t43750;
    (t51131, t51133, t51147)
}
