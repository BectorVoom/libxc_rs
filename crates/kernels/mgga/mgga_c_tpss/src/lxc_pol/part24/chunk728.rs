//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 728/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk728<F: Float>(t45: F, t57: F, t4716: F, t773: F, t774: F, t1364: F, t226: F, t3629: F, t2175: F, t3643: F, t2225: F, t4573: F, t4579: F, t78: F, t2232: F, t81: F, t150: F, t190: F, zeta_threshold: F) -> (F, F, F, F, F, F, F) {
    let t151 = t45 <= zeta_threshold;
    let t155 = t57 <= zeta_threshold;
    let t4718 = t773 * t774 * t4716;
    let t4722 = t226 * t1364;
    let t4723 = t3629 * t4722;
    let t4724 = t2175 * t4723;
    let t4727 = 8.0 * t3643;
    let t4733 = piecewise3(t151, 0.0, 4.0 / 9.0 * t2225 * t4573 + 4.0 / 3.0 * t78 * t4579);
    let t4739 = piecewise3(t155, 0.0, 4.0 / 9.0 * t2232 * t4573 - 4.0 / 3.0 * t81 * t4579);
    let t4740 = t4733 + t4739;
    let t4741 = t150 * t4740;
    let t4742 = t4741 * t190;
    (t4718, t4722, t4724, t4727, t4740, t4741, t4742)
}
