//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 1999/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1999<F: Float>(t460: F, t6144: F, t64: F, t9365: F, t20: F, t60: F, t9108: F, t94: F, t102: F, t9174: F, t2: F, t591: F) -> (F, F, F, F, F, F) {
    let t29614 = t6144 * t460;
    let t29903 = t64 * t9365;
    let t32253 = F::new(1.0) / t60 / t20;
    let t35577 = t94 * t9108;
    let t35761 = t102 * t9174;
    let t39031 = t2 * t591;
    (t29614, t29903, t32253, t35577, t35761, t39031)
}
