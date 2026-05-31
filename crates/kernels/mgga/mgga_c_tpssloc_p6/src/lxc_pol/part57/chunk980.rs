//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 980/1049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk980<F: Float>(t127630: F, t8657: F, t127601: F, t127603: F, t127606: F, t127669: F, t127671: F, t127673: F, t127677: F, t127679: F, t127681: F, t127684: F, t127686: F, t127688: F, t26523: F, t31795: F, t5493: F, t7801: F, t7956: F, t8508: F, t86647: F) -> F {
    let t127690 = F::cast_from(54.0_f64) * t127630 * t8657;
    let t127695 = t8508 + t127669 + t127671 + t127673 + t127601 + t127603 + F::cast_from(54.0_f64) * t86647 * t7956 + t127677 + t127679 + t127681 + t127684 + t127606 + t127686 + t127688 + t127690 + F::cast_from(27.0_f64) * t26523 * t7801 + F::cast_from(0.135e2_f64) * t31795 * t5493;
    t127695
}
