//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1232/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1232<F: Float>(t4017: F, t79: F, t8513: F, t31003: F, t45844: F, t12571: F, t31016: F, t4021: F, t8307: F, t113845: F, t113848: F, t113851: F, t119944: F, t119948: F, t119952: F, t119955: F, t119965: F, t119971: F, t2240: F, t31004: F, t31006: F, t31017: F, t31019: F, t31022: F, t31024: F, t33107: F, t33115: F, t33119: F, t6504: F, t8301: F, t8309: F) -> F {
    let t119975 = t8513 * t79 * t4017;
    let t119978 = t45844 * t31003;
    let t119981 = t12571 * t31016;
    let t119984 = t12571 * t31003;
    let t119990 = t8513 * t8307 * t4021;
    let t119993 = F::cast_from(5.0_f64) / F::cast_from(36.0_f64) * t31017 * t119944 - F::cast_from(5.0_f64) / F::cast_from(12.0_f64) * t31004 * t119948 + F::cast_from(5.0_f64) / F::cast_from(36.0_f64) * t31017 * t119952 + F::cast_from(5.0_f64) / F::cast_from(144.0_f64) * t119955 * t8309 + F::cast_from(5.0_f64) / F::cast_from(72.0_f64) * t113848 * t33115 + F::cast_from(5.0_f64) / F::cast_from(72.0_f64) * t2240 * t8301 * t6504 * t33115 + F::cast_from(5.0_f64) / F::cast_from(72.0_f64) * t31017 * t119965 + F::cast_from(5.0_f64) / F::cast_from(72.0_f64) * t113851 * t33119 + F::cast_from(5.0_f64) / F::cast_from(72.0_f64) * t31022 * t119971 + F::cast_from(5.0_f64) / F::cast_from(72.0_f64) * t31022 * t119975 - F::cast_from(5.0_f64) / F::cast_from(24.0_f64) * t119978 * t31006 + F::cast_from(5.0_f64) / F::cast_from(72.0_f64) * t119981 * t31019 + F::cast_from(5.0_f64) / F::cast_from(72.0_f64) * t119984 * t31024 - F::cast_from(5.0_f64) / F::cast_from(24.0_f64) * t113845 * t33107 - F::cast_from(5.0_f64) / F::cast_from(24.0_f64) * t31004 * t119990;
    t119993
}
