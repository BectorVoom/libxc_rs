//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2546/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2546<F: Float>(t11190: F, t11191: F, t1671: F, t50826: F, t50919: F, t43727: F, t43729: F, t43748: F, t43750: F, t50828: F, t50832: F, t50834: F, t50897: F, t50900: F, t50903: F, t50905: F, t50907: F, t50912: F, t50917: F, t50921: F, t50926: F, t50931: F, t50934: F) -> (F, F) {
    let t51549 = F::cast_from(24.0_f64) * t11190 * t1671 * t11191;
    let t51550 = F::cast_from(0.23744444444444444444e-1_f64) * t50826;
    let t51565 = F::cast_from(0.15829629629629629629e-1_f64) * t50919;
    let t51570 = t51550 - F::cast_from(0.17808333333333333333e-1_f64) * t50828 + F::cast_from(0.17808333333333333333e-1_f64) * t50832 - F::cast_from(0.18467901234567901234e-1_f64) * t50834 + F::cast_from(0.11872222222222222222e-1_f64) * t43727 - F::cast_from(0.35616666666666666666e-1_f64) * t43729 - F::cast_from(0.15829629629629629629e-1_f64) * t43748 - F::cast_from(0.65956790123456790122e-2_f64) * t43750 - F::cast_from(0.11872222222222222222e-1_f64) * t50897 - F::cast_from(0.42739999999999999999e0_f64) * t50900 - F::cast_from(0.71233333333333333332e-1_f64) * t50903 - F::cast_from(0.35616666666666666666e-1_f64) * t50905 - F::cast_from(0.10685e0_f64) * t50907 + F::cast_from(0.5936111111111111111e-1_f64) * t50912 + F::cast_from(0.23744444444444444444e0_f64) * t50917 - t51565 - F::cast_from(0.19787037037037037036e-1_f64) * t50921 - F::cast_from(0.52765432098765432099e-1_f64) * t50926 + F::cast_from(0.10685e0_f64) * t50931 + F::cast_from(0.10685e0_f64) * t50934;
    (t51549, t51570)
}
