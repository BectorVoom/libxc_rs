//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 969/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk969<F: Float>(t10590: F, t3628: F, t783: F, t2365: F, t3629: F, t3671: F, t8313: F, t2175: F, t2177: F, t8320: F, t3630: F, t8330: F, t1385: F, t8130: F, t2383: F, t3689: F) -> (F, F, F, F, F, F, F, F, F) {
    let t10592 = t3628 * t10590 * t783;
    let t10596 = t3628 * t3629 * t2365;
    let t10600 = 7.0 / 2304.0 * t8313 * t3671;
    let t10602 = t2175 * t10590 * t2177;
    let t10606 = t2175 * t3629 * t8320;
    let t10610 = t3628 * t10590 * t3630;
    let t10614 = t3628 * t3629 * t8330;
    let t10617 = t8130 * t1385;
    let t10620 = 7.0 / 576.0 * t2383 * t3689;
    (t10592, t10596, t10600, t10602, t10606, t10610, t10614, t10617, t10620)
}
