//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2292/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2292<F: Float>(t16413: F, t1985: F, t214: F, t225: F, t567: F, t22635: F, t26214: F, t26331: F, t3734: F, t22666: F, t26202: F, t22642: F, t22643: F, t7700: F) -> (F, F, F, F) {
    let t90626 = t1985 * t214 * t16413 * t225 * t567;
    let t90634 = t26331 * t22635 * t26214 * t3734;
    let t90639 = t1985 * t22666 * t26202;
    let t90642 = t22642 * t22643 * t7700;
    (t90626, t90634, t90639, t90642)
}
