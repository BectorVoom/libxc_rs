//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1134/1154 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1134<F: Float>(t1090: F, t117809: F, t117934: F, t118038: F, t1238: F, t1241: F, t125295: F, t125306: F, t125311: F, t125313: F, t125358: F, t125383: F, t125530: F, t125568: F, t1720: F, t2128: F, t24567: F, t24589: F, t24601: F, t27445: F, t27786: F, t27830: F, t32451: F, t32489: F, t32493: F, t32514: F, t32538: F, t34250: F, t3598: F, t4733: F, t4940: F, t4945: F, t498: F, t5055: F, t5088: F, t7283: F, t7287: F, t7392: F, t8002: F, t8882: F, t8897: F) -> (F,) {
    let t125580 = 0.54831135561607547883e-2 * t24589 * t118038 * t8002 - 0.10966227112321509577e-1 * t24589 * t117809 * t27445 + 0.18277045187202515961e-2 * t117934 + 0.54831135561607547883e-2 * t24589 * t24601 * t32514 * t4733 - 0.9869604401089358619e-1 * t2128 * t24601 * t27786 + 0.54831135561607547883e-2 * t24589 * t24601 * t125295 * t1090 + 2.0 * t1238 * t3598 * t8897 * t5088 - 2.0 * t27830 * t7392 - 0.54831135561607547883e-2 * t7283 * t125306 * t7287 + 0.54831135561607547883e-2 * t125311 - 0.54831135561607547883e-2 * t125313 + 4.0 * t5055 * t32493 - 6.0 * t5055 * t32538 + 2.0 * t4945 * t32489 - t1238 * t1241 * (t125358 + t125383 + t125530 + t125568) + t4940 * t8882 * t498 + t1720 * t32451 * t498 - 0.16449340668482264365e-1 * t7283 * t24567 * t34250;
    (t125580,)
}
