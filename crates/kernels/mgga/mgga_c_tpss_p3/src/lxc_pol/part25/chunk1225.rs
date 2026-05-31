//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1225/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1225<F: Float>(t1830: F, t3537: F, t6399: F, t645: F, t20319: F, t485: F, t1600: F, t5815: F, t1163: F, t6323: F, t1846: F, t19577: F, t2056: F, t3493: F, t3499: F, t5816: F, t5820: F, t5937: F, t6103: F, t624: F, t6243: F, t626: F, t6318: F, t6324: F) -> (F, F, F, F, F, F) {
    let t20368 = t1830 * t3537;
    let t20371 = t6399 * t645;
    let t20374 = t485 * t20319;
    let t20379 = t1600 * t5815;
    let t20386 = t1163 * t6323;
    let t20395 = t1846 * t19577 - F::cast_from(2.0_f64) * t20368 * t626 - F::cast_from(2.0_f64) * t20371 * t626 - F::cast_from(2.0_f64) * t20374 * t626 - F::cast_from(2.0_f64) * t20379 * t626 - F::cast_from(2.0_f64) * t20386 * t626 - F::cast_from(2.0_f64) * t2056 * t6318 - F::cast_from(2.0_f64) * t2056 * t6324 - F::cast_from(2.0_f64) * t3493 * t5820 - F::cast_from(2.0_f64) * t3499 * t6318 - F::cast_from(2.0_f64) * t3499 * t6324 - F::cast_from(2.0_f64) * t5816 * t6103 + t5937 * t6243 - t624 * t6399;
    (t20368, t20371, t20374, t20379, t20386, t20395)
}
