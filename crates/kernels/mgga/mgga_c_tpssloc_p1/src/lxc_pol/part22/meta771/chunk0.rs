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
    let t73355 = t71333 / F::new(18.0) - t71335 / F::new(9.0) + F::new(2.0) / F::new(3.0) * t71337 + F::new(4.0) / F::new(27.0) * t63841 + F::new(2.0) / F::new(3.0) * t63843 - t63845 / F::new(9.0) + t63886 / F::new(3.0) + F::new(5.0) / F::new(27.0) * t63888 - F::new(10.0) / F::new(9.0) * t63893 + F::new(14.0) / F::new(81.0) * t71400 - F::new(5.0) / F::new(9.0) * t63911 - t71406 / F::new(6.0) + t71408 / F::new(9.0) - F::new(2.0) / F::new(9.0) * t71411 - F::new(8.0) / F::new(9.0) * t71414 + t71417 + F::new(2.0) * t71420 - F::new(3.0) * t71423 - F::new(4.0) * t71426 + F::new(40.0) / F::new(27.0) * t50846;
    (t73330, t73355)
}
