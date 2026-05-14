//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 829/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk829<F: Float>(t1232: F, t1768: F, t520: F, t5745: F, t1773: F, t522: F, t5731: F, t1266: F, t1772: F, t1775: F, t538: F, t5732: F, t5734: F, t5737: F, t5739: F, t5742: F) -> (F, F, F) {
    let t5747 = t1768 * t1232 * t520;
    let t5748 = t5745 * t5747;
    let t5751 = t1773 * t522 * t5731;
    let t5753 = -t1266 * t5734 - t1772 * t5751 - t1775 * t5737 + t538 * t5732 + 2.0 * t5739 * t5742 + t5739 * t5748;
    (t5748, t5751, t5753)
}
