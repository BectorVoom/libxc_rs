//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1781/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1781<F: Float>(t2240: F, t240: F, t33: F, t6492: F, t23993: F, t6495: F, t1860: F, t1864: F, t67: F, t835: F, t6486: F, t80743: F) -> (F, F, F, F, F, F) {
    let t84241 = t2240 * t33 * t240;
    let t84242 = t84241 * t6492;
    let t84248 = t6495 * t23993;
    let t84280 = F::new(1232.0) / F::new(81.0) * t1860 * t835 * t67 * t1864;
    let t84285 = t6486 * t23993;
    let t84400 = F::cast_from(0.3244175520728446583e0_f64) * t80743;
    (t84241, t84242, t84248, t84280, t84285, t84400)
}
