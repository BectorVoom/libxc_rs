//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1115/1475 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1115<F: Float>(t5: F, t26938: F, t26964: F, t112: F, t24990: F, t7170: F, t24432: F, t25988: F, t2035: F, t671: F) -> (F, F, F, F, F) {
    let t7 = piecewise3::<f64>(F::new(0.0) < t5, t5, -t5);
    let t8 = -t7 <= -F::new(0.999999999999e0);
    let t26966 = piecewise3::<f64>(t8, F::new(0.0), t26938 + t26964);
    let t26967 = t26966 * t112;
    let t26969 = t7170 * t24990;
    let t26974 = t24432 * t25988;
    let t26977 = t2035 * t671;
    (t26966, t26967, t26969, t26974, t26977)
}
