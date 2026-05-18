//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 892/1056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk892<F: Float>(t31623: F, t6897: F, t1351: F, t2085: F, t550: F, t6976: F, t1992: F, t1998: F, t7191: F, t214: F, t1985: F, t1338: F, t8617: F) -> (F, F, F, F, F, F, F, F) {
    let t31624 = t6897 * t31623;
    let t31625 = F::new(0.41123351671205660912e-2) * t31624;
    let t31627 = t2085 * t1351 * t550;
    let t31628 = t6976 * t31627;
    let t31629 = t1992 * t31628;
    let t31631 = t1998 * t7191;
    let t31632 = t214 * t31631;
    let t31633 = t1985 * t31632;
    let t31636 = t1338 * t8617;
    (t31625, t31627, t31628, t31629, t31631, t31632, t31633, t31636)
}
