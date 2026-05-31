//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1419/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1419<F: Float>(t1661: F, t71445: F, t71448: F, t18754: F, t5999: F, t18746: F, t43895: F, t63361: F, t78057: F, t78084: F, t78087: F, t78090: F, t78093: F, t78095: F, t78097: F, t78100: F) -> (F, F, F, F, F) {
    let t78103 = t71445 * t1661;
    let t78105 = t71448 * t1661;
    let t78107 = t18754 * t5999;
    let t78109 = t18746 * t5999;
    let t78112 = -F::cast_from(0.11038e0_f64) * t78084 - F::cast_from(0.99342e0_f64) * t78087 + F::cast_from(0.66228e0_f64) * t78090 + F::cast_from(0.298026e1_f64) * t78093 + F::cast_from(0.258925e1_f64) * t78095 + t43895 + F::cast_from(0.247573125e0_f64) * t78097 + F::cast_from(0.22076e0_f64) * t78100 + F::cast_from(0.16102666666666666667e1_f64) * t63361 + F::cast_from(0.3300975e0_f64) * t78103 - F::cast_from(0.51785e1_f64) * t78105 + F::cast_from(0.11651625e2_f64) * t78107 - F::cast_from(0.247573125e0_f64) * t78109 - F::cast_from(0.72462e1_f64) * t78057;
    (t78103, t78105, t78107, t78109, t78112)
}
