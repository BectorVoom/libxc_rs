//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2417/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2417<F: Float>(t21101: F, t2940: F, t1581: F, t49541: F, t68888: F, t41684: F, t48688: F, t48689: F, t48698: F, t59657: F, t68442: F, t68444: F, t68446: F, t68448: F, t68479: F, t68483: F, t68486: F, t68489: F, t68492: F, t68494: F, t68498: F, t68571: F, t68577: F, t68580: F, t68583: F) -> (F, F, F) {
    let t68951 = F::cast_from(0.10254018858216406658e4_f64) * t2940 * t21101;
    let t68954 = F::cast_from(0.10526802520742363173e2_f64) * t49541 * t1581 * t68888;
    let t68972 = F::cast_from(0.35616666666666666667e-1_f64) * t68442 + F::cast_from(0.5936111111111111111e-2_f64) * t68444 + F::cast_from(0.65956790123456790123e-2_f64) * t68446 - F::cast_from(0.23744444444444444444e-1_f64) * t68448 + t48688 - t48689 - t48698 + F::cast_from(0.18467901234567901234e-1_f64) * t41684 - F::cast_from(0.52765432098765432099e-1_f64) * t68479 - F::cast_from(0.42739999999999999999e0_f64) * t68483 + F::new(0.2137e0) * t68486 - F::cast_from(0.35616666666666666666e-1_f64) * t68489 - F::cast_from(0.35616666666666666666e-1_f64) * t68492 + F::cast_from(0.11872222222222222222e-1_f64) * t68494 - F::cast_from(0.35616666666666666667e-1_f64) * t68498 - F::cast_from(0.15829629629629629629e-1_f64) * t59657 - F::cast_from(0.17808333333333333333e-1_f64) * t68571 + F::new(0.4274e0) * t68577 - F::new(0.32055e0) * t68580 + F::new(0.10685e0) * t68583;
    (t68951, t68954, t68972)
}
