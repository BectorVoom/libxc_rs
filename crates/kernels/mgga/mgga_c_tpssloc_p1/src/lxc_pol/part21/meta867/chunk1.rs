//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3165/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3165<F: Float>(t1653: F, t3507: F, t11678: F, t11697: F, t19001: F, t11692: F, t11825: F, t14726: F, t15659: F, t15661: F, t15702: F, t1735: F, t18395: F, t19083: F, t19101: F, t3490: F, t3493: F, t3577: F, t3578: F, t3587: F, t45114: F, t45128: F, t45197: F, t52704: F, t53149: F, t6207: F, t65464: F, t65469: F, t65474: F, t65479: F, t65482: F, t65485: F) -> F {
    let t65492 = t1653 * t3507;
    let t65506 = t11678 * t11697 * t19001;
    let t65518 = -t11678 * t3578 * t65464 * t15661 / F::new(1152.0) + t11692 * t3578 * t65469 * t15702 / F::new(2304.0) - t45197 * t3578 * t65474 * t15661 / F::new(384.0) - t65479 / F::new(1728.0) + t65482 / F::new(1728.0) - t65485 / F::new(864.0) - t11678 * t3578 * t15659 * t1653 * t3493 / F::new(1152.0) - t45197 * t3578 * t52704 * t65492 / F::new(384.0) + t45114 * t3578 * t15659 * t65492 / F::new(384.0) + t11692 * t3578 * t53149 * t18395 / F::new(2304.0) - t65506 / F::new(864.0) - F::new(5.0) / F::new(2592.0) * t3577 * t45128 * t1735 * t14726 - F::new(5.0) / F::new(1296.0) * t19083 * t3587 - t11825 * t6207 / F::new(4608.0) - t3490 * t19101 / F::new(2304.0);
    t65518
}
