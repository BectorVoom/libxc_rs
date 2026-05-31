//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1186/1438 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1186<F: Float>(t1955: F, t3206: F, t3174: F, t10160: F, t1052: F, t1066: F, t1920: F, t1956: F, t23346: F, t23354: F, t23359: F, t23366: F, t23369: F, t23372: F, t3169: F, t3176: F, t3207: F, t6687: F, t6695: F, t6771: F, t6816: F) -> (F, F) {
    let t23377 = t1955 * t3206;
    let t23378 = t3174 * t23377;
    let t23381 = F::cast_from(0.43864908449286038306e-1_f64) * t23346 * t6695 + F::cast_from(0.82246703342411321825e-2_f64) * t1920 * t23354 - t23359 - F::cast_from(2.0_f64) * t3169 * t6816 - t6771 * t3207 + F::cast_from(2.0_f64) * t6771 * t3176 - F::cast_from(0.16449340668482264365e-1_f64) * t6687 * t23366 - F::cast_from(2.0_f64) * t23369 * t1066 - F::cast_from(2.0_f64) * t23372 * t1066 - F::cast_from(2.0_f64) * t10160 * t1956 + F::cast_from(2.0_f64) * t1052 * t23378;
    (t23378, t23381)
}
