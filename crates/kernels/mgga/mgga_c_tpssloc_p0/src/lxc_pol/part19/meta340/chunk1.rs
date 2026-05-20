//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1210/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1210<F: Float>(t2707: F, t9601: F, t2697: F, t9997: F, t9609: F, t2703: F, t40904: F, t842: F, t2623: F, t2701: F, t40959: F, t40962: F, t40966: F, t40971: F, t40972: F, t40977: F, t820: F, t843: F, t849: F, t9990: F) -> F {
    let t40982 = t9601 * t2707;
    let t40984 = t2697 * t9997;
    let t40988 = t2697 * t9609;
    let t40990 = t9601 * t2703;
    let t40992 = t40904 * t842;
    let t40995 = -F::new(35.0) / F::new(96.0) * t40959 + F::new(7.0) / F::new(96.0) * t40962 + F::new(595.0) / F::new(648.0) * t40966 - F::new(5.0) / F::new(32.0) * t2623 * t9609 + F::new(35.0) / F::new(128.0) * t843 * t40971 * t820 * t40972 + F::new(5.0) / F::new(256.0) * t843 * t2701 * t820 * t40977 - F::new(119.0) / F::new(576.0) * t40982 + F::new(7.0) / F::new(288.0) * t40984 + F::new(5.0) / F::new(128.0) * t9990 * t2703 + F::new(35.0) / F::new(48.0) * t40988 + F::new(595.0) / F::new(576.0) * t40990 - t40992 * t849 / F::new(192.0);
    t40995
}
