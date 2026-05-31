//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1288/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1288<F: Float>(t16524: F, t31285: F, t16521: F, t8326: F, t12524: F, t33188: F, t26135: F, t7010: F, t120758: F, t120786: F, t120788: F, t120789: F, t120792: F, t120793: F, t120795: F, t120800: F, t120803: F, t120804: F, t31284: F, t33195: F, t577: F, t8508: F) -> F {
    let t120807 = F::cast_from(27.0_f64) * t16524 * t31285;
    let t120809 = F::cast_from(0.135e2_f64) * t16521 * t8326;
    let t120811 = F::cast_from(54.0_f64) * t12524 * t33188;
    let t120812 = t7010 * t26135;
    let t120814 = t31284 + t8508 + t120786 + t120788 + F::cast_from(54.0_f64) * t120789 + t33195 + t120792 + F::cast_from(27.0_f64) * t120793 + F::cast_from(54.0_f64) * t120795 + F::cast_from(0.45e1_f64) * t120758 * t577 + t120800 + t120803 + F::cast_from(54.0_f64) * t120804 + t120807 + t120809 + t120811 + F::cast_from(27.0_f64) * t120812;
    t120814
}
