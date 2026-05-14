//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1121/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk1121<F: Float>(t1153: F, t15478: F, t15481: F, t15484: F, t15601: F, t15605: F, t15607: F, t15609: F, t15612: F, t15615: F, t15618: F, t15621: F, t15625: F, t15628: F, t15632: F, t15634: F, t15637: F, t15639: F, t15794: F, t16015: F, t198: F, t330: F, t4023: F, t4325: F, t4329: F) -> (F,) {
    let t16022 = t1153 * t16015 * t198 * t330 - 2.0 * t4023 * t4325 * t4329 - t15478 - t15481 - t15484 - t15601 + t15605 - t15607 + t15609 + t15612 - t15615 - t15618 - t15621 + t15625 + t15628 + t15632 + t15634 + t15637 + t15639 + t15794;
    (t16022,)
}
