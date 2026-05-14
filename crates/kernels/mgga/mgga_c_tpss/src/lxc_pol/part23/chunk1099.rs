//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1099/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk1099<F: Float>(t33: F, t2: F, t3225: F, t1201: F, t12715: F, t12795: F, t22: F, t2829: F, t3226: F, t4388: F, t4391: F, t555: F, t12794: F, zeta_threshold: F) -> (F,) {
    let t34 = t33 <= zeta_threshold;
    let t12798 = t3225 * t2;
    let t12808 = piecewise3(t34, 0.0, 8.0 / 27.0 * t12795 * t3226 + 8.0 / 9.0 * t12798 * t12715 - 2.0 / 9.0 * t4388 * t2829 - 4.0 / 3.0 * t1201 * t555 + 4.0 * t4391 * t22);
    let t12810 = t12794 / 2.0 + t12808 / 2.0;
    (t12810,)
}
