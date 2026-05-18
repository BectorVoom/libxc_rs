//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 795/1034 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk795<F: Float>(t24649: F, t7324: F, t3493: F, t475: F, t68: F, t7328: F, t2131: F, t23508: F, t7325: F, t3030: F, t3502: F, t478: F) -> (F, F, F, F) {
    let t24650 = t7324 * t24649;
    let t24654 = t3493 * t68 * t475;
    let t24655 = t7328 * t24654;
    let t24658 = t2131 * t23508;
    let t24659 = t24658 * t7325;
    let t24660 = t3030 * t3502;
    let t24661 = t24660 * t478;
    (t24650, t24655, t24659, t24661)
}
