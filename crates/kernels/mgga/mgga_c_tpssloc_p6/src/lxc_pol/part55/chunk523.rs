//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 523/1304 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk523<F: Float>(t1340: F, t3777: F, t1333: F, t1358: F, t1362: F, t1337: F, t551: F) -> (F, F, F, F) {
    let t3778 = t3777 * t1340;
    let t3781 = t1333 * t1358;
    let t3783 = t3777 * t1362;
    let t3787 = F::cast_from(1.0_f64) / t1337 / t551;
    (t3778, t3781, t3783, t3787)
}
