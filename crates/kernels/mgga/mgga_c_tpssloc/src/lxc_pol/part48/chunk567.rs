//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 567/1034 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk567<F: Float>(t1988: F, t6883: F, t131: F, t209: F, t547: F, t1878: F) -> (F, F, F, F) {
    let t6884 = t6883 * t1988;
    let t6885 = F::new(0.19190897446562641759e-1) * t6884;
    let t6887 = t547 * t131 * t209;
    let t6888 = t1878 * t6887;
    (t6884, t6885, t6887, t6888)
}
