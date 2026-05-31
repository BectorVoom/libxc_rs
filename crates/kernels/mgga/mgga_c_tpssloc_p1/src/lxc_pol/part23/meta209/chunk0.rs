//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 852/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk852<F: Float>(t12344: F, t1336: F, t241: F, t67: F, t6924: F, t1339: F, t2690: F, t3788: F, t835: F, t1995: F, t246: F, t3700: F, t570: F) -> (F, F, F, F, F, F, F, F, F) {
    let t12345 = t1336 * t12344;
    let t12351 = t241 * t6924 * t67;
    let t12364 = t1339 * t2690;
    let t12365 = t1336 * t12364;
    let t12384 = t3788 * t835;
    let t12385 = t1336 * t12384;
    let t12418 = t1995 * t67;
    let t12419 = t12418 * t246;
    let t12461 = F::cast_from(1.0_f64) / t3700 / t570;
    (t12345, t12351, t12364, t12365, t12384, t12385, t12418, t12419, t12461)
}
