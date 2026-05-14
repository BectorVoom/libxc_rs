//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 996/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk996<F: Float>(t14680: F, t318: F, t294: F, t10961: F, t1425: F, t4960: F, t905: F, t912: F, t14447: F, t14449: F, t14451: F, t14573: F, t14575: F, t14578: F, t14579: F, t14583: F, t14585: F, t14586: F, t14636: F, t14638: F, t14641: F, t14658: F, t14662: F, t14666: F, t4023: F, t993: F) -> (F, F, F, F, F) {
    let t14681 = t14680 * t318;
    let t14683 = 0.19751673498613801407e-1 * t294 * t14681;
    let t14685 = 2.0 * t10961 * t1425;
    let t14686 = t4960 * t905;
    let t14688 = 0.35089341735807877242e1 * t912 * t14686;
    let t14689 = -t14579 * t4023 * t993 + 2.0 * t14586 * t4023 * t993 + t14447 - t14449 + t14451 + t14573 + t14575 + t14578 - t14583 + t14585 - t14636 - t14638 - t14641 - t14658 + t14662 + t14666 + t14683 + t14685 - t14688;
    (t14681, t14683, t14685, t14688, t14689)
}
