//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 656/1154 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk656<F: Float>(t2010: F, t6883: F, t552: F, t562: F, t1307: F, t6637: F, t6888: F, t2009: F, t794: F, t6897: F, t1338: F, t6604: F) -> (F, F, F, F, F, F, F, F) {
    let t6966 = t6883 * t2010;
    let t6967 = 0.19190897446562641759e-1 * t6966;
    let t6968 = t552 * t562;
    let t6969 = t6968 * t1307;
    let t6970 = t6637 * t6969;
    let t6971 = t6888 * t6970;
    let t6973 = t794 * t2009;
    let t6974 = t6897 * t6973;
    let t6975 = 0.41123351671205660912e-2 * t6974;
    let t6976 = t6604 * t1338;
    (t6967, t6968, t6969, t6970, t6971, t6973, t6975, t6976)
}
