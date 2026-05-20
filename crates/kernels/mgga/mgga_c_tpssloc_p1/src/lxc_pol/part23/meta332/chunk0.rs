//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1100/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1100<F: Float>(t39035: F, t14: F, t2230: F, t594: F, t9223: F, t22811: F, t19: F, t85: F, t24: F, t10276: F, t73: F, t11152: F, t76: F) -> (F, F, F, F, F, F, F, F) {
    let t39036 = F::new(0.74688e4) * t39035;
    let t39037 = t14 * t2230;
    let t39038 = F::new(0.175056e5) * t39037;
    let t39039 = t594 * t9223;
    let t39040 = F::new(0.1822464e5) * t39039;
    let t39041 = F::new(1.0) / t22811;
    let t39043 = F::new(0.683424e4) * t19 * t39041;
    let t39061 = t85 * t85;
    let t39063 = t24 / t39061;
    let t39096 = F::new(1.0) / t73 / t10276;
    let t39114 = F::new(1.0) / t76 / t11152;
    (t39036, t39037, t39038, t39040, t39043, t39063, t39096, t39114)
}
