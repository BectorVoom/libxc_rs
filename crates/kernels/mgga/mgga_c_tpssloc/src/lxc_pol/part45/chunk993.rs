//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 993/1056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk993<F: Float>(t114617: F, t114764: F, t114802: F, t114838: F, t114870: F, t114902: F, t114934: F, t114967: F, t870: F, t1914: F, t2379: F, t2745: F) -> (F, F, F, F) {
    let t114970 = t114617 + t114764 + t114802 + t114838 + t114870 + t114902 + t114934 + t114967;
    let t114971 = t114970 * t870;
    let t114977 = t1914 * t2379;
    let t114988 = t1914 * t2745;
    (t114970, t114971, t114977, t114988)
}
