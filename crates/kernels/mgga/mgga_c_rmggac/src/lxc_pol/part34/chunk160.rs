//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 160/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk160<F: Float>(t236: F, t676: F, t515: F, t664: F, t653: F, t657: F, t659: F, t662: F) -> (F, F, F) {
    let t677 = t676 * t236;
    let t687 = t515 * t664;
    let t698 = -F::new(0.99785347515531738034e-2) * t653 + F::new(0.22728884711871118108e-2) * t657 - F::new(0.13276154105060581339e-3) * t659 + F::new(0.3024012879486021305e-4) * t662;
    (t677, t687, t698)
}
