//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2293/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2293<F: Float>(t1933: F, t23479: F, t99665: F, t1015: F, t23472: F, t28586: F, t17615: F, t6717: F, t17620: F, t23422: F, t28558: F, t28566: F, t5890: F, t5894: F, t5909: F, t6723: F, t83008: F, t88648: F, t88689: F, t88692: F) -> F {
    let t99774 = t1933 * t99665 * t23479;
    let t99779 = t23472 * t1015 * t28586;
    let t99785 = t6717 * t17615;
    let t99789 = t6717 * t17620;
    let t99793 = t88648 - F::cast_from(0.10093189023535097714e-3_f64) * t99774 + F::cast_from(0.80745512188280781712e-3_f64) * t6723 * t28566 - t88689 - t88692 + F::cast_from(0.10093189023535097714e-3_f64) * t99779 + t83008 * t5909 / F::new(1152.0) - t23422 * t5890 / F::new(108.0) + t99785 / F::new(864.0) - t23422 * t5894 / F::new(81.0) + t99789 / F::new(648.0) + F::cast_from(0.80745512188280781712e-3_f64) * t6723 * t28558;
    t99793
}
