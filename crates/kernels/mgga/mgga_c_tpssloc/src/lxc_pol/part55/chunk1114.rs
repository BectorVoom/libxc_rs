//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1114/1304 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1114<F: Float>(t1874: F, t33690: F, t7266: F, t7461: F, t27863: F, t1873: F, t7467: F, t7756: F, t8690: F, t2165: F, t652: F, t4028: F, t8675: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t33691 = t33690 * t1874;
    let t33693 = t7266 * t7461;
    let t33697 = t27863 * t1874;
    let t33711 = t27863 * t1873;
    let t33713 = t33690 * t1873;
    let t33715 = t7266 * t7467;
    let t33725 = t8690 * t7756;
    let t33726 = t2165 * t7467;
    let t33727 = t652 * t33726;
    let t33731 = t4028 * t8675;
    (t33691, t33693, t33697, t33711, t33713, t33715, t33725, t33726, t33727, t33731)
}
