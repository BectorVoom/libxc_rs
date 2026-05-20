//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 975/1400 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk975<F: Float>(t461: F, t6729: F, t7324: F, t2131: F, t23508: F, t1222: F, t7334: F, t2141: F, t3540: F, t3: F, t1184: F, t52: F) -> (F, F, F, F, F, F) {
    let t24649 = t6729 * t461;
    let t24650 = t7324 * t24649;
    let t24658 = t2131 * t23508;
    let t24675 = t7334 * t1222;
    let t24681 = t2141 * t3540 / F::new(6912.0);
    let t24682 = t7324 * t3;
    let t24683 = t52 * t1184;
    (t24650, t24658, t24675, t24681, t24682, t24683)
}
