//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 513/919 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk513<F: Float>(t240: F, t6943: F, t1336: F, t1358: F, t2003: F, t552: F, t59: F, t2007: F, t225: F) -> (F, F, F, F, F, F, F, F) {
    let t6944 = t6943 * t240;
    let t6945 = t1336 * t6944;
    let t6948 = t2003 * t1358;
    let t6949 = 7.0 / 2304.0 * t6948;
    let t6950 = t552 * t59;
    let t6951 = t6950 * t240;
    let t6952 = t1336 * t6951;
    let t6958 = t2007 * t225;
    (t6944, t6945, t6948, t6949, t6950, t6951, t6952, t6958)
}
