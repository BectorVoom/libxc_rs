//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1386/1400 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1386<F: Float>(t26149: F, t8690: F, t12725: F, t8675: F, t33690: F, t6535: F, t24932: F, t7461: F, t27888: F, t25980: F, t7266: F, t31832: F, t7688: F) -> (F, F, F, F, F, F, F) {
    let t123205 = t8690 * t26149;
    let t123206 = t12725 * t8675;
    let t123211 = t33690 * t6535;
    let t123213 = t24932 * t7461;
    let t123215 = t27888 * t7461;
    let t123217 = t7266 * t25980;
    let t123220 = t31832 * t7688;
    (t123205, t123206, t123211, t123213, t123215, t123217, t123220)
}
