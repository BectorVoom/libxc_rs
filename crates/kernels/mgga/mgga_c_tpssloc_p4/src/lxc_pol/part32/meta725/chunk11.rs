//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2340/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2340<F: Float>(t24574: F, t29813: F, t1238: F, t14980: F, t1760: F, t19213: F, t19219: F, t19225: F, t24589: F, t24601: F, t24602: F, t24615: F, t27389: F, t27406: F, t27741: F, t27784: F, t27785: F, t27830: F, t3598: F, t3966: F, t5060: F, t7283: F, t7300: F, t8002: F, t8061: F, t94358: F, t94680: F, t95863: F, t95866: F, t95884: F, t95889: F) -> F {
    let t104609 = t24574 * t29813;
    let t104631 = F::new(4.0) * t14980 * t8061 + F::new(4.0) * t1238 * t3598 * t27741 * t1760 + t95863 + t95866 + F::new(4.0) * t27830 * t5060 - F::cast_from(0.54831135561607547884e-2_f64) * t7283 * t94680 * t8002 - F::cast_from(0.91385225936012579807e-3_f64) * t104609 - F::new(6.0) * t27784 * t27785 * t19219 + F::cast_from(0.3289868133696452873e-1_f64) * t7283 * t7300 * t24615 * t19213 + t95889 + F::cast_from(0.54831135561607547884e-2_f64) * t24589 * t24601 * t24602 * t3966 * t1760 + F::cast_from(0.54831135561607547884e-2_f64) * t24589 * t94358 * t8002 + F::cast_from(0.14621636149762012769e-1_f64) * t27406 * t27389 + F::new(24.0) * t27784 * t95884 * t19225;
    t104631
}
