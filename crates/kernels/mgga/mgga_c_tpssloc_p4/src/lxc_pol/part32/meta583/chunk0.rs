//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 1966/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1966<F: Float>(t29726: F, t462: F, t1409: F, t1734: F, t7376: F, t24851: F, t1653: F, t27460: F, t7362: F, t6260: F, t7375: F, t1244: F, t2121: F, t2149: F, t24773: F, t24849: F, t27406: F, t27451: F, t27556: F, t29678: F, t29702: F, t29705: F, t29709: F, t29712: F, t29716: F, t29720: F, t29723: F, t3610: F, t3624: F, t5064: F, t7283: F, t7373: F, t8070: F, t8083: F) -> (F, F, F, F, F, F, F, F) {
    let t29727 = t462 * t29726;
    let t29734 = t1409 * t1734;
    let t29735 = t29734 * t7376;
    let t29736 = t24851 * t29735;
    let t29740 = t27460 * t1653;
    let t29741 = t7362 * t29740;
    let t29744 = t6260 * t7376;
    let t29745 = t7375 * t29744;
    let t29748 = -F::cast_from(0.18277045187202515961e-2_f64) * t27451 - t24773 - F::cast_from(0.82246703342411321825e-2_f64) * t7283 * t29702 - F::cast_from(0.82246703342411321825e-2_f64) * t7283 * t29705 - t3624 * t29709 + t1244 * t29712 + F::cast_from(2.0_f64) * t5064 * t8083 - F::cast_from(0.16449340668482264365e-1_f64) * t7373 * t29716 + F::cast_from(2.0_f64) * t1244 * t29720 + F::cast_from(2.0_f64) * t3610 * t29723 + F::cast_from(0.82246703342411321825e-2_f64) * t2121 * t29727 + F::cast_from(0.80418998823691070228e-1_f64) * t29678 * t2149 + F::cast_from(0.43864908449286038306e-1_f64) * t27406 * t8070 - F::cast_from(0.54831135561607547884e-2_f64) * t24849 * t29736 + F::cast_from(0.54831135561607547884e-2_f64) * t27556 - F::cast_from(0.54831135561607547884e-2_f64) * t7283 * t29741 + F::cast_from(0.82246703342411321825e-2_f64) * t7373 * t29745;
    (t29734, t29735, t29736, t29740, t29741, t29744, t29745, t29748)
}
