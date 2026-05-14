//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 821/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk821<F: Float>(t12248: F, t236: F, t240: F, t1336: F, t10022: F, t248: F, t557: F, t555: F, t10027: F, t541: F, t1361: F, t2690: F, t241: F, t67: F, t6924: F, t1339: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t12289 = t12248 * t236;
    let t12290 = t12289 * t240;
    let t12291 = t1336 * t12290;
    let t12328 = t10022 * t557 * t248;
    let t12330 = 595.0 / 10368.0 * t555 * t12328;
    let t12335 = 455.0 / 1296.0 * t10027 * t541;
    let t12344 = t1361 * t2690;
    let t12345 = t1336 * t12344;
    let t12351 = t241 * t6924 * t67;
    let t12364 = t1339 * t2690;
    (t12289, t12290, t12291, t12328, t12330, t12335, t12344, t12345, t12351, t12364)
}
