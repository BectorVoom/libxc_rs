//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1144/1149 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1144<F: Float>(t20173: F, t33193: F, t3941: F, t4072: F, t8326: F, t7015: F, t86647: F, t16524: F, t31285: F, t16521: F, t12524: F, t33188: F, t26135: F, t7010: F, t120758: F, t120786: F, t120788: F, t120789: F, t120792: F, t120793: F, t120795: F, t31284: F, t33195: F, t577: F, t8508: F) -> (F,) {
    let t120800 = 27.0 * t20173 * t33193;
    let t120803 = 27.0 * t3941 * t8326 * t4072;
    let t120804 = t86647 * t7015;
    let t120807 = 27.0 * t16524 * t31285;
    let t120809 = 0.135e2 * t16521 * t8326;
    let t120811 = 54.0 * t12524 * t33188;
    let t120812 = t7010 * t26135;
    let t120814 = t31284 + t8508 + t120786 + t120788 + 54.0 * t120789 + t33195 + t120792 + 27.0 * t120793 + 54.0 * t120795 + 0.45e1 * t120758 * t577 + t120800 + t120803 + 54.0 * t120804 + t120807 + t120809 + t120811 + 27.0 * t120812;
    (t120814,)
}
