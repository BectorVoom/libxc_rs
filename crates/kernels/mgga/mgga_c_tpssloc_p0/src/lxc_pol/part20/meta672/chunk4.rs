//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2530/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2530<F: Float>(t3266: F, t51246: F, t11189: F, t1657: F, t11192: F, t50826: F, t43727: F, t43729: F, t43748: F, t43750: F, t50824: F, t50828: F, t50832: F, t50834: F, t50837: F, t50839: F) -> (F, F, F) {
    let t51248 = F::new(6.0) * t51246 * t3266;
    let t51249 = t1657 * t11189;
    let t51251 = F::cast_from(0.96491876992155210402e2_f64) * t51249 * t11192;
    let t51257 = F::cast_from(0.68863333333333333332e0_f64) * t50826;
    let t51267 = F::new(0.187551e1) * t50824 + t51257 - F::cast_from(0.51647499999999999999e0_f64) * t50828 + F::new(0.516475e0) * t50832 - F::cast_from(0.53560370370370370369e0_f64) * t50834 + F::cast_from(0.794188125e1_f64) * t50837 - F::cast_from(0.473371875e0_f64) * t50839 + F::cast_from(0.34431666666666666666e0_f64) * t43727 - F::new(0.103295e1) * t43729 - F::cast_from(0.45908888888888888888e0_f64) * t43748 - F::cast_from(0.19128703703703703703e0_f64) * t43750;
    (t51248, t51251, t51267)
}
