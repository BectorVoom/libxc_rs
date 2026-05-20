//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2536/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2536<F: Float>(t43816: F, t43820: F, t51073: F, t51082: F, t63361: F, t63382: F, t63384: F, t63398: F, t63400: F, t71166: F, t71170: F, t71174: F, t71179: F, t71183: F, t71187: F, t71191: F, t71195: F, t71199: F, t71203: F, t71206: F) -> F {
    let t71389 = -F::new(80.0) / F::new(81.0) * t71166 + F::new(6.0) * t71170 + F::new(8.0) * t71174 + F::new(2.0) / F::new(3.0) * t71179 - F::new(2.0) / F::new(3.0) * t71183 - F::new(2.0) / F::new(3.0) * t71187 + F::new(2.0) * t71191 - F::new(4.0) * t71195 - F::new(8.0) * t71199 + F::new(2.0) * t71203 + F::new(6.0) * t71206 - t51073 + t51082 + t43820 - F::new(28.0) / F::new(81.0) * t43816 + F::new(8.0) / F::new(9.0) * t63361 + F::new(4.0) / F::new(9.0) * t63382 + F::new(4.0) / F::new(3.0) * t63384 - F::new(4.0) / F::new(3.0) * t63398 - F::new(2.0) * t63400;
    t71389
}
