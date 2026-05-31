//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2695/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2695<F: Float>(t1372: F, t5286: F, t1824: F, t3879: F, t12240: F, t1351: F, t16205: F, t562: F, t12168: F, t1352: F, t16036: F, t16040: F, t16041: F, t16047: F, t16048: F, t16055: F, t26409: F, t3773: F, t3793: F, t3851: F, t3856: F, t5333: F, t5334: F, t5335: F, t5336: F, t5343: F, t5344: F, t5345: F) -> (F, F, F, F, F) {
    let t54840 = t1372 * t5286;
    let t54854 = t3879 * t1824;
    let t54858 = t12240 * t1351;
    let t54883 = t562 * t16205;
    let t54900 = -t12168 * t5335 * t5344 + F::cast_from(6.0_f64) * t12240 * t16036 * t5334 - F::cast_from(3.0_f64) * t1352 * t5344 * t54883 - F::cast_from(18.0_f64) * t16036 * t16047 * t16048 + F::cast_from(18.0_f64) * t16036 * t3793 * t5334 + F::cast_from(18.0_f64) * t16040 * t3793 * t5334 - F::cast_from(3.0_f64) * t16040 * t3856 * t5344 - F::cast_from(3.0_f64) * t26409 * t3851 * t5344 + F::cast_from(6.0_f64) * t3773 * t5333 * t5336 - F::cast_from(3.0_f64) * t3773 * t5343 * t5345 + F::cast_from(12.0_f64) * t16041 * t16055;
    (t54840, t54854, t54858, t54883, t54900)
}
