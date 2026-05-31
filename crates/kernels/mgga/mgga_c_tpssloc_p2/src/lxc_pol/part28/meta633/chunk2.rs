//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 2003/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk2003<F: Float>(t1877: F, t23781: F, t23796: F, t23810: F, t24191: F, t24344: F, t2522: F, t25901: F, t26744: F, t26756: F, t4314: F, t6848: F, t7110: F, t7114: F, t7656: F, t7845: F, t84791: F, t89837: F, t89840: F, t89846: F, t89872: F, t89907: F, t89931: F, t89941: F, t89982: F, t89993: F, t92276: F) -> F {
    let t93246 = F::cast_from(3.0_f64) * t4314 * t7845 * t23781 + t26756 * t89872 + F::cast_from(2.0_f64) * t26756 * t89846 - F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t24191 * t89840 - t1877 * t26744 * t23810 - t1877 * t7114 * t89941 + t1877 * t24344 * t89982 - t1877 * t92276 * t6848 - t1877 * t7114 * t89907 / F::cast_from(2.0_f64) - F::cast_from(3.0_f64) * t24191 * t89931 - F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t24191 * t89837 + F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t2522 * t7845 * t23796 - F::cast_from(3.0_f64) * t24191 * t89993 - t1877 * t84791 * t7656 / F::cast_from(2.0_f64) + F::cast_from(3.0_f64) * t2522 * t7110 * t25901;
    t93246
}
