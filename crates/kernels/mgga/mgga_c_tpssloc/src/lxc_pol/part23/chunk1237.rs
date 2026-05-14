//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1237/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1237<F: Float>(t21101: F, t4483: F, t5726: F, t2842: F, t2844: F, t21373: F, t10702: F, t5694: F, t60378: F, t17492: F, t17947: F, t959: F, t4475: F, t68902: F, t17934: F, t5812: F) -> (F, F, F, F, F, F, F, F) {
    let t76997 = 0.4101607543286562663e4 * t4483 * t21101;
    let t76998 = t5726 * t5726;
    let t77001 = 0.48245938496077605201e2 * t2842 * t76998 * t2844;
    let t77003 = 0.14035736694323150897e2 * t4483 * t21373;
    let t77006 = 0.3103560775156404018e4 * t10702 * t60378 * t5694;
    let t77009 = 0.62337092780453269531e3 * t959 * t17947 * t17492;
    let t77012 = 0.69263436422725855036e2 * t959 * t68902 * t4475;
    let t77014 = 0.10389515463408878255e3 * t17934 * t5812;
    (t76997, t76998, t77001, t77003, t77006, t77009, t77012, t77014)
}
