//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1225/1475 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1225<F: Float>(t7940: F, t8643: F, t1983: F, t25224: F, t8547: F, t1880: F, t1484: F, t31376: F, t6637: F, t6552: F, t232: F, t26656: F) -> (F, F, F, F, F, F, F, F) {
    let t33366 = t7940 * t8643;
    let t33367 = t1983 * t33366;
    let t33371 = t25224 * t8547;
    let t33372 = t1880 * t33371;
    let t33375 = t31376 * t1484;
    let t33376 = t6637 * t33375;
    let t33377 = t6552 * t33376;
    let t33379 = t26656 * t232;
    (t33366, t33367, t33371, t33372, t33375, t33376, t33377, t33379)
}
