//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1279/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1279<F: Float>(t11832: F, t2127: F, t10401: F, t24739: F, t3610: F, t3624: F, t11553: F, t2121: F, t2123: F, t2157: F, t43706: F, t1453: F, t81439: F) -> (F, F, F, F, F, F) {
    let t86278 = F::new(5.0) / F::new(1296.0) * t2127 * t11832;
    let t86323 = t24739 * t10401;
    let t86324 = t3610 * t86323;
    let t86327 = t3624 * t86323;
    let t86451 = F::cast_from(0.30461741978670859935e-2_f64) * t2121 * t11553 * t2123;
    let t86524 = t2157 * t43706;
    let t86586 = t81439 * t1453;
    (t86278, t86324, t86327, t86451, t86524, t86586)
}
