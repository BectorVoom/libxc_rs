//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 1831/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1831<F: Float>(t1268: F, t1458: F, t19451: F, t20293: F, t20296: F, t20347: F, t4028: F, t5493: F, t7676: F, t19542: F, t19576: F, t1799: F, t6330: F) -> (F, F, F, F) {
    let t20350 = F::new(2.0) * t1268 * t20347 + F::new(6.0) * t1458 * t19451 + F::new(6.0) * t4028 * t5493 + F::new(6.0) * t5493 * t7676 + t20293 + F::new(6.0) * t20296;
    let t20354 = F::cast_from(0.54934341918019635162e-3_f64) * t19542;
    let t20355 = F::new(3.0) * t19576;
    let t20356 = t6330 * t1799;
    (t20350, t20354, t20355, t20356)
}
