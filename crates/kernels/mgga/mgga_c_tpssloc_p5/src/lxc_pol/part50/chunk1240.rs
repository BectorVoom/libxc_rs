//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1240/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1240<F: Float>(t19577: F, t22574: F, t36363: F, t24995: F, t37589: F, t5308: F, t1983: F, t31221: F, t5161: F, t120063: F, t120064: F, t120067: F, t120069: F, t120072: F, t120075: F, t120078: F, t120079: F, t120083: F, t120085: F, t120086: F, t120088: F, t31055: F, t31057: F, t31060: F) -> F {
    let t120092 = F::cast_from(3.0_f64) * t22574 * t36363 * t19577;
    let t120095 = F::cast_from(6.0_f64) * t24995 * t37589 * t5308;
    let t120097 = t1983 * t31221 * t5161;
    let t120098 = -t31055 - t31057 - t31060 - t120063 - F::cast_from(4.0_f64) * t120064 - t120067 - t120069 + F::cast_from(2.0_f64) * t120072 - t120075 + t120078 + F::cast_from(6.0_f64) * t120079 - t120083 + t120085 + F::cast_from(6.0_f64) * t120086 + F::cast_from(6.0_f64) * t120088 - t120092 + t120095 - t120097;
    t120098
}
