//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2289/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2289<F: Float>(t24574: F, t27779: F, t8015: F, t85660: F, t27826: F, t11606: F, t11925: F, t1238: F, t12652: F, t15771: F, t15789: F, t2121: F, t2155: F, t225: F, t24564: F, t24591: F, t24601: F, t27406: F, t27549: F, t27774: F, t27784: F, t27785: F, t27792: F, t3598: F, t3599: F, t3600: F, t462: F, t497: F, t5088: F, t53658: F, t7391: F, t8087: F, t8088: F, t86426: F, t94395: F) -> F {
    let t94700 = F::cast_from(0.18277045187202515961e-2_f64) * t24574 * t27779;
    let t94701 = t85660 * t8015;
    let t94710 = F::cast_from(0.54831135561607547884e-2_f64) * t24574 * t27826;
    let t94734 = -t94700 + F::cast_from(0.18277045187202515961e-2_f64) * t94701 + F::cast_from(0.54831135561607547884e-2_f64) * t86426 - F::new(12.0) * t27784 * t27785 * t15789 + F::cast_from(0.21932454224643019153e-1_f64) * t27406 * t24564 - t94710 - t53658 * t2155 + F::cast_from(0.82246703342411321825e-2_f64) * t2121 * t462 * t15771 * t225 * t497 + F::new(4.0) * t1238 * t3598 * t7391 * t5088 - F::new(6.0) * t1238 * t11606 * t8087 * t3599 + F::new(2.0) * t27792 * t3600 - t11925 * t8088 - F::cast_from(0.14621636149762012769e-1_f64) * t94395 * t24591 + F::cast_from(0.73108180748810063846e-2_f64) * t27549 * t24601 * t27774 * t12652;
    t94734
}
