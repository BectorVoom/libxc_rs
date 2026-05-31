//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1004/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1004<F: Float>(t10777: F, t10779: F, t10782: F, t10786: F, t10790: F, t10795: F, t10799: F, t10803: F, t10805: F, t10809: F, t10813: F, t2173: F, t3626: F, t8289: F, t8293: F, t8314: F) -> F {
    let t10816 = F::cast_from(7.0_f64) / F::cast_from(4608.0_f64) * t8289 - F::cast_from(7.0_f64) / F::cast_from(2304.0_f64) * t8293 - F::cast_from(7.0_f64) / F::cast_from(576.0_f64) * t8314 - t10777 - t10779 * t10782 / F::cast_from(512.0_f64) + t3626 * t10786 / F::cast_from(512.0_f64) - t3626 * t10790 / F::cast_from(384.0_f64) + t2173 * t10795 / F::cast_from(384.0_f64) + t2173 * t10799 / F::cast_from(768.0_f64) - t10803 + t2173 * t10805 / F::cast_from(768.0_f64) - t2173 * t10809 / F::cast_from(3072.0_f64) - F::cast_from(5.0_f64) / F::cast_from(768.0_f64) * t2173 * t10813;
    t10816
}
