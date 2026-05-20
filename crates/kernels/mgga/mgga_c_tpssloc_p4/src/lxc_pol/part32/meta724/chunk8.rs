//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2326/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2326<F: Float>(t27634: F, t3030: F, t95420: F, t52: F, t6144: F, t24682: F, t460: F, t1210: F, t1215: F, t18387: F, t18969: F, t24741: F, t27639: F, t27645: F, t29563: F, t3032: F, t475: F, t488: F, t4965: F, t6224: F, t7321: F, t7331: F, t8048: F, t86275: F, t86278: F, t95396: F, t95480: F, t95487: F, t95491: F) -> (F, F) {
    let t104266 = t27634 * t95420 * t3030;
    let t104280 = t52 * t6144;
    let t104282 = t24682 * t104280 * t460;
    let t104292 = -F::cast_from(0.40372756094140390856e-3_f64) * t104266 * t27639 + F::cast_from(0.20186378047070195428e-3_f64) * t104266 * t27645 + t95480 - F::cast_from(0.72670960969452703541e-2_f64) * t29563 * t7321 - t95487 - t86275 / F::new(6912.0) + F::cast_from(0.10093189023535097714e-3_f64) * t95396 * t1210 * t6224 * t3032 * t1215 * t475 - F::cast_from(0.10093189023535097714e-3_f64) * t104282 * t7331 + t86278 - t95491 - t4965 * t8048 * t488 / F::new(144.0) - t24741 * t18969 / F::new(2304.0) - t24741 * t18387 / F::new(1152.0);
    (t104280, t104292)
}
