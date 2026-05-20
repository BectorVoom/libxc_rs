//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2087/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2087<F: Float>(t1015: F, t23472: F, t23503: F, t10423: F, t23419: F, t23418: F, t3180: F, t10401: F, t23417: F, t3186: F, t3158: F, t6712: F) -> (F, F, F, F, F, F) {
    let t82996 = t23472 * t1015 * t23503;
    let t83004 = t23419 * t10423;
    let t83008 = t3180 * t23418;
    let t83015 = t23417 * t10401;
    let t83016 = t3186 * t83015;
    let t83025 = t6712 * t3158;
    (t82996, t83004, t83008, t83015, t83016, t83025)
}
