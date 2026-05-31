//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2300/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2300<F: Float>(t24826: F, t27540: F, t1235: F, t14706: F, t24812: F, t24813: F, t27478: F, t27489: F, t27491: F, t27724: F, t3477: F, t3502: F, t3604: F, t3610: F, t4978: F, t5068: F, t7283: F, t7362: F, t7363: F, t8077: F, t85941: F, t85943: F, t85945: F, t85952: F, t85955: F, t94986: F) -> F {
    let t95069 = F::cast_from(0.54831135561607547884e-2_f64) * t24826 * t27540;
    let t95087 = F::cast_from(0.3289868133696452873e-1_f64) * t24812 * t24813 * t3502 * t1235 * t27491 + F::cast_from(0.3289868133696452873e-1_f64) * t24812 * t27489 * t94986 * t4978 - t95069 + F::cast_from(4.0_f64) * t3610 * t27724 * t5068 - F::cast_from(0.36554090374405031922e-2_f64) * t85941 - F::cast_from(0.91385225936012579807e-3_f64) * t85943 - F::cast_from(0.18277045187202515961e-2_f64) * t85945 - F::cast_from(0.27415567780803773942e-2_f64) * t7283 * t7362 * t7363 * t14706 + F::cast_from(0.12184696791468343974e-2_f64) * t85952 - F::cast_from(0.82246703342411321825e-2_f64) * t7283 * t3477 * t8077 + F::cast_from(0.27415567780803773942e-2_f64) * t85955 + F::cast_from(2.0_f64) * t3604 * t27478;
    t95087
}
