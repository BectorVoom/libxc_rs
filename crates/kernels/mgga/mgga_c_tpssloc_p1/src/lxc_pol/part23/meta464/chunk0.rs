//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1358/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1358<F: Float>(t21101: F, t4483: F, t5726: F, t2842: F, t2844: F, t21373: F, t10702: F, t5694: F, t60378: F, t17492: F, t17947: F, t959: F) -> (F, F, F, F, F, F) {
    let t76997 = F::cast_from(0.4101607543286562663e4_f64) * t4483 * t21101;
    let t76998 = t5726 * t5726;
    let t77001 = F::cast_from(0.48245938496077605201e2_f64) * t2842 * t76998 * t2844;
    let t77003 = F::cast_from(0.14035736694323150897e2_f64) * t4483 * t21373;
    let t77006 = F::cast_from(0.3103560775156404018e4_f64) * t10702 * t60378 * t5694;
    let t77009 = F::cast_from(0.62337092780453269531e3_f64) * t959 * t17947 * t17492;
    (t76997, t76998, t77001, t77003, t77006, t77009)
}
