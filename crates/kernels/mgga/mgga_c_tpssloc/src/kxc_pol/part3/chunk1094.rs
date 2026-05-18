//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 3 (v3rho3_1) CSE chunk 1094/1255 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part3_v3rho3_1_chunk1094<F: Float>(t13566: F, t13602: F, t10556: F, t10558: F, t10560: F, t10562: F, t10636: F, t13563: F, t13569: F, t13572: F, t13575: F, t13578: F, t13581: F, t13584: F, t13587: F, t13598: F, t13613: F) -> F {
    let t14245 = F::new(0.23744444444444444444e-1) * t13566;
    let t14246 = F::new(0.11872222222222222222e-1) * t13602;
    let t14255 = -t10636 - F::new(0.15829629629629629629e-1) * t10556 + F::new(0.39574074074074074073e-2) * t10558 - F::new(0.11872222222222222222e-1) * t10560 + F::new(0.5936111111111111111e-2) * t10562 - F::new(0.79148148148148148146e-2) * t13598 + F::new(0.79148148148148148146e-2) * t13563 - t14245 + t14246 - F::new(0.19787037037037037037e-1) * t13569 + F::new(0.71233333333333333332e-1) * t13572 - F::new(0.23744444444444444444e-1) * t13575 - F::new(0.11872222222222222222e-1) * t13578 - F::new(0.10685e0) * t13581 + F::new(0.71233333333333333332e-1) * t13584 + F::new(0.35616666666666666666e-1) * t13587 - F::new(0.17808333333333333333e-1) * t13613;
    t14255
}
