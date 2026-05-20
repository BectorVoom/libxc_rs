//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 555/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk555<F: Float>(t1131: F, t1134: F, t1655: F, t1662: F, t1665: F, t1668: F) -> F {
    let t1682 = F::new(0.3529725e1) * t1662 - t1131 + F::new(0.516475e0) * t1655 + F::new(0.6311625e0) * t1665 - t1134 + F::new(0.104195e0) * t1668;
    t1682
}
