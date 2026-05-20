//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2001/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2001<F: Float>(t85: F, t24: F, t10276: F, t73: F, t11152: F, t76: F, t41: F, t42: F, t53: F, t54: F, t9576: F, t2405: F) -> (F, F, F, F, F, F, F) {
    let t39061 = t85 * t85;
    let t39063 = t24 / t39061;
    let t39096 = F::new(1.0) / t73 / t10276;
    let t39114 = F::new(1.0) / t76 / t11152;
    let t39157 = t41 * t41;
    let t39159 = F::new(1.0) / t42 / t39157;
    let t39166 = t53 * t53;
    let t39168 = F::new(1.0) / t54 / t39166;
    let t39210 = F::new(20944.0) / F::new(81.0) * t9576;
    let t39246 = t2405 * t2405;
    (t39063, t39096, t39114, t39159, t39168, t39210, t39246)
}
