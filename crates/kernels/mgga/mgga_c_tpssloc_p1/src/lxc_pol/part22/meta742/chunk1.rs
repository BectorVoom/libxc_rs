//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2453/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2453<F: Float>(t17635: F, t4337: F, t10254: F, t21510: F, t13769: F, t13835: F, t13839: F, t1409: F, t17748: F, t17800: F, t17804: F, t2986: F, t2988: F, t2989: F, t4518: F, t4531: F, t4540: F, t5681: F, t5685: F, t61082: F, t61103: F, t61279: F, t61307: F, t61310: F, t61313: F, t61322: F, t61327: F, t61365: F, t6733: F) -> (F, F) {
    let t69742 = t4337 * t17635;
    let t69746 = t10254 * t21510;
    let t69791 = F::cast_from(0.49999999999999999998e-2_f64) * t2986 * t4518 * t69742 + F::cast_from(0.16666666666666666666e-2_f64) * t2986 * t2988 * t69746 + F::cast_from(0.16666666666666666666e-2_f64) * t2986 * t17800 * t13835 - F::cast_from(0.11111111111111111111e-2_f64) * t2986 * t61322 * t13839 + F::cast_from(0.22222222222222222222e-2_f64) * t61307 + F::cast_from(0.55555555555555555554e-3_f64) * t61310 + F::cast_from(0.55555555555555555554e-3_f64) * t61313 - F::cast_from(0.27777777777777777777e-3_f64) * t61327 - F::cast_from(0.49999999999999999998e-2_f64) * t2986 * t4531 * t61082 - F::cast_from(0.11111111111111111111e-2_f64) * t2986 * t13769 * t61279 - F::cast_from(0.83333333333333333331e-3_f64) * t2986 * t4531 * t6733 * t5685 + F::cast_from(0.33333333333333333332e-2_f64) * t2986 * t4531 * t61103 + F::cast_from(0.16666666666666666666e-2_f64) * t2986 * t4531 * t6733 * t5681 - F::cast_from(0.16666666666666666666e-2_f64) * t2986 * t4531 * t2989 * t1409 * t4540 + F::cast_from(0.16666666666666666666e-2_f64) * t2986 * t17804 * t13835 - F::cast_from(0.11111111111111111111e-2_f64) * t2986 * t61365 * t13839 - F::cast_from(0.83333333333333333331e-3_f64) * t2986 * t17804 * t17748;
    (t69742, t69791)
}
