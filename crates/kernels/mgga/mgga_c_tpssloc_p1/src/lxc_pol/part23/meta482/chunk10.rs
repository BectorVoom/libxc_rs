//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1461/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1461<F: Float>(t63332: F, t63334: F, t63361: F, t71142: F, t71144: F, t71146: F, t71152: F, t77989: F, t77992: F, t77995: F, t78057: F, t44320: F, t50834: F, t71154: F, t71156: F, t77998: F, t78002: F, t78005: F, t78033: F, t78037: F, t78041: F, t78045: F, t78049: F) -> (F, F) {
    let t78961 = -F::cast_from(0.3044148148148148148e-1_f64) * t63332 + F::cast_from(0.45662222222222222221e-1_f64) * t63334 + F::cast_from(0.4566222222222222222e-1_f64) * t71142 - F::cast_from(0.13698666666666666667e0_f64) * t71144 + F::cast_from(0.9132444444444444444e-1_f64) * t63361 - F::cast_from(0.41095999999999999999e0_f64) * t78057 - F::cast_from(0.25367901234567901233e-1_f64) * t71146 + F::cast_from(0.41096e0_f64) * t77989 + F::cast_from(0.17123333333333333333e-1_f64) * t77992 - F::cast_from(0.50735802469135802467e-1_f64) * t77995 - F::cast_from(0.13698666666666666667e0_f64) * t71152;
    let t78973 = -F::cast_from(0.22831111111111111111e-1_f64) * t71154 + F::cast_from(0.10274e0_f64) * t77998 + F::cast_from(0.9132444444444444444e-1_f64) * t71156 + F::cast_from(0.2283111111111111111e0_f64) * t78002 - F::cast_from(0.4566222222222222222e-1_f64) * t78033 - F::cast_from(0.71030123456790123454e-1_f64) * t50834 + t44320 + F::cast_from(0.11415555555555555555e0_f64) * t78037 - F::cast_from(0.41095999999999999998e0_f64) * t78041 + F::cast_from(0.61644e0_f64) * t78045 + F::cast_from(0.13698666666666666667e0_f64) * t78049 - F::cast_from(0.34246666666666666665e-1_f64) * t78005;
    (t78961, t78973)
}
