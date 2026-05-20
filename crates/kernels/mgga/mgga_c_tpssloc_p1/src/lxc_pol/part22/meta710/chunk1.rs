//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2307/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2307<F: Float>(t16596: F, t16662: F, t17116: F, t1877: F, t2522: F, t39483: F, t40732: F, t4310: F, t46237: F, t67146: F, t67147: F, t67153: F, t67154: F, t67158: F, t67159: F, t868: F) -> F {
    let t67160 = -F::new(9.0) * t16596 * t17116 * t2522 + F::new(9.0) * t16662 * t2522 * t4310 - t1877 * t67154 * t868 + t39483 - t40732 + t46237 - t67146 + t67147 + t67153 + t67158 + t67159;
    t67160
}
