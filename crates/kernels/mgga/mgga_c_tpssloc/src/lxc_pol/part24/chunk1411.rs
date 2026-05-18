//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1411/1438 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1411<F: Float>(t3231: F, t868: F, t1877: F, t1915: F, t22959: F, t23290: F, t23781: F, t23792: F, t23796: F, t23807: F, t23810: F, t2522: F, t28: F, t4314: F, t6666: F, t6670: F, t81539: F, t82308: F, t82312: F, t83556: F, t83559: F, t83566: F, t83579: F, t83582: F, t83585: F, t83592: F, t83596: F) -> F {
    let t83603 = t3231 * t868;
    let t83607 = -F::new(9.0) * t22959 * t83556 - t1877 * t6670 * t83559 / F::new(2.0) + F::new(9.0) * t2522 * t6666 * t23792 + F::new(9.0) * t4314 * t1915 * t83566 + t1877 * t82308 * t28 / F::new(2.0) + F::new(9.0) * t4314 * t6666 * t23781 - F::new(3.0) * t1877 * t23290 * t23810 - F::new(9.0) / F::new(2.0) * t22959 * t83579 + F::new(9.0) * t22959 * t83582 - F::new(3.0) * t1877 * t82312 * t83585 + F::new(3.0) * t1877 * t81539 * t23807 + F::new(9.0) / F::new(2.0) * t2522 * t1915 * t83592 + F::new(9.0) / F::new(2.0) * t2522 * t1915 * t83596 + F::new(9.0) / F::new(2.0) * t2522 * t6666 * t23796 - F::new(3.0) / F::new(2.0) * t1877 * t6670 * t83603;
    t83607
}
