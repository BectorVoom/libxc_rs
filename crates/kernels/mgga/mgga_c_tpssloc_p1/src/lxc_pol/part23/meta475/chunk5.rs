//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1425/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1425<F: Float>(t1099: F, t1118: F, t78147: F, t78162: F, t78177: F, t78191: F, t44075: F, t44077: F, t78129: F, t63332: F, t63334: F, t63361: F, t71142: F, t71144: F, t71146: F, t71152: F, t77989: F, t77992: F, t77995: F, t78057: F) -> (F, F, F) {
    let t78196 = F::new(1.0) * t1099 * (t78147 + t78162 + t78177 + t78191) * t1118;
    let t78199 = F::cast_from(0.24955700379505800916e5_f64) * t44075 * t78129 * t44077;
    let t78211 = -F::cast_from(0.16481481481481481482e-1_f64) * t63332 + F::cast_from(0.24722222222222222222e-1_f64) * t63334 + F::cast_from(0.24722222222222222222e-1_f64) * t71142 - F::cast_from(0.74166666666666666668e-1_f64) * t71144 + F::cast_from(0.49444444444444444445e-1_f64) * t63361 - F::cast_from(0.22249999999999999999e0_f64) * t78057 - F::cast_from(0.13734567901234567901e-1_f64) * t71146 + F::new(0.2225e0) * t77989 + F::cast_from(0.92708333333333333333e-2_f64) * t77992 - F::cast_from(0.27469135802469135803e-1_f64) * t77995 - F::cast_from(0.74166666666666666668e-1_f64) * t71152;
    (t78196, t78199, t78211)
}
