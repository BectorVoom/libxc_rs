//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2493/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2493<F: Float>(t10231: F, t21122: F, t973: F, t13995: F, t17649: F, t17681: F, t21526: F, t42541: F, t43382: F, t50425: F, t50443: F, t62891: F, t62893: F, t62901: F, t62903: F) -> F {
    let t70929 = t973 * t10231 * t21122;
    let t70933 = t42541 * t21526 / F::cast_from(768.0_f64) + t13995 * t17681 / F::cast_from(1536.0_f64) + F::cast_from(5.0_f64) / F::cast_from(1296.0_f64) * t50425 - t13995 * t17649 / F::cast_from(768.0_f64) + t62891 / F::cast_from(576.0_f64) - t62893 / F::cast_from(288.0_f64) + t43382 / F::cast_from(10368.0_f64) + t50443 + t70929 / F::cast_from(216.0_f64) + t62901 / F::cast_from(384.0_f64) - t62903 / F::cast_from(768.0_f64);
    t70933
}
