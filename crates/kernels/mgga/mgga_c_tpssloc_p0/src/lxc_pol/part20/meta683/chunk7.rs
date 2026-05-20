//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2591/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2591<F: Float>(t225: F, t52377: F, t11638: F, t11720: F, t11888: F, t11910: F, t11914: F, t11915: F, t1244: F, t1246: F, t1247: F, t14988: F, t15245: F, t15247: F, t1751: F, t1755: F, t23508: F, t3610: F, t3624: F, t3626: F, t44785: F, t475: F, t491: F, t494: F, t5068: F, t5072: F, t5079: F, t52424: F, t52435: F, t52447: F, t52458: F) -> (F, F) {
    let t52462 = t52377 * t225;
    let t52471 = -t11720 * t1755 * t23508 * t44785 * t475 + t11638 * t1244 * t1246 * t1751 + t1244 * t1246 * t491 * t52458 - F::new(18.0) * t11888 * t15247 * t5072 + t11914 * t11915 * t52424 + F::new(12.0) * t14988 * t3610 * t5068 - F::new(6.0) * t14988 * t3624 * t5079 - F::new(3.0) * t11910 * t15245 + F::new(3.0) * t1247 * t52447 - F::new(3.0) * t3626 * t52435 + t494 * t52462;
    (t52462, t52471)
}
