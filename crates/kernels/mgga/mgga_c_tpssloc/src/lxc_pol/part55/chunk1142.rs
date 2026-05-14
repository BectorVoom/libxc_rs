//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1142/1154 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1142<F: Float>(t12571: F, t32578: F, t117727: F, t117757: F, t117762: F, t119880: F, t119884: F, t119955: F, t119971: F, t119975: F, t119990: F, t125837: F, t1410: F, t27363: F, t31006: F, t31024: F, t31857: F, t31864: F, t31868: F, t32579: F, t32587: F, t32590: F, t33107: F, t33119: F, t33669: F, t34222: F, t7254: F, t8307: F, t8308: F, t8513: F, t8663: F, t8856: F) -> (F,) {
    let t125842 = t12571 * t32578;
    let t125855 = -5.0 / 24.0 * t32579 * t119990 + 5.0 / 72.0 * t31857 * t34222 + 5.0 / 72.0 * t31868 * t34222 + 5.0 / 72.0 * t8663 * t8513 * t8307 * t27363 + 5.0 / 72.0 * t117762 * t33119 + 5.0 / 72.0 * t32590 * t119971 + 5.0 / 72.0 * t32590 * t119975 + 5.0 / 144.0 * t119955 * t8856 - 5.0 / 24.0 * t125837 * t31006 + 5.0 / 72.0 * t33669 * t32587 + 5.0 / 72.0 * t125842 * t31024 - 5.0 / 24.0 * t117757 * t33107 + 5.0 / 6.0 * t117727 * t119880 + 5.0 / 6.0 * t117727 * t119884 - 5.0 / 18.0 * t31864 * t8308 * t1410 * t7254;
    (t125855,)
}
