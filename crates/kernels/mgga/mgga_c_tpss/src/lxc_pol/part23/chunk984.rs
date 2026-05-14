//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 984/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk984<F: Float>(t3638: F, t8313: F, t236: F, t339: F, t8276: F, t2161: F, t8279: F, t3628: F, t3629: F, t2163: F, t2175: F, t3676: F, t125: F, t3610: F, t783: F, t2365: F) -> (F, F, F, F, F, F, F, F) {
    let t10777 = 7.0 / 576.0 * t8313 * t3638;
    let t10779 = t339 * t8276 * t236;
    let t10780 = t8279 * t2161;
    let t10782 = t3628 * t3629 * t10780;
    let t10786 = t3628 * t3629 * t2163;
    let t10790 = t2175 * t3676 * t2163;
    let t10793 = t125 * t3610;
    let t10795 = t2175 * t10793 * t783;
    let t10799 = t2175 * t3676 * t2365;
    (t10777, t10779, t10780, t10782, t10786, t10790, t10795, t10799)
}
