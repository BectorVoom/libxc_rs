//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1280/1304 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1280<F: Float>(t27650: F, t8875: F, t27680: F, t1222: F, t34272: F, t34266: F, t118017: F, t118019: F, t1201: F, t24650: F, t27684: F, t27691: F, t27711: F, t32432: F, t32433: F, t34263: F, t34271: F, t488: F, t4964: F, t7326: F, t8878: F) -> F {
    let t125483 = t27650 * t8875;
    let t125485 = t27680 * t8875;
    let t125488 = t34272 * t1222;
    let t125492 = t34266 * t1222;
    let t125508 = -F::cast_from(0.40372756094140390856e-3_f64) * t125483 - F::cast_from(0.32298204875312312685e-2_f64) * t125485 + F::cast_from(0.40372756094140390856e-3_f64) * t118017 - t125488 / F::new(432.0) - F::cast_from(0.40372756094140390856e-3_f64) * t27684 * t32433 + t125492 / F::new(2304.0) - t1201 * t34271 * t488 / F::new(288.0) - F::cast_from(0.32298204875312312685e-2_f64) * t27711 * t32433 + F::cast_from(0.40372756094140390856e-3_f64) * t7326 * t32432 * t27691 - F::cast_from(0.40372756094140390856e-3_f64) * t24650 * t34263 + t4964 * t8878 * t488 / F::new(1536.0) - F::cast_from(0.40372756094140390856e-3_f64) * t118019;
    t125508
}
