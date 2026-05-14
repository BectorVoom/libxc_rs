//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 887/1291 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk887<F: Float>(t10544: F, t10530: F, t10538: F, t10556: F, t10558: F, t10560: F, t10562: F, t10566: F, t10569: F, t10572: F, t10575: F, t894: F, t901: F, t276: F, t285: F, t2799: F, t896: F) -> (F, F, F, F) {
    let t10577 = 28.0 / 27.0 * t10544;
    let t10588 = -t10577 - 4.0 / 9.0 * t10556 + 2.0 / 9.0 * t10558 - 2.0 / 3.0 * t10560 + t10562 / 3.0 - 10.0 / 27.0 * t10566 + 4.0 / 3.0 * t10569 - 2.0 / 3.0 * t10530 - 2.0 * t10572 + 2.0 * t10538 - t10575 / 3.0;
    let t10589 = t894 * t10588;
    let t10591 = t901 * t10588;
    let t10595 = 1.0 / t276 / t285 / 4.0;
    let t10596 = t2799 * t896;
    (t10589, t10591, t10595, t10596)
}
