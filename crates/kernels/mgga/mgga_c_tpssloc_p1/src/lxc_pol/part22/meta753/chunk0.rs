//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2529/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2529<F: Float>(t18280: F, t4869: F, t51819: F, t63519: F, t71115: F, t1117: F, t11275: F, t15067: F, t6020: F, t18271: F, t18283: F, t18915: F, t4884: F) -> (F, F, F, F, F, F) {
    let t71238 = F::cast_from(0.31168546390226634765e3_f64) * t4869 * t18280;
    let t71241 = F::cast_from(0.30762056574649219973e4_f64) * t51819 * t63519 * t71115;
    let t71245 = F::cast_from(0.1551780387578202009e4_f64) * t11275 * t6020 * t15067 * t1117;
    let t71247 = F::cast_from(0.10526802520742363173e2_f64) * t4869 * t18271;
    let t71249 = F::cast_from(0.10389515463408878255e3_f64) * t4869 * t18283;
    let t71251 = F::cast_from(0.51947577317044391276e2_f64) * t18915 * t4884;
    (t71238, t71241, t71245, t71247, t71249, t71251)
}
