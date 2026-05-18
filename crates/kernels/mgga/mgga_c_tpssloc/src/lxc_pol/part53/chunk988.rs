//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 988/1059 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk988<F: Float>(t114790: F, t23164: F, t7479: F, t114866: F, t1880: F, t7488: F, t23168: F, t33419: F, t22986: F, t2647: F, t26656: F, t6646: F) -> (F, F, F, F) {
    let t121464 = t23164 * t114790 * t7479;
    let t121467 = t1880 * t114866 * t7488;
    let t121469 = t23168 * t33419;
    let t121493 = t22986 * t6646 * t26656 * t2647;
    (t121464, t121467, t121469, t121493)
}
