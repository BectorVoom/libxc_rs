//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1189/1438 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1189<F: Float>(t1922: F, t3010: F, t2776: F, t6690: F, t6689: F, t1945: F, t3020: F, t6768: F, t990: F, t2250: F, t3: F, t1933: F) -> (F, F, F, F, F, F, F) {
    let t23399 = t3010 * t1922;
    let t23402 = t6690 * t2776;
    let t23403 = t6689 * t23402;
    let t23408 = t3020 * t1945;
    let t23410 = t990 * t6768;
    let t23413 = t3 * t2250;
    let t23414 = t1933 * t23413;
    (t23399, t23402, t23403, t23408, t23410, t23413, t23414)
}
