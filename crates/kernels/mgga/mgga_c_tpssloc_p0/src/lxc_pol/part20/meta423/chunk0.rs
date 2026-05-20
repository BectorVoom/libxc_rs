//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 1838/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1838<F: Float>(t10817: F, t4359: F, t10655: F, t4400: F, t4396: F, t912: F, t2792: F, t1557: F, t2836: F, t2793: F, t4399: F, t10661: F) -> (F, F, F, F, F, F, F, F) {
    let t14376 = F::new(4.0) * t10817 * t4359;
    let t14378 = F::cast_from(0.32163958997385070134e2_f64) * t10655 * t4400;
    let t14379 = t4396 * t912;
    let t14381 = F::new(4.0) * t2792 * t14379;
    let t14382 = t1557 * t2836;
    let t14384 = F::new(2.0) * t2792 * t14382;
    let t14385 = t4399 * t2793;
    let t14387 = F::cast_from(0.96491876992155210402e2_f64) * t10661 * t14385;
    (t14376, t14378, t14379, t14381, t14382, t14384, t14385, t14387)
}
