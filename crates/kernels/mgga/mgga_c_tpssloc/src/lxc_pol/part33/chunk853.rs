//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 853/1415 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk853<F: Float>(t12384: F, t1336: F, t1995: F, t67: F, t246: F, t3700: F, t570: F, t1406: F, t2239: F, t1454: F, t2281: F, t1472: F, t2517: F) -> (F, F, F, F, F, F) {
    let t12385 = t1336 * t12384;
    let t12418 = t1995 * t67;
    let t12419 = t12418 * t246;
    let t12461 = F::new(1.0) / t3700 / t570;
    let t12571 = t1406 * t2239;
    let t12747 = t2281 * t1454;
    let t12861 = t1472 * t2517;
    (t12385, t12419, t12461, t12571, t12747, t12861)
}
