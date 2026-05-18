//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 847/1049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk847<F: Float>(t6883: F, t8631: F, t2085: F, t552: F, t794: F, t8630: F, t6897: F, t1338: F, t8617: F, t8622: F, t225: F, t8618: F) -> (F, F, F, F, F, F, F) {
    let t31616 = t6883 * t8631;
    let t31617 = F::new(0.19190897446562641759e-1) * t31616;
    let t31618 = t552 * t2085;
    let t31623 = t794 * t8630;
    let t31624 = t6897 * t31623;
    let t31625 = F::new(0.41123351671205660912e-2) * t31624;
    let t31636 = t1338 * t8617;
    let t31648 = t6883 * t8622;
    let t31649 = F::new(0.19190897446562641759e-1) * t31648;
    let t31653 = t8618 * t225;
    (t31617, t31618, t31623, t31625, t31636, t31649, t31653)
}
