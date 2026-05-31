//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2965/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2965<F: Float>(t13961: F, t4641: F, t14137: F, t4644: F, t12606: F, t1409: F, t10408: F, t1041: F, t10891: F, t13555: F, t13559: F, t14077: F, t1616: F, t17632: F, t17962: F, t3070: F, t3071: F, t3109: F, t42743: F, t4337: F, t4582: F, t4583: F, t4652: F, t48460: F, t48463: F, t5880: F, t61768: F, t61775: F, t61782: F, t61784: F) -> (F, F) {
    let t61794 = t4641 * t13961;
    let t61796 = t4644 * t14137;
    let t61798 = t1409 * t12606;
    let t61803 = t3070 * t3071 * t1616 * t13559 / F::cast_from(384.0_f64) - t48460 / F::cast_from(864.0_f64) + F::cast_from(5.0_f64) / F::cast_from(5184.0_f64) * t48463 + F::cast_from(5.0_f64) / F::cast_from(5184.0_f64) * t61768 - F::cast_from(5.0_f64) / F::cast_from(1152.0_f64) * t3070 * t10408 * t1616 * t13555 + F::cast_from(5.0_f64) / F::cast_from(3456.0_f64) * t3070 * t10408 * t4337 * t61775 - t61782 / F::cast_from(20736.0_f64) - t61784 / F::cast_from(864.0_f64) + t10891 * t17632 / F::cast_from(144.0_f64) - t3109 * t17962 / F::cast_from(288.0_f64) - t42743 * t5880 / F::cast_from(3072.0_f64) - t14077 * t4652 / F::cast_from(144.0_f64) + t61794 / F::cast_from(1152.0_f64) + F::cast_from(5.0_f64) / F::cast_from(5184.0_f64) * t61796 - t1041 * t4582 * t4583 * t61798 / F::cast_from(1152.0_f64);
    (t61798, t61803)
}
