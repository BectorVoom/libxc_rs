//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2456/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2456<F: Float>(t69533: F, t69574: F, t69665: F, t69695: F, t69741: F, t69791: F, t69817: F, t69837: F, t1049: F, t1052: F, t1065: F, t1625: F, t1635: F, t17583: F, t17588: F, t17875: F, t18071: F, t18166: F, t21480: F, t21662: F, t21663: F, t3026: F, t3174: F, t381: F, t388: F, t4557: F, t4660: F, t4665: F, t4694: F, t61058: F) -> (F, F) {
    let t69840 = t69533 + t69574 + t69665 + t69695 + t69741 + t69791 + t69817 + t69837;
    let t69860 = F::new(2.0) * t1052 * t1065 * t21662 * t3174 + t1049 * t21480 * t388 + F::new(3.0) * t1625 * t17875 * t388 + t381 * t388 * t69840 - F::new(6.0) * t1635 * t61058 + F::new(12.0) * t17583 * t4557 + F::new(12.0) * t17583 * t4660 + F::new(12.0) * t17588 * t4665 - F::new(6.0) * t17588 * t4694 - F::new(18.0) * t18071 * t4557 - F::new(18.0) * t18071 * t4660 - F::new(3.0) * t18166 * t4660 - t21663 * t3026;
    (t69840, t69860)
}
