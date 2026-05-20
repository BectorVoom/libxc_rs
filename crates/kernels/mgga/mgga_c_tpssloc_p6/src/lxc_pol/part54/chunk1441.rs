//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1441/1484 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1441<F: Float>(t31776: F, t96797: F, t1983: F, t33136: F, t7217: F, t33623: F, t6876: F, t33214: F, t7057: F, t25985: F, t8607: F, t27171: F, t8526: F) -> (F, F, F, F, F, F) {
    let t122587 = F::new(2.0) * t96797 * t31776;
    let t122589 = t1983 * t7217 * t33136;
    let t122590 = t6876 * t33623;
    let t122593 = F::new(2.0) * t33214 * t7057;
    let t122595 = F::new(3.0) * t8607 * t25985;
    let t122597 = t8526 * t27171;
    (t122587, t122589, t122590, t122593, t122595, t122597)
}
