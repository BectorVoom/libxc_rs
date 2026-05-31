//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2401/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2401<F: Float>(t17934: F, t4493: F, t21697: F, t3216: F, t17299: F, t4483: F, t14473: F, t5812: F, t41684: F, t47706: F, t47707: F, t47731: F, t59657: F, t68442: F, t68444: F, t68446: F, t68448: F, t68479: F, t68483: F, t68486: F, t68489: F, t68492: F, t68494: F, t68498: F, t68571: F, t68577: F, t68580: F, t68583: F) -> (F, F, F, F, F) {
    let t68710 = F::cast_from(0.17544670867903938621e1_f64) * t17934 * t4493;
    let t68711 = t21697 * t3216;
    let t68715 = F::cast_from(0.17544670867903938621e1_f64) * t4483 * t17299;
    let t68717 = F::cast_from(0.51947577317044391276e2_f64) * t14473 * t5812;
    let t68736 = F::cast_from(0.18541666666666666667e-1_f64) * t68442 + F::cast_from(0.30902777777777777778e-2_f64) * t68444 + F::cast_from(0.34336419753086419753e-2_f64) * t68446 - F::cast_from(0.12361111111111111111e-1_f64) * t68448 + t47706 - F::cast_from(0.82407407407407407407e-2_f64) * t47707 - t47731 + F::cast_from(0.96141975308641975307e-2_f64) * t41684 - F::cast_from(0.27469135802469135803e-1_f64) * t68479 - F::cast_from(0.22249999999999999999e0_f64) * t68483 + F::cast_from(0.11125e0_f64) * t68486 - F::cast_from(0.18541666666666666666e-1_f64) * t68489 - F::cast_from(0.18541666666666666666e-1_f64) * t68492 + F::cast_from(0.61805555555555555553e-2_f64) * t68494 - F::cast_from(0.18541666666666666667e-1_f64) * t68498 - F::cast_from(0.82407407407407407408e-2_f64) * t59657 - F::cast_from(0.92708333333333333333e-2_f64) * t68571 + F::cast_from(0.2225e0_f64) * t68577 - F::cast_from(0.166875e0_f64) * t68580 + F::cast_from(0.55625000000000000001e-1_f64) * t68583;
    (t68710, t68711, t68715, t68717, t68736)
}
