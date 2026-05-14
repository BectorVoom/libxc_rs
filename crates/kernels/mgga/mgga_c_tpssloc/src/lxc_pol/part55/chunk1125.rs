//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1125/1154 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1125<F: Float>(t24574: F, t34310: F, t34247: F, t34323: F, t32496: F, t7999: F, t34349: F, t11605: F, t11606: F, t117897: F, t117910: F, t117924: F, t1238: F, t2121: F, t225: F, t24589: F, t24880: F, t27406: F, t27721: F, t27761: F, t27784: F, t32523: F, t32538: F, t32544: F, t32547: F, t34314: F, t3593: F, t462: F, t4945: F, t497: F, t5059: F, t5088: F, t7351: F, t8088: F, t8887: F, t8897: F, t94458: F) -> (F,) {
    let t125254 = t24574 * t34310;
    let t125266 = t24574 * t34247;
    let t125270 = t24574 * t34323;
    let t125276 = t7999 * t32496;
    let t125278 = t24574 * t34349;
    let t125280 = -2.0 * t24880 * t8088 + 4.0 * t3593 * t34314 - 0.54831135561607547883e-2 * t117897 + 0.54831135561607547883e-2 * t24589 * t94458 * t32523 - 6.0 * t1238 * t11606 * t8887 * t5088 - 0.18277045187202515961e-2 * t117910 - 6.0 * t4945 * t32538 - 0.18277045187202515961e-2 * t125254 + 0.43864908449286038307e-1 * t27406 * t32544 + 0.43864908449286038307e-1 * t27406 * t32547 + 0.16449340668482264365e-1 * t2121 * t462 * t27721 * t225 * t497 - 0.54831135561607547883e-2 * t117924 - 0.54831135561607547883e-2 * t125266 + 4.0 * t7351 * t27761 + 0.10966227112321509577e-1 * t125270 - 6.0 * t27784 * t11605 * t8897 * t5059 - 0.14621636149762012769e-1 * t125276 + 0.54831135561607547883e-2 * t125278;
    (t125280,)
}
