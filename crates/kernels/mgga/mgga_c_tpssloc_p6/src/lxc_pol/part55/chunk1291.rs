//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1291/1304 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1291<F: Float>(t32578: F, t45844: F, t12571: F, t117727: F, t117757: F, t117762: F, t119880: F, t119884: F, t119955: F, t119971: F, t119975: F, t119990: F, t1410: F, t27363: F, t31006: F, t31024: F, t31857: F, t31864: F, t31868: F, t32579: F, t32587: F, t32590: F, t33107: F, t33119: F, t33669: F, t34222: F, t7254: F, t8307: F, t8308: F, t8513: F, t8663: F, t8856: F) -> F {
    let t125837 = t45844 * t32578;
    let t125842 = t12571 * t32578;
    let t125855 = -F::cast_from(5.0_f64) / F::cast_from(24.0_f64) * t32579 * t119990 + F::cast_from(5.0_f64) / F::cast_from(72.0_f64) * t31857 * t34222 + F::cast_from(5.0_f64) / F::cast_from(72.0_f64) * t31868 * t34222 + F::cast_from(5.0_f64) / F::cast_from(72.0_f64) * t8663 * t8513 * t8307 * t27363 + F::cast_from(5.0_f64) / F::cast_from(72.0_f64) * t117762 * t33119 + F::cast_from(5.0_f64) / F::cast_from(72.0_f64) * t32590 * t119971 + F::cast_from(5.0_f64) / F::cast_from(72.0_f64) * t32590 * t119975 + F::cast_from(5.0_f64) / F::cast_from(144.0_f64) * t119955 * t8856 - F::cast_from(5.0_f64) / F::cast_from(24.0_f64) * t125837 * t31006 + F::cast_from(5.0_f64) / F::cast_from(72.0_f64) * t33669 * t32587 + F::cast_from(5.0_f64) / F::cast_from(72.0_f64) * t125842 * t31024 - F::cast_from(5.0_f64) / F::cast_from(24.0_f64) * t117757 * t33107 + F::cast_from(5.0_f64) / F::cast_from(6.0_f64) * t117727 * t119880 + F::cast_from(5.0_f64) / F::cast_from(6.0_f64) * t117727 * t119884 - F::cast_from(5.0_f64) / F::cast_from(18.0_f64) * t31864 * t8308 * t1410 * t7254;
    t125855
}
