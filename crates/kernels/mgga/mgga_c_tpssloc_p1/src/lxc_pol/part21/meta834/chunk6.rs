//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2959/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2959<F: Float>(t10904: F, t17667: F, t1040: F, t17877: F, t1041: F, t1046: F, t10517: F, t10863: F, t10898: F, t13995: F, t14235: F, t17890: F, t17962: F, t248: F, t3048: F, t3062: F, t3114: F, t42522: F, t42600: F, t5857: F, t5869: F, t5875: F, t5880: F, t59676: F, t61655: F, t61659: F, t61663: F, t61665: F) -> F {
    let t61675 = t10904 * t17667;
    let t61677 = t17877 * t1040;
    let t61686 = -t10898 * t5869 / F::new(288.0) + F::new(5.0) / F::new(3456.0) * t13995 * t14235 + t61655 / F::new(2304.0) - F::new(19.0) / F::new(1728.0) * t42600 * t5880 + t61659 / F::new(1728.0) - t61663 / F::new(6912.0) + t61665 / F::new(2304.0) + F::new(5.0) / F::new(6912.0) * t1041 * t248 * t3062 * t59676 + t3114 * t17962 / F::new(1536.0) + F::new(19.0) / F::new(864.0) * t42522 * t5875 - t61675 / F::new(216.0) + t61677 * t1046 / F::new(2304.0) - t10863 * t5857 / F::new(432.0) - t3048 * t17890 / F::new(432.0) + F::new(19.0) / F::new(1728.0) * t10517 * t5869;
    t61686
}
