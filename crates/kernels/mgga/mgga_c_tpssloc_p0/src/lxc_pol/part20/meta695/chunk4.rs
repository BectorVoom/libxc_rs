//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2651/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2651<F: Float>(t1351: F, t3850: F, t12290: F, t5234: F, t16060: F, t3789: F, t12012: F, t12215: F, t12293: F, t12303: F, t12420: F, t16048: F, t16224: F, t16233: F, t16235: F, t16242: F, t16306: F, t1810: F, t1825: F, t210: F, t3719: F, t3733: F, t3734: F, t3795: F, t3803: F, t39971: F, t5226: F, t5248: F, t53985: F, t53990: F, t53998: F, t54003: F, t54013: F, t54014: F) -> (F, F) {
    let t54015 = t1351 * t3850;
    let t54020 = t5234 * t12290;
    let t54023 = t16060 * t3789;
    let t54026 = -F::cast_from(5.0_f64) / F::cast_from(256.0_f64) * t3803 * t16224 * t1825 * t12303 - F::cast_from(7.0_f64) / F::cast_from(384.0_f64) * t39971 - t53985 - F::cast_from(5.0_f64) / F::cast_from(256.0_f64) * t3803 * t16224 * t16306 * t12420 - F::cast_from(3.0_f64) / F::cast_from(512.0_f64) * t53990 * t16235 - F::cast_from(3.0_f64) / F::cast_from(512.0_f64) * t16233 * t5248 * t16242 * t16048 + t53998 - F::cast_from(3.0_f64) / F::cast_from(4.0_f64) * t12215 * t210 * t5226 * t3734 - F::cast_from(7.0_f64) / F::cast_from(16.0_f64) * t54003 + F::cast_from(3.0_f64) / F::cast_from(16.0_f64) * t3733 * t210 * t5226 * t3719 + t3733 * t210 * t1810 * t12012 / F::cast_from(16.0_f64) - F::cast_from(3.0_f64) / F::cast_from(512.0_f64) * t16233 * t54013 * t54014 * t54015 - t54020 * t12293 / F::cast_from(512.0_f64) + t54023 * t3795 / F::cast_from(512.0_f64);
    (t54015, t54026)
}
