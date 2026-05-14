//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 405/939 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk405<F: Float>(t2989: F, t607: F, t2822: F, t225: F, t991: F, t1008: F, t191: F, t349: F, t1011: F, t68: F) -> (F, F, F, F, F, F) {
    let t2990 = t2989 * t607;
    let t3003 = 5.0 / 18.0 * t2822;
    let t3026 = t991 * t225;
    let t3030 = 1.0 / t1008 / t191;
    let t3031 = t349 * t3030;
    let t3032 = t1011 * t68;
    (t2990, t3003, t3026, t3030, t3031, t3032)
}
