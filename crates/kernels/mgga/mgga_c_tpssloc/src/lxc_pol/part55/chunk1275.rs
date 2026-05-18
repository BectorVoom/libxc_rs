//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1275/1304 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1275<F: Float>(t1170: F, t2121: F, t34295: F, t118142: F, t118157: F, t118162: F, t1201: F, t1244: F, t1246: F, t1729: F, t2144: F, t24589: F, t24788: F, t27406: F, t27516: F, t27550: F, t32458: F, t32462: F, t3247: F, t32477: F, t34284: F, t34303: F, t3961: F, t4964: F, t5011: F, t8882: F, t8895: F) -> F {
    let t125378 = t2121 * t1170 * t34295;
    let t125383 = F::new(0.43864908449286038307e-1) * t27406 * t32462 - t118142 + t1244 * t8882 * t5011 * t1246 + F::new(0.54831135561607547883e-2) * t118157 + F::new(0.54831135561607547883e-2) * t24589 * t24788 * t34284 - F::new(0.10966227112321509577e-1) * t24589 * t27550 * t2144 * t3247 * t3961 - F::new(0.54831135561607547883e-2) * t118162 + t4964 * t8895 + t1729 * t32477 + t1201 * t34303 + F::new(0.54831135561607547883e-2) * t125378 + F::new(0.54831135561607547883e-2) * t24589 * t27516 * t32458;
    t125383
}
