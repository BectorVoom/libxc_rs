//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1352/1484 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1352<F: Float>(t55353: F, t8657: F, t1983: F, t24990: F, t31758: F, t24991: F, t8607: F, t4026: F, t8595: F, t1442: F, t31518: F, t22574: F, t31299: F, t33899: F) -> (F, F, F, F, F, F) {
    let t120871 = F::new(27.0) * t55353 * t8657;
    let t120874 = F::new(3.0) * t1983 * t31758 * t24990;
    let t120876 = F::new(3.0) * t8607 * t24991;
    let t120877 = t4026 * t8595;
    let t120878 = t1442 * t31518;
    let t120881 = F::new(3.0) * t22574 * t33899 * t31299;
    (t120871, t120874, t120876, t120877, t120878, t120881)
}
