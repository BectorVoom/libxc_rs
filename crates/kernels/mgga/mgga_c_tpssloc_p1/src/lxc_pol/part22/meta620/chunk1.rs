//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2152/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2152<F: Float>(t52973: F, t11820: F, t5019: F, t11791: F, t5024: F, t5002: F, t11153: F, t4899: F, t3540: F, t4961: F, t1227: F, t4973: F, t49850: F) -> (F, F, F, F, F, F, F) {
    let t52974 = t52973 / F::cast_from(4608.0_f64);
    let t52987 = t5019 * t11820;
    let t52988 = t52987 / F::cast_from(864.0_f64);
    let t52991 = t5024 * t11791;
    let t52992 = t52991 / F::cast_from(1296.0_f64);
    let t52993 = t5002 * t11820;
    let t52994 = t52993 / F::cast_from(4608.0_f64);
    let t52995 = t4899 * t11153;
    let t52999 = t4961 * t3540;
    let t53000 = t52999 / F::cast_from(864.0_f64);
    let t53033 = t1227 * t49850 * t4973;
    (t52974, t52988, t52992, t52994, t52995, t53000, t53033)
}
