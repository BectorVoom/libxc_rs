//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 918/919 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk918<F: Float>(t120857: F, t122852: F, t122853: F, t122856: F, t122857: F, t122860: F, t122862: F, t122864: F, t127695: F, t128976: F, t128989: F, t1398: F, t1852: F, t1858: F, t2023: F, t2029: F, t2099: F, t2105: F, t28869: F, t28904: F, t29396: F, t29430: F, t3: F, t33628: F, t33662: F, t580: F, t6471: F, t6483: F, t7759: F, t7774: F, t7946: F, t7961: F, t8647: F, t8660: F) -> (F,) {
    let tv4rho2sigma213 = 2.0 * t122864 + t29396 * t2029 + 2.0 * t33628 * t1858 + t28869 * t2105 + t2023 * t29430 + t8647 * t6483 + 2.0 * t122853 + 2.0 * t7946 * t7774 + 2.0 * t122852 + t6471 * t8660 + 2.0 * t122860 + 2.0 * t7759 * t7961 + t1398 * (t127695 + t128989) + t2099 * t28904 + t3 * t128976 * t580 + 2.0 * t120857 + 2.0 * t1852 * t33662 + 2.0 * t122857 + 2.0 * t122862 + 2.0 * t122856;
    (tv4rho2sigma213,)
}
