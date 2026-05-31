//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2625/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2625<F: Float>(t1174: F, t22059: F, t3431: F, t50846: F, t63841: F, t63843: F, t63845: F, t63886: F, t63888: F, t63893: F, t63911: F, t71333: F, t71335: F, t71337: F, t71400: F, t71406: F, t71408: F, t71411: F, t71414: F, t71417: F, t71420: F, t71423: F, t71426: F) -> (F, F) {
    let t73330 = t1174 * t3431 * t22059;
    let t73355 = t71333 / F::cast_from(18.0_f64) - t71335 / F::cast_from(9.0_f64) + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t71337 + F::cast_from(4.0_f64) / F::cast_from(27.0_f64) * t63841 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t63843 - t63845 / F::cast_from(9.0_f64) + t63886 / F::cast_from(3.0_f64) + F::cast_from(5.0_f64) / F::cast_from(27.0_f64) * t63888 - F::cast_from(10.0_f64) / F::cast_from(9.0_f64) * t63893 + F::cast_from(14.0_f64) / F::cast_from(81.0_f64) * t71400 - F::cast_from(5.0_f64) / F::cast_from(9.0_f64) * t63911 - t71406 / F::cast_from(6.0_f64) + t71408 / F::cast_from(9.0_f64) - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t71411 - F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t71414 + t71417 + F::cast_from(2.0_f64) * t71420 - F::cast_from(3.0_f64) * t71423 - F::cast_from(4.0_f64) * t71426 + F::cast_from(40.0_f64) / F::cast_from(27.0_f64) * t50846;
    (t73330, t73355)
}
