//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1216/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1216<F: Float>(t12328: F, t555: F, t10027: F, t541: F, t3777: F, t3865: F, t1361: F, t2690: F, t1336: F, t1369: F, t241: F, t67: F, t6924: F) -> (F, F, F, F, F, F) {
    let t12330 = F::cast_from(595.0_f64) / F::cast_from(10368.0_f64) * t555 * t12328;
    let t12335 = F::cast_from(455.0_f64) / F::cast_from(1296.0_f64) * t10027 * t541;
    let t12339 = t3777 * t3865;
    let t12344 = t1361 * t2690;
    let t12345 = t1336 * t12344;
    let t12346 = t12345 * t1369;
    let t12351 = t241 * t6924 * t67;
    (t12330, t12335, t12339, t12345, t12346, t12351)
}
