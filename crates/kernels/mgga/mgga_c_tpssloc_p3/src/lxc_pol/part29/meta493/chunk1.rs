//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 1845/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1845<F: Float>(t265: F, t504: F, t24629: F, t24900: F, t3640: F, t7394: F, t11947: F, t2157: F, t1254: F, t1256: F, t193: F, t23772: F, t336: F, t3633: F, t3637: F, t4700: F, t7398: F) -> (F, F, F, F) {
    let t505 = t265 < t504;
    let t24901 = t24629 + t24900;
    let t24905 = t7394 * t3640;
    let t24909 = t2157 * t11947;
    let t24916 = piecewise3::<F>(t505, t1256 * t193 * t24901 * t336 - F::cast_from(2.0_f64) * t1254 * t24905 * t4700 + F::cast_from(2.0_f64) * t24909 * t3637 * t4700 - t3633 * t4700 * t7398, t23772);
    (t24901, t24905, t24909, t24916)
}
