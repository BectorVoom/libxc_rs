//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 954/1304 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk954<F: Float>(t1983: F, t26149: F, t6880: F, t7685: F, t6876: F, t7754: F, t1982: F, t8944: F) -> (F, F, F, F) {
    let t26150 = t1983 * t26149;
    let t26153 = F::new(3.0) * t7685 * t6880;
    let t26157 = t6876 * t7754;
    let t26161 = t1982 * t8944;
    (t26150, t26153, t26157, t26161)
}
